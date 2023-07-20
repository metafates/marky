use std::fs;

use crate::info;
use anyhow::Result;
use colored::Colorize;
use handlebars::Handlebars;
use image::{DynamicImage, ImageOutputFormat};
use lol_html::{element, HtmlRewriter, Settings};
use serde::Serialize;
use std::ffi::OsStr;
use std::io::Cursor;
use std::path::Path;

use crate::included::{TEMPLATES_DIR, VENDOR_DIR};
use crate::themes::Theme;

pub struct Document {
    pub text: String,
    pub options: RenderOptions,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq)]
pub enum IncludeLevel {
    Local,
    Remote,
    All,
}

#[derive(Clone)]
pub struct RenderOptions {
    pub theme: Theme,
    pub highlight: bool,
    pub math: bool,
    pub diagrams: bool,
    pub live: bool,
    pub include_images: Option<IncludeLevel>,
    pub optimize_images: bool,
}

#[derive(Serialize)]
pub struct TemplateData {
    pub theme: String,
    pub highlight: bool,
    pub math: bool,
    pub diagrams: bool,
    pub compiled: String,
    pub title: String,
    pub websocket: String,
    pub script: String,
    pub live: bool,
}

impl Document {
    fn handlebars() -> Handlebars<'static> {
        let mut reg = Handlebars::new();
        let template_string = TEMPLATES_DIR
            .get_file("template.hbs")
            .expect("must be present")
            .contents_utf8()
            .expect("template must be a valid utf8");

        reg.register_template_string("html", template_string)
            .expect("must be a valid handlebars template");

        reg
    }

    pub fn render_body(&self) -> String {
        let markdown_options = markdown::Options {
            parse: markdown::ParseOptions {
                constructs: markdown::Constructs {
                    html_flow: true,
                    html_text: true,
                    math_flow: self.options.math,
                    math_text: self.options.math,
                    definition: true,
                    ..markdown::Constructs::gfm()
                },
                ..markdown::ParseOptions::gfm()
            },
            compile: markdown::CompileOptions {
                allow_dangerous_html: true,
                ..markdown::CompileOptions::gfm()
            },
        };

        let html = markdown::to_html_with_options(self.text.as_str(), &markdown_options)
            .expect("never errors with MDX disabled");

        if self.options.include_images.is_some() {
            self.include_images(html).unwrap() // TODO
        } else {
            html
        }
    }

    pub fn render(&self) -> Result<Vec<u8>> {
        let body = self.render_body();

        let script: String = {
            let mut minified_script = Vec::new();
            let script = VENDOR_DIR
                .get_file("js/script.js")
                .unwrap()
                .contents()
                .to_vec();

            match minify_js::minify(
                minify_js::TopLevelMode::Global,
                script.clone(),
                &mut minified_script,
            ) {
                Ok(()) => String::from_utf8_lossy(minified_script.as_slice()).to_string(),
                Err(_) => String::from_utf8_lossy(script.as_slice()).to_string(),
            }
        };

        let html = Self::handlebars().render(
            "html",
            &TemplateData {
                theme: self.options.theme.resolve()?,
                highlight: self.options.highlight,
                math: self.options.math,
                diagrams: self.options.diagrams,
                compiled: body,
                title: self.title().unwrap_or("Document".into()),
                live: self.options.live,
                websocket: VENDOR_DIR
                    .get_file("js/reconnecting-websocket.js")
                    .unwrap()
                    .contents_utf8()
                    .unwrap()
                    .to_string(),
                script,
            },
        )?;

        Ok(html.into_bytes())
    }

    pub fn title(&self) -> Option<String> {
        match markdown::to_mdast(&self.text, &markdown::ParseOptions::gfm()) {
            Ok(node) => Document::get_title_from_node(&node),
            Err(_) => None,
        }
    }

    fn get_title_from_node(node: &markdown::mdast::Node) -> Option<String> {
        match node {
            markdown::mdast::Node::Heading(_) => Some(node.to_string()),
            _ => match node.children() {
                Some(children) => {
                    for child in children.iter() {
                        if let Some(title) = Self::get_title_from_node(&child) {
                            return Some(title);
                        }

                        return None;
                    }

                    None
                }
                None => None,
            },
        }
    }

    fn include_images(&self, html_page: String) -> anyhow::Result<String> {
        let mut output = vec![];

        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![element!("img[src]", |el| {
                    let src = el.get_attribute("src").expect("src was required");

                    let data = {
                        let level = self.options.include_images.expect("must be not none");

                        let include_remote =
                            level == IncludeLevel::All || level == IncludeLevel::Remote;

                        let include_local =
                            level == IncludeLevel::All || level == IncludeLevel::Local;

                        let is_remote = src.starts_with("http");

                        if is_remote && include_remote {
                            info!("Downloading {}", src);

                            if src.ends_with(".svg") {
                                let svg_data = download_image(src.as_str())?;
                                let base64_svg = base64::encode(&svg_data);
                                el.set_attribute(
                                    "src",
                                    &format!("data:image/svg+xml;base64,{}", base64_svg),
                                )?;
                                None
                            } else {
                                Some(download_image(src.as_str())?)
                            }
                        } else if !is_remote && include_local {
                            info!("Reading {}", src);

                            let path = Path::new(&src);
                            let is_svg = path.extension() == Some(OsStr::new("svg"));

                            if is_svg {
                                let svg_data = self.svg_to_base64(path)?;
                                el.set_attribute("src", &svg_data)?;
                                None
                            } else {
                                Some(fs::read(src)?)
                            }
                        } else {
                            info!("Skipping {}", src);
                            None
                        }
                    };

                    if let Some(data) = data {
                        info!("Encoding to base64",);
                        let img = image::load_from_memory(data.as_slice())?;
                        el.set_attribute("src", &self.image_to_base64(&img)?)?;
                    }

                    Ok(())
                })],
                ..Settings::default()
            },
            |c: &[u8]| output.extend_from_slice(c),
        );

        rewriter.write(html_page.as_bytes())?;
        rewriter.end()?;

        Ok(String::from_utf8(output)?)
    }

    fn image_to_base64(&self, img: &DynamicImage) -> anyhow::Result<String> {
        let mut image_data: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
            .unwrap();

        if self.options.optimize_images {
            info!("Optimizing image");
            match oxipng::optimize_from_memory(image_data.as_slice(), &oxipng::Options::default()) {
                Ok(optimized) => image_data = optimized,
                Err(e) => return Err(anyhow::Error::new(e)),
            }
        }

        let res_base64 = base64::encode(image_data);
        Ok(format!("data:image/png;base64,{}", res_base64))
    }

    fn svg_to_base64(&self, path: &Path) -> anyhow::Result<String> {
        let svg_data = fs::read(path)?;
        let base64_svg = base64::encode(svg_data);
        let svg_data_uri = format!("data:image/svg+xml;base64,{}", base64_svg);
        Ok(svg_data_uri)
    }
}

fn download_image(url: &str) -> anyhow::Result<Vec<u8>> {
    Ok(reqwest::blocking::get(url)?.bytes()?.into())
}

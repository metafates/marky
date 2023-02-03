use anyhow::Result;
use handlebars::Handlebars;
use serde::Serialize;

use crate::included::{TEMPLATES_DIR, VENDOR_DIR};
use crate::pdf;
use crate::themes::Theme;

pub struct Document {
    pub text: String,
    pub options: RenderOptions,
}

#[derive(Clone)]
pub struct RenderOptions {
    pub theme: Theme,
    pub highlight: bool,
    pub math: bool,
    pub diagrams: bool,
    pub pdf: bool,
    pub live: bool,
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

        markdown::to_html_with_options(self.text.as_str(), &markdown_options)
            .expect("never errors with MDX disabled")
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

        let bytes: Vec<u8> = {
            if self.options.pdf {
                pdf::html_to_pdf(html.as_str())?
            } else {
                html.into_bytes()
            }
        };

        Ok(bytes)
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
}

use crate::included::VENDOR_DIR;
use crate::pdf;
use crate::themes::Theme;

pub struct Document {
    text: String,
}

pub struct RenderOptions {
    pub theme: Theme,
    pub highlight: bool,
    pub math: bool,
    pub diagrams: bool,
    pub pdf: bool,
}

impl Document {
    pub fn render(&self, options: &RenderOptions) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let markdown_options = markdown::Options {
            parse: markdown::ParseOptions {
                constructs: markdown::Constructs {
                    html_flow: true,
                    html_text: true,
                    math_flow: options.math,
                    math_text: options.math,
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

        let body = markdown::to_html_with_options(self.text.as_str(), &markdown_options)?;

        let highlighter: String = format!(
            r#"<style>{style}</style>
<script>{script}</script>
<script>hljs.highlightAll();</script>"#,
            style = VENDOR_DIR
                .get_file("highlight/highlight.min.css")
                .unwrap()
                .contents_utf8()
                .unwrap(),
            script = VENDOR_DIR
                .get_file("highlight/highlight.min.js")
                .unwrap()
                .contents_utf8()
                .unwrap(),
        );

        const MATH: &str = r#"<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.4/dist/katex.min.css" integrity="sha384-vKruj+a13U8yHIkAyGgK1J3ArTLzrFGBbBc0tDp4ad/EyewESeXE/Iv67Aj8gKZ0" crossorigin="anonymous">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.4/dist/katex.min.js" integrity="sha384-PwRUT/YqbnEjkZO0zZxNqcxACrXe+j766U2amXcgMg5457rve2Y7I6ZJSm2A0mS4" crossorigin="anonymous"></script>
<script>document.addEventListener("DOMContentLoaded",()=>{{for(let e of document.querySelectorAll(".language-math"))katex.render(e.textContent,e)}});</script>"#;

        const DIAGRAMS: &str = r#"<script src="https://cdn.jsdelivr.net/npm/mermaid@9.3.0/dist/mermaid.min.js"></script>
<script>mermaid.initialize({startOnLoad:!1}),document.addEventListener("DOMContentLoaded",()=>{const e=document.querySelectorAll("code.language-mermaid");let n=0;for(const t of e){const e=`mermaid${n}`;n++;const o=(e,n)=>{t.innerHTML=e},d=t.textContent;mermaid.mermaidAPI.render(e,d,o)}});</script>"#;

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">

{highlight}
{math}
{diagrams}

<title>{title}</title>

<style>{style}</style>
</head>

<body>
<main class="container">
{body}
</main>
</body>
</html>"#,
            highlight = if options.highlight {
                highlighter
            } else {
                String::new()
            },
            math = if options.math { MATH } else { "" },
            diagrams = if options.diagrams { DIAGRAMS } else { "" },
            title = self.title().unwrap_or("Document".into()),
            style = options.theme.resolve()?,
            body = body,
        );

        let bytes: Vec<u8> = {
            if options.pdf {
                pdf::html_to_pdf(html.as_str(), None)?
            } else {
                html.into_bytes()
            }
        };

        Ok(bytes)
    }

    pub fn new(text: &str) -> Self {
        Document { text: text.into() }
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

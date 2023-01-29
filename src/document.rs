use crate::included::STATIC_DIR;
use crate::themes::Theme;

pub struct Document {
    text: String,
}

pub struct RenderOptions {
    pub highlight: bool,
    pub theme: Theme,
}

impl Document {
    pub fn render(&self, options: &RenderOptions) -> Result<String, Box<dyn std::error::Error>> {
        let highlighter: String = format!(
            r#"
    <style>{style}</style>
    <script>{script}</script>
    <script>hljs.highlightAll();</script>
        "#,
            style = STATIC_DIR
                .get_file("vendor/highlight.min.css")
                .unwrap()
                .contents_utf8()
                .unwrap(),
            script = STATIC_DIR
                .get_file("vendor/highlight.min.js")
                .unwrap()
                .contents_utf8()
                .unwrap(),
        );

        let html = format!(
            r#"
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">

{highlight}

<title>{title}</title>

<style>
{style}
</style>
</head>

<body>
{body}
</body>
</html>
"#,
            highlight = if options.highlight {
                highlighter
            } else {
                String::new()
            },
            title = self.title().unwrap_or("Document".into()),
            style = options.theme.resolve()?,
            body = markdown::to_html(self.text.as_str()),
        );

        Ok(html)
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

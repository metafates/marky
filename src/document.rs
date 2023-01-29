use crate::themes::Theme;

pub struct Document {
    text: String,
}

impl Document {
    pub fn to_html(
        &self,
        theme: &Theme,
        highlight_code: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        const HIGHLIGHTER: &str = r#"
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/default.min.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js"></script>
    <script>hljs.highlightAll();</script>
        "#;

        let html = format!(
            r#"
<!doctype html>

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
            highlight = if highlight_code { HIGHLIGHTER } else { "" },
            title = self.title().unwrap_or("Document".into()),
            style = theme.resolve()?,
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

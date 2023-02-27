use crate::info;
use anyhow::Result;
use colored::Colorize;
use headless_chrome::types::PrintToPdfOptions;
use std::io::{self, Write};

pub fn html_to_pdf(html: &str) -> Result<Vec<u8>> {
    let options = headless_chrome::LaunchOptionsBuilder::default()
        .build()
        .expect("Default should not panic");

    let mut builder = tempfile::Builder::new();
    builder.suffix(".html");

    let mut file = builder.tempfile()?;
    file.write(html.as_bytes())?;

    let path = file
        .path()
        .as_os_str()
        .to_str()
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?
        .to_string();

    let uri = format!("file://{}", path);

    info!("Opening browser");
    let browser = headless_chrome::Browser::new(options)?;

    info!("Waiting for initial tab");
    let tab = browser.new_tab()?;

    info!("Opening temporary html file");
    let tab = tab.navigate_to(&uri)?.wait_until_navigated()?;

    let options = PrintToPdfOptions {
        display_header_footer: Some(false),
        print_background: None,
        scale: None,
        paper_width: None,
        paper_height: None,
        margin_top: None,
        margin_bottom: None,
        margin_left: None,
        margin_right: None,
        page_ranges: None,
        ignore_invalid_page_ranges: None,
        header_template: None,
        footer_template: None,
        prefer_css_page_size: None,
        transfer_mode: None,
        landscape: None,
    };

    info!("Converting to PDF");
    let bytes = tab.print_to_pdf(Some(options))?;

    info!("Closing headless browser");
    tab.close_with_unload()?;
    Ok(bytes)
}

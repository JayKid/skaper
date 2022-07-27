// HTML Scraper crate
use scraper::{Html, Selector};

pub struct HtmlHelper {
    html: String,
}

impl HtmlHelper {
    // Consider re-implementing with: https://docs.rs/thirtyfour/latest/thirtyfour/extensions/query/index.html
    pub fn parse_html(&mut self, html_contents: String) {
        self.html = html_contents;
    }

    pub fn select(&self, selector: String) {
        let document = Html::parse_document(&self.html);
        let selector = Selector::parse(&selector).unwrap();

        let mut results: Vec<String> = Vec::new();

        for element in document.select(&selector) {
            println!("Item name TBD:\r");
            println!("{:?}\r",element.inner_html());
            results.push(element.inner_html());
        }
        println!("Found {} items\r", results.len());
    }
}

pub fn get_instance() -> HtmlHelper {
    return HtmlHelper { html: String::from("") }
}
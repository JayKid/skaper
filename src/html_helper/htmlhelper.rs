// HTML Scraper crate
use scraper::{Html, Selector};

pub struct Result {
    pub name: String,
    pub price: String,
}

pub struct HtmlHelper {
    html: String,
}

impl HtmlHelper {
    // Consider re-implementing with: https://docs.rs/thirtyfour/latest/thirtyfour/extensions/query/index.html
    pub fn parse_html(&mut self, html_contents: String) {
        self.html = html_contents;
    }

    fn is_valid_result(&self, haystack: String, search_term: String) -> bool {
        if search_term.is_empty() {
            return true;
        }
        let mut compressed_haystack = String::from(haystack);
        compressed_haystack.retain(|c| c != ' ');
        compressed_haystack = compressed_haystack.to_lowercase();
        let mut compressed_search_term = String::from(search_term);
        compressed_search_term.retain(|c| c != ' ');
        compressed_search_term = compressed_search_term.to_lowercase();
        return compressed_haystack.contains(&compressed_search_term);
    }

    pub fn select(
        &self,
        search_term: String,
        item_selector: String,
        name_subselector: String,
        price_subselector: String,
    ) -> Vec<Result> {
        let document = Html::parse_document(&self.html); // This would be inefficient if selecting several times per document
        let selector = Selector::parse(&item_selector).unwrap();

        let mut results: Vec<Result> = Vec::new();

        for element in document.select(&selector) {
            let mut result = Result {
                name: String::from(""),
                price: String::from(""),
            };
            let name_selector = Selector::parse(&name_subselector).unwrap();
            let mut skip_element = false;

            element.select(&name_selector).for_each(|subelement| {
                if !self.is_valid_result(subelement.inner_html(), search_term.clone()) {
                    skip_element = true;
                    return;
                }
                result.name = subelement.inner_html();
            });

            if !skip_element {
                let price_selector = Selector::parse(&price_subselector).unwrap();

                element.select(&price_selector).for_each(|subelement| {
                    result.price = subelement.inner_html();
                });

                results.push(result);
            }
        }
        println!("Found {} items\r", results.len());
        return results;
    }
}

pub fn get_instance() -> HtmlHelper {
    return HtmlHelper {
        html: String::from(""),
    };
}

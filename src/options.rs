use super::html_helper::htmlhelper::{get_instance, Result};
use super::network;

pub struct Choice {
    pub name: String,
    pub action: Box<dyn Fn() -> Vec<Result>>,
}

fn action_3090_ti() -> Vec<Result> {
    let website_contents = network::fetch(String::from("https://www.orderflow.ch/de/search?limit=100&sort=price%7Casc&q=3090+ti&cat=%5BPC-Grafikkarten%5D"));

    let mut html_helper = get_instance();
    html_helper.parse_html(website_contents);

    // Ideally we would return results here that we can process/display however we want
    let results = html_helper.select(
        "3090 TI".to_string(),
        ".item.media".to_string(),
        ".media-heading>a".to_string(),
        ".first_price>.price".to_string(),
    );

    return results;
    // Display that currently happens within select should happen below
}

fn action_3090() -> Vec<Result> {
    let website_contents = network::fetch(String::from("https://www.orderflow.ch/de/search?limit=100&sort=price%7Casc&q=3090&cat=%5BPC-Grafikkarten%5D"));

    let mut html_helper = get_instance();
    html_helper.parse_html(website_contents);

    // Ideally we would return results here that we can process/display however we want
    let results = html_helper.select(
        "3090".to_string(),
        ".item.media".to_string(),
        ".media-heading>a".to_string(),
        ".first_price>.price".to_string(),
    );

    return results;

    // Display that currently happens within select should happen below
}
fn action_3080_ti() -> Vec<Result> {
    let website_contents = network::fetch(String::from("https://www.orderflow.ch/de/search?limit=100&sort=price%7Casc&q=3080+ti&cat=%5BPC-Grafikkarten%5D"));

    let mut html_helper = get_instance();
    html_helper.parse_html(website_contents);

    // Ideally we would return results here that we can process/display however we want
    let results = html_helper.select(
        "3080 TI".to_string(),
        ".item.media".to_string(),
        ".media-heading>a".to_string(),
        ".first_price>.price".to_string(),
    );

    return results;

    // Display that currently happens within select should happen below
}
fn preispirat_elektronik() -> Vec<Result> {
    let website_contents = network::fetch(String::from("https://www.preispirat.ch/elektronik/"));

    let mut html_helper = get_instance();
    html_helper.parse_html(website_contents);

    // Ideally we would return results here that we can process/display however we want
    let results = html_helper.select(
        "".to_string(),
        ".news-community".to_string(),
        ".newstitle>a".to_string(),
        ".rh_regular_price".to_string(),
    );

    return results;
    // Display that currently happens within select should happen below
}

pub fn get_menu_options() -> Vec<Choice> {
    return vec![
        Choice {
            name: String::from("Preispirat Elektronik"),
            action: Box::new(preispirat_elektronik),
        },
        Choice {
            name: String::from("3090 TI"),
            action: Box::new(action_3090_ti),
        },
        Choice {
            name: String::from("3090"),
            action: Box::new(action_3090),
        },
        Choice {
            name: String::from("3080 TI"),
            action: Box::new(action_3080_ti),
        },
    ];
}

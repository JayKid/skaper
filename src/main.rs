extern crate termion;

mod html_helper;
mod network;
mod ui;
use ui::{Ui, Choice};

use std::io::{stdout};
use termion::raw::{IntoRawMode};

pub fn get_results() {
    vec!["a", "b", "c"];
}

fn main() {
    let stdout = stdout().into_raw_mode().unwrap();

    let mut ui_service = Ui {
        ..Default::default()
    };
    let options = vec![
        Choice {
            name: String::from("3090 TI"),
            action: Box::new(action_3090_ti),
        },
        Choice {
            name: String::from("3090"),
            action: Box::new(action_3090_ti),
        },
        Choice {
            name: String::from("3080 TI"),
            action: Box::new(action_3090_ti),
        },
    ];
    ui_service.init(options, stdout);
    ui_service.refresh();
}

fn action_3090_ti() {
    let website_contents = network::fetch(String::from("https://www.orderflow.ch/de/search?limit=100&sort=price%7Casc&q=3090+ti&cat=%5BPC-Grafikkarten%5D"));

    let mut html_helper = html_helper::htmlhelper::get_instance();
    html_helper.parse_html(website_contents);

    // Ideally we would return results here that we can process/display however we want
    let selector = html_helper.select(".item.media .first_price>.price".to_string());
    // Display that currently happens within select should happen below
}

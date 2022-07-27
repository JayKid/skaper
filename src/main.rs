extern crate termion;

mod html_helper;
mod network; // bc used in options.rs
mod options;
mod ui;

use ui::Ui;
use options::get_menu_options;

use std::io::stdout;
use termion::raw::IntoRawMode;

fn main() {
    let stdout = stdout().into_raw_mode().unwrap();

    let mut ui_service = Ui {
        ..Default::default()
    };
    let options = get_menu_options();
    ui_service.init(options, stdout);
    ui_service.refresh();
}

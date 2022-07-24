// use std::io::{stdin, stdout};
use colored::*;

struct Ui {
    // current_item_selected: usize,
    caret: ColoredString,
}

impl Ui {
    fn print_item(&self, item: &str) {
        println!("{}", item);
    }

    fn print_selected_item(&self, item: &str) {
        println!("{}{}", self.caret, String::from(item).cyan());
    }

    fn render_options(&self, options: Vec<&str>) {
        let mut index: usize = 0;
        while index < options.len() {
            if index == 1 {
                self.print_selected_item(options[index])
            } else {
                self.print_item(options[index])
            }
            index = index + 1;
        }
    }
}

impl Default for Ui {
    fn default() -> Ui {
        Ui {
            caret: String::from("> ").magenta(),
        }
    }
}

fn main() {
    // let stdin = stdin();
    // let mut stdout = stdout();
    // let current_item_selected: usize = 0;

    // for character in stdin.keys() {
    //     match character.unwrap() {
    //         Key::Char('q') => break,
    //         _ => {}
    //     }
    // }
    let ui_service = Ui {
        ..Default::default()
    };
    let options = vec!["1st option", "2nd option", "3rd option"];

    ui_service.render_options(options);
}

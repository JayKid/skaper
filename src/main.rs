extern crate termion;

use colored::*;
use std::cmp::{max, min};
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

struct Choice {
    name: String,
    action: Box<dyn Fn() -> ()>,
}

struct Ui {
    caret: ColoredString,
    options: Vec<Choice>,
    current_index_selected: usize,
    stdout: RawTerminal<Stdout>,
}

impl Ui {
    fn init(&mut self, options: Vec<Choice>, stdout: RawTerminal<Stdout>) {
        self.options = options;
        self.stdout = stdout;
    }

    fn refresh(&mut self) {
        let mut index: usize = 0;
        self.move_caret_back();
        while index < self.options.len() {
            if index == self.current_index_selected {
                self.print_selected_item(String::from(&self.options[index].name));
            } else {
                self.print_item(String::from(&self.options[index].name));
            }
            index = index + 1;
        }
    }

    fn navigate_up(&mut self) {
        if self.current_index_selected != 0 {
            self.current_index_selected = max(0, self.current_index_selected - 1);
        }
        self.refresh();
    }

    fn navigate_down(&mut self) {
        self.current_index_selected = min(self.current_index_selected + 1, self.options.len() - 1);
        self.refresh();
    }

    fn print_item(&mut self, item: String) {
        write!(self.stdout, "{}\r\n", item).unwrap();
        self.stdout.flush().unwrap();
    }

    fn print_selected_item(&mut self, item: String) {
        write!(self.stdout, "{}{}\r\n", String::from(item).cyan(), self.caret).unwrap();
        self.stdout.flush().unwrap();
    }

    fn move_caret_back(&mut self) {
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
    }

    fn clear_current_line(&mut self) {
        write!(self.stdout, "{}", termion::clear::CurrentLine).unwrap();
    }

    fn select_option(&mut self) {
        let results = (self.options[self.current_index_selected].action)();
        write!(self.stdout, "{:?}", results).unwrap();
    }

    fn listen_for_keys(&mut self) {
        let stdin = stdin();

        for c in stdin.keys() {
            self.clear_current_line();
            match c.unwrap() {
                Key::Char('q') => break,
                Key::Esc => break,
                Key::Up => self.navigate_up(),
                Key::Down => self.navigate_down(),
                Key::Right => self.select_option(),
                _ => {}
            }
        }
    }
}

impl Default for Ui {
    fn default() -> Ui {
        Ui {
            caret: String::from("<-").magenta(),
            options: Vec::from([]),
            current_index_selected: 0,
            stdout: stdout().into_raw_mode().unwrap(),
        }
    }
}

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
            action: Box::new(get_results),
        },
        Choice {
            name: String::from("3090"),
            action: Box::new(get_results),
        },
        Choice {
            name: String::from("3080 TI"),
            action: Box::new(get_results),
        },
    ];
    ui_service.init(options, stdout);
    ui_service.refresh();
    ui_service.listen_for_keys();
}

// Terminal I/O crates
use colored::*;
use std::cmp::{max, min};
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Choice {
    pub name: String,
    pub action: Box<dyn Fn() -> ()>,
}

pub struct Ui {
    pub caret: ColoredString,
    pub options: Vec<Choice>,
    pub current_index_selected: usize,
    pub stdout: RawTerminal<Stdout>,
}

impl Ui {
    pub fn init(&mut self, options: Vec<Choice>, stdout: RawTerminal<Stdout>) {
        self.options = options;
        self.stdout = stdout;
        self.listen_for_keys();
    }

    pub fn refresh(&mut self) {
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
        write!(
            self.stdout,
            "{}{}\r\n",
            String::from(item).cyan(),
            self.caret
        )
        .unwrap();
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
            caret: String::from(" <-").magenta(),
            options: Vec::from([]),
            current_index_selected: 0,
            stdout: stdout().into_raw_mode().unwrap(),
        }
    }
}
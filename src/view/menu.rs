use super::Display;
use super::Window;

use std::vec::Vec;

pub struct Menu {
    window: Window,
    fields: Vec<String>,
}

impl Menu {
    pub fn new(window: Window, fields: Vec<String>) -> Menu {
        Menu {
            window: window,
            fields: fields,
        }
    }
}

impl Display<Menu> for Menu {
    fn start_line(mut self) -> Menu {
        let mut line = "┏".to_string();
        for _x in 0..(self.window.cols - 2) {
            line = line + "━";
        }
        line = line + "┓";
        self.fields.push(line);
        self
    }

    fn disp_line(mut self) -> Menu {
        let mut line = "┃".to_string();
        for _x in 0..(self.window.cols - 2) {
            line = line + " ";
        }
        line = line + "┃";
        self.fields.push(line);
        self
    }

    fn sepalate_line(mut self) -> Menu {
        let mut line = "┣".to_string();
        for _x in 0..(self.window.cols - 2) {
            line = line + "━";
        }
        line = line + "┫";
        self.fields.push(line);
        self
    }

    fn end_line(mut self) -> Menu {
        let mut line = "┗".to_string();
        for _x in 0..(self.window.cols - 2) {
            line = line + "━";
        }
        line = line + "┛";
        self.fields.push(line);
        self
    }

    fn disp(mut self) {
        for line in self.fields {
            println!("{}", line);
        }
    }
}

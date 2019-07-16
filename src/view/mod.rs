pub trait Display<T> {
    fn start_line(mut self) -> T;
    fn disp_line(mut self) -> T;
    fn sepalate_line(mut self) -> T;
    fn end_line(mut self) -> T;
    fn disp(mut self);
}

pub struct Menu {
    display_width: i32,
    display_height: i32,
    fields: Vec<String>,
}

impl Menu {
    pub fn new(display_width: i32, display_height: i32, fields: Vec<String>) -> Menu {
        Menu {
            display_width: display_width,
            display_height: display_height,
            fields: fields,
        }
    }
}

impl Display<Menu> for Menu {
    fn start_line(mut self) -> Menu {
        let mut line = "┏".to_string();
        for _x in 0..(self.display_width - 2) {
            line = line + "━";
        }
        line = line + "┓";
        self.fields.push(line);
        self
    }

    fn disp_line(mut self) -> Menu {
        let mut line = "┃".to_string();
        for _x in 0..(self.display_width - 2) {
            line = line + " ";
        }
        line = line + "┃";
        self.fields.push(line);
        self
    }

    fn sepalate_line(mut self) -> Menu {
        let mut line = "┣".to_string();
        for _x in 0..(self.display_width - 2) {
            line = line + "━";
        }
        line = line + "┫";
        self.fields.push(line);
        self
    }

    fn end_line(mut self) -> Menu {
        let mut line = "┗".to_string();
        for _x in 0..(self.display_width - 2) {
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

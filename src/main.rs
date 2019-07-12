trait Display {
    fn disp();
}
struct Menu;

impl Display for Menu {
    fn disp() {
        let WIDTH = 60;
        let HEIGHT = 30;
        for y in 0..HEIGHT {
            let mut line = if y == 0 {
                "┏".to_string()
            } else if y == (HEIGHT - 1) {
                "┗".to_string()
            } else {
                "┃".to_string()
            };
            for x in 0..WIDTH {
                line = if y == 0 || y == (HEIGHT - 1) {
                    line + "━"
                } else {
                    line + " "
                };
            }
            line = if y == 0 {
                line + "┓"
            } else if y == (HEIGHT - 1) {
                line + "┛"
            } else {
                line + "┃"
            };
            println!("{}", line);
        }
    }
}
fn main() {
    Menu::disp();
    let stdin = std::io::stdin();
}

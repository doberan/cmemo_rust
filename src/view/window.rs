//! Window information
use std::process::Command;

/// Window info
pub struct Window {
    /// window columns
    pub cols: i32,
    /// window lines
    pub lines: i32,
    /// cursor position for x coordinate
    pub cursor_x: u16,
    /// cursor position for y coordinate
    pub cursor_y: u16,
}

/// Window behavior
impl Window {
    /// generate window instance
    pub fn new() -> Window {
        let (x, y) = get_window_size();
        Window {
            cols: x,
            lines: y,
            cursor_x: 1u16,
            cursor_y: 1u16,
        }
    } 
}

/// Get application window size
fn get_window_size() -> (i32, i32) {
    let output_width = Command::new("tput")
        .arg("cols")
        .output()
        .expect("failed to execute tput");

    let output_height = Command::new("tput")
        .arg("lines")
        .output()
        .expect("failed to execute tput");
    let mut width_str = std::str::from_utf8(&(output_width.stdout))
        .unwrap()
        .to_string();
    let mut height_str = std::str::from_utf8(&(output_height.stdout))
        .unwrap()
        .to_string();
    width_str.retain(|c| c != '\n');
    height_str.retain(|c| c != '\n');
    //    println!("{}, {}", width_str, height_str);
    (
        width_str.parse::<i32>().unwrap(),
        height_str.parse::<i32>().unwrap(),
    )
}
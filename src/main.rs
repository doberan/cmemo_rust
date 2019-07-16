//! # task management tool.
//!
//! ## what cmemo_rust
//! This tool is task management for work.
//!
//! ## how cmemo_rust
//! This tool is working speedly for task management.
//!
//! ## for example
//! ```
//! > Add copy and paste function
//! ┏━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
//! ┃No.┃            task              ┃
//! ┣━━━╋━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
//! ┃ 1 ┃ Add copy and paste function  ┃
//! ┗━━━┻━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
//!
//! > /add date -n 1 2019-07-18 09:00-10:00
//! ┏━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━┳━━━━━━━┳━━━━━━━┓
//! ┃No.┃            task              ┃  deadline  ┃ start ┃  end  ┃
//! ┣━━━╋━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━━━━╋━━━━━━━╋━━━━━━━┫
//! ┃ 1 ┃ Add copy and paste function  ┃ 2019-07-18 ┃ 09:00 ┃ 10:00 ┃
//! ┗━━━┻━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━┻━━━━━━━┻━━━━━━━┛
//! ```
//!
extern crate clap;
extern crate cmemo_rust;
extern crate termion;

pub use cmemo_rust::view::{Display, Menu};

use clap::{App, Arg};

use std::ffi::OsStr;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::{fs, path};

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{clear, cursor};
struct pos {
    window_x: u16,
    window_y: u16,
}
fn main() {
    let matches = App::new("cmemo_rust")
        .about("doberan memo tool.")
        .bin_name("cmemo_rust")
        .arg(Arg::with_name("file"))
        .get_matches();

    let (_w, _h) = get_window_size();
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut pos = pos {
        window_x: 1,
        window_y: 1,
    };
    write!(stdout, "{}", clear::All);
    write!(stdout, "{}", cursor::Goto(pos.window_x, pos.window_y));
    write!(stdout, "> ");
    // let mut s = String::new();
    // stdin.read_line(&mut s).ok();
    stdout.flush().unwrap();
    for evt in stdin.events() {
        let key = evt.unwrap();
        let _ = match key {
            Event::Key(Key::Ctrl('c')) => return,
            Event::Key(Key::Char('\n')) => {
                pos.window_x = 1;
                pos.window_y += 1;
                write!(stdout, "\n");
                write!(stdout, "{}", cursor::Goto(pos.window_x, pos.window_y));
            }
            Event::Key(Key::Char(n)) => {
                pos.window_x += 1;
                write!(stdout, "{}", n);
            }
            Event::Key(Key::Left) => {
                pos.window_x += 1;
                write!(stdout, "{}", cursor::Left(1 as u16));
            }
            Event::Key(Key::Right) => {
                pos.window_x -= 1;
                write!(stdout, "{}", cursor::Right(1 as u16));
            }
            Event::Key(_) => {}
            Event::Mouse(_) => {}
            Event::Unsupported(_) => return,
        };
        stdout.flush().unwrap();
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

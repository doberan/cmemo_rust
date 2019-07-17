//! main logic

extern crate clap;
extern crate cmemo_rust;
extern crate termion;

pub use cmemo_rust::view::{
    Display, 
    Menu, 
    Window
};

use clap::{App, Arg};

use std::ffi::OsStr;
pub use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::{fs, path};

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{clear, cursor};

fn main() {
    let matches = App::new("cmemo_rust")
        .about("doberan memo tool.")
        .bin_name("cmemo_rust")
        .arg(Arg::with_name("file"))
        .get_matches();
    let mut window = Window::new();
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(
        stdout().into_raw_mode().unwrap());
        // println!("{:?}", stdout);
    clear(&mut stdout);
    setup(&mut window, &mut stdout);
    invalidate(&mut stdout);
    let mut arg: Vec<u8> = vec![];
    for evt in stdin.events() {
        let key = evt.unwrap();
        let _ = match key {
            Event::Key(Key::Ctrl('c')) => return,
            Event::Key(Key::Char('\n')) => {
                window.cursor_x = 1u16;
                window.cursor_y = 1u16;
                clear(&mut stdout);
                invalidate(&mut stdout);
                write!(stdout, "{}", cursor::Goto(1, 2));
                write!(stdout, "{}", std::str::from_utf8(&arg).unwrap().to_string());
                setup(&mut window, &mut stdout);
                invalidate(&mut stdout);
            }
            Event::Key(Key::Char(n)) => {
                window.cursor_x += 1;
                arg.push(n as u8);
                write!(stdout, "{}", n);
            }
            Event::Key(Key::Left) => {
                window.cursor_x += 1;
                write!(stdout, "{}", cursor::Left(1 as u16));
            }
            Event::Key(Key::Right) => {
                window.cursor_x -= 1;
                write!(stdout, "{}", cursor::Right(1 as u16));
            }
            Event::Key(_) => {}
            Event::Mouse(_) => {}
            Event::Unsupported(_) => return,
        };
        stdout.flush().unwrap();
    }
}

fn setup(window: &mut Window, stdout: &mut AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>) {
    write!(*stdout, "{}", cursor::Goto(
        (*window).cursor_x, (*window).cursor_y));
    write!(*stdout, "> ");
}

fn clear(stdout: &mut AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>) {
    write!(*stdout, "{}", clear::All);
}

fn invalidate(stdout: &mut AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>) {
    (*stdout).flush().unwrap();
}
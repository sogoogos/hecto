use crate::Position;
use std::io::{self, stdout, Write};
// use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::style;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(position: &Position) {
        let Position{x, y} = position;
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    // Termion color does not work properly on Mac terminal
    pub fn set_bg_color(/*color: color::Rgb*/) {
        // print!{"{}", color::Bg(color)};
        print!{"{}", style::Invert};
    }

    pub fn reset_bg_color() {
        // print!("{}", color::Bg(color::Reset));
        print!{"{}", style::Reset};
    }

    pub fn set_fg_color(/*color: color::Rgb*/) {
        // print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color(){
        // print!("{}", color::Fg(color::Reset));
    }

}
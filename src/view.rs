use termion::raw::RawTerminal;

use std::io::{Write, Stdout};
use termion::{color, clear};
use failure::ResultExt;

pub fn flush_work_timer(stdout: &mut RawTerminal<Stdout>, timer: &str) -> Result<(), failure::Error> {
    write!(
        stdout,
        "{timer_cursor}{color}{clear}\u{1F345} {timer}{desc_cursor}[Q]: quit, [Space]: pause/resume",
        timer_cursor = termion::cursor::Goto(2, 1),
        color = color::Fg(color::Red),
        clear = clear::All,
        timer = timer,
        desc_cursor = termion::cursor::Goto(2, 2)
    ).context("failed to show work timer")?;
    stdout.flush().context("failed to flush work timer")?;
    Ok(())
}

pub fn flush_break_timer(stdout: &mut RawTerminal<Stdout>, timer: &str) -> Result<(), failure::Error> {
    write!(
        stdout,
        "{timer_cursor}{color}{clear}\u{2615} {timer}{desc_cursor}[Q]: quit, [Space]: pause/resume",
        timer_cursor = termion::cursor::Goto(2, 1),
        color = color::Fg(color::Green),
        clear = clear::All,
        timer = timer,
        desc_cursor = termion::cursor::Goto(2, 2)
    ).context("failed to show break timer")?;
    stdout.flush().context("failed to flush break timer")?;
    Ok(())
}

pub fn flush_break_interval(stdout: &mut RawTerminal<Stdout>) -> Result<(), failure::Error> {
    write!(
        stdout,
        "{}{}{}\u{1F389} press [Enter] to take a break",
        color::Fg(color::Green),
        clear::All,
        termion::cursor::Goto(2, 1)
    ).context("failed to show break interval")?;
    stdout.flush().context("failed to flush break interval")?;
    Ok(())
}

pub fn flush_work_interval(stdout: &mut RawTerminal<Stdout>) -> Result<(), failure::Error> {
    write!(
        stdout,
        "{}{}{}\u{1F514} press [Enter] to work!!",
        color::Fg(color::Red),
        termion::cursor::Goto(2, 1),
        clear::All,
    ).context("failed to show work interval")?;
    stdout.flush().context("failed to flush work interval")?;
    Ok(())
}

pub fn release_raw_mode(stdout: &mut RawTerminal<Stdout>) -> Result<(), failure::Error> {
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::cursor::Show
    ).context("failed to release raw mode")?;
    Ok(())
}
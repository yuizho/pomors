use termion::raw::RawTerminal;
use exitfailure::ExitFailure;

use std::io::{Write, Stdout};

pub fn release_raw_mode(stdout: &mut RawTerminal<Stdout>) -> Result<(), ExitFailure> {
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::cursor::Show
    )?;
    Ok(())
}
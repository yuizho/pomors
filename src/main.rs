extern crate termion;

use std::io::{stdout, Write, Stdout};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color};
use exitfailure::ExitFailure;

mod key_handler;

fn main() -> Result<(), ExitFailure> {
    // start key handler on another thread
    let receiver = key_handler::run();

    // start timer
    let mut stdout = stdout().into_raw_mode().unwrap();
    loop {
        // TODO: argsからdurationを受ける
        for duration in (0..10).rev() {
            if handle_input_on_timer(&receiver) {
                // rawモードを解除
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1, 1),
                    termion::cursor::Show
                )?;
                return Ok(());
            }
            write!(
                stdout,
                "{timer_cursor}{color}{clear}\u{1F345} {timer}{desc_cursor}[Q]: quit, [Space]: pause/resume",
                timer_cursor = termion::cursor::Goto(2, 1),
                color = color::Fg(color::Red),
                clear = clear::All,
                timer = convert_to_min(duration),
                desc_cursor = termion::cursor::Goto(2, 2)
            )?;
            stdout.flush()?;

            // https://crates.io/crates/spin_sleep
            spin_sleep::sleep(Duration::from_secs(1));
        }

        write!(
            stdout,
            "{}{}{}\u{1F389} press [Enter] to take a break",
            color::Fg(color::Green),
            clear::All,
            termion::cursor::Goto(2, 1)
        )?;
        stdout.flush()?;

        // handle key input on interval
        handle_input_on_interval(&mut stdout, &receiver)?;

        // TODO: argsからdurationを受ける
        for duration in (0..10).rev() {
            if handle_input_on_timer(&receiver) {
                // rawモードを解除
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1, 1),
                    termion::cursor::Show
                )?;
                return Ok(());
            }
            write!(
                stdout,
                "{timer_cursor}{color}{clear}\u{2615} {timer}{desc_cursor}[Q]: quit, [Space]: pause/resume",
                timer_cursor = termion::cursor::Goto(2, 1),
                color = color::Fg(color::Green),
                clear = clear::All,
                timer = convert_to_min(duration),
                desc_cursor = termion::cursor::Goto(2, 2)
            )?;
            stdout.flush()?;

            // https://crates.io/crates/spin_sleep
            spin_sleep::sleep(Duration::from_secs(1));
        }

        write!(
            stdout,
            "{}{}{}\u{1F514} press [Enter] to work!!",
            color::Fg(color::Red),
            termion::cursor::Goto(2, 1),
            clear::All,
        )?;
        stdout.flush()?;

        // handle key input on interval
        handle_input_on_interval(&mut stdout, &receiver)?;
    }
}

fn convert_to_min(duration: i32) -> String {
    let min = duration / 60;
    let sec = duration % 60;
    format!("{:02}:{:02}", min, sec)
}

// TODO: もどり値をResultに
fn handle_input_on_timer(receiver: &Receiver<&str>) -> bool {
    match receiver.try_recv() {
        Ok(message) => message == key_handler::TERMINATE,
        _ => false,
    }
}

fn handle_input_on_interval(stdout: &mut RawTerminal<Stdout>, receiver: &Receiver<&str>)
    -> Result<(), ExitFailure> {
    loop {
        match receiver.try_recv() {
            Ok(message) => match message {
                key_handler::ENTER => break,
                key_handler::TERMINATE => {
                    write!(
                        stdout,
                        "{}{}",
                        termion::cursor::Goto(1, 1),
                        termion::cursor::Show
                    )?;
                    break;
                }
                _ => (),
            },
            _ => (),
        }
    }
    Ok(())
}
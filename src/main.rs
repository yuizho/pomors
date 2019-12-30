extern crate termion;

use std::io::{stdout, Stdout};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use termion::raw::{IntoRawMode, RawTerminal};
use exitfailure::ExitFailure;

mod key_handler;
mod view;

fn main() -> Result<(), ExitFailure> {
    // start key handler on another thread
    let receiver = key_handler::run();

    // start timer
    let mut stdout = stdout().into_raw_mode().unwrap();
    loop {
        // TODO: argsからdurationを受ける
        for duration in (0..10).rev() {
            if handle_input_on_timer(&receiver) {
                view::release_raw_mode(&mut stdout)?;
                return Ok(());
            }
            view::flush_work_timer(&mut stdout, convert_to_min(duration).as_str())?;

            // https://crates.io/crates/spin_sleep
            spin_sleep::sleep(Duration::from_secs(1));
        }

        view::flush_break_interval(&mut stdout)?;

        // handle key input on interval
        if handle_input_on_interval(&mut stdout, &receiver)? {
            return Ok(());
        }

        // TODO: argsからdurationを受ける
        for duration in (0..10).rev() {
            if handle_input_on_timer(&receiver) {
                view::release_raw_mode(&mut stdout)?;
                return Ok(());
            }
            view::flush_break_timer(&mut stdout, convert_to_min(duration).as_str())?;

            // https://crates.io/crates/spin_sleep
            spin_sleep::sleep(Duration::from_secs(1));
        }

        view::flush_work_interval(&mut stdout)?;

        // handle key input on interval
        if handle_input_on_interval(&mut stdout, &receiver)? {
            return Ok(());
        }
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
    -> Result<bool, ExitFailure> {
    let mut terminated = false;
    loop {
        match receiver.try_recv() {
            Ok(message) => match message {
                key_handler::ENTER => {
                    break;
                },
                key_handler::TERMINATE => {
                    view::release_raw_mode(stdout)?;
                    terminated = true;
                    break;
                }
                _ => (),
            },
            _ => (),
        }
    }
    Ok(terminated)
}
extern crate termion;

use std::io::{stdout, Stdout};
use std::sync::mpsc::Receiver;
use std::time::Duration;

use exitfailure::ExitFailure;
use structopt::StructOpt;
use termion::raw::{IntoRawMode, RawTerminal};

mod key_handler;
mod view;

#[derive(StructOpt)]
struct Option {
    #[structopt(short = "w", long = "work-sec", default_value = "1500")]
    work_sec: u32,
    #[structopt(short = "b", long = "break-sec", default_value = "300")]
    break_sec: u32,
}

fn main() -> Result<(), ExitFailure> {
    // receive cli arguemnts
    let args = Option::from_args();

    // start key handler on another thread
    let receiver = key_handler::run();

    // start timer
    let mut stdout = stdout().into_raw_mode().unwrap();
    loop {
        // work timer
        for duration in (0..args.work_sec).rev() {
            if handle_input_on_timer(&receiver) {
                view::release_raw_mode(&mut stdout)?;
                return Ok(());
            }
            view::flush_work_timer(&mut stdout, convert_to_min(duration).as_str())?;
            spin_sleep::sleep(Duration::from_secs(1));
        }

        // break interval
        view::flush_break_interval(&mut stdout)?;
        if handle_input_on_interval(&mut stdout, &receiver)? {
            return Ok(());
        }

        // break timer
        for duration in (0..args.break_sec).rev() {
            if handle_input_on_timer(&receiver) {
                view::release_raw_mode(&mut stdout)?;
                return Ok(());
            }
            view::flush_break_timer(&mut stdout, convert_to_min(duration).as_str())?;
            spin_sleep::sleep(Duration::from_secs(1));
        }

        // work interval
        view::flush_work_interval(&mut stdout)?;
        if handle_input_on_interval(&mut stdout, &receiver)? {
            return Ok(());
        }
    }
}

fn convert_to_min(duration: u32) -> String {
    let min = duration / 60;
    let sec = duration % 60;
    format!("{:02}:{:02}", min, sec)
}

// TODO: もどり値をResultに
fn handle_input_on_timer(receiver: &Receiver<key_handler::KeyAction>) -> bool {
    match receiver.try_recv() {
        Ok(key_handler::KeyAction::Quit) => true,
        _ => false,
    }
}

fn handle_input_on_interval(stdout: &mut RawTerminal<Stdout>, receiver: &Receiver<key_handler::KeyAction>)
                            -> Result<bool, ExitFailure> {
    let mut terminated = false;
    loop {
        match receiver.try_recv() {
            Ok(message) => match message {
                key_handler::KeyAction::Ok => {
                    break;
                }
                key_handler::KeyAction::Quit => {
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
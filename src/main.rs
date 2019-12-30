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
        let mut paused_work_timer = false;
        let mut work_remaining_sec = args.work_sec;
        while work_remaining_sec != 0 {
            match handle_input_on_timer(&receiver) {
                key_handler::KeyAction::Quit => {
                    view::release_raw_mode(&mut stdout)?;
                    return Ok(());
                }
                key_handler::KeyAction::Pause => paused_work_timer = !paused_work_timer,
                _ => ()
            }
            if !paused_work_timer {
                view::flush_work_timer(&mut stdout, convert_to_min(work_remaining_sec).as_str())?;
                work_remaining_sec -= 1;
                spin_sleep::sleep(Duration::from_secs(1));
            }
        }

        // break interval
        view::flush_break_interval(&mut stdout)?;
        if handle_input_on_interval(&mut stdout, &receiver)? {
            return Ok(());
        }

        // break timer
        let mut paused_break_timer = false;
        let mut break_remaining_sec = args.break_sec;
        while break_remaining_sec != 0 {
            match handle_input_on_timer(&receiver) {
                key_handler::KeyAction::Quit => {
                    view::release_raw_mode(&mut stdout)?;
                    return Ok(());
                }
                key_handler::KeyAction::Pause => paused_break_timer = !paused_break_timer,
                _ => ()
            }
            if !paused_break_timer {
                view::flush_break_timer(&mut stdout, convert_to_min(break_remaining_sec).as_str())?;
                break_remaining_sec -= 1;
                spin_sleep::sleep(Duration::from_secs(1));
            }
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

fn handle_input_on_timer(receiver: &Receiver<key_handler::KeyAction>) -> key_handler::KeyAction {
    match receiver.try_recv() {
        Ok(key_handler::KeyAction::Quit) => key_handler::KeyAction::Quit,
        Ok(key_handler::KeyAction::Pause) => key_handler::KeyAction::Pause,
        _ => key_handler::KeyAction::None,
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
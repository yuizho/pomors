extern crate termion;

use std::io::{stdout, Stdout};
use std::sync::mpsc::Receiver;
use std::time::Duration;

use exitfailure::ExitFailure;
use structopt::StructOpt;
use termion::raw::{IntoRawMode, RawTerminal};

mod key_handler;
mod view;
mod notification;
mod sound;

#[derive(StructOpt)]
struct Option {
    #[structopt(short = "w", long = "work-sec", default_value = "1500")]
    work_sec: u16,
    #[structopt(short = "s", long = "short-break-sec", default_value = "300")]
    short_break_sec: u16,
    #[structopt(short = "l", long = "long-break-sec", default_value = "1200")]
    long_break_sec: u16,
}

fn main() -> Result<(), ExitFailure> {
    // receive cli arguemnts
    let args = Option::from_args();

    // start key handler on another thread
    let receiver = key_handler::run();

    // set up sound player
    let sound_player = sound::Player::new(sound::SoundFile::BELL);

    // start timer
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut round: u64 = 1;
    loop {
        // work timer
        if start_timer(args.work_sec, round, &receiver, &mut stdout, view::flush_work_timer)? {
            return Ok(());
        }

        notification::send("it's time to take a break \u{2615}")?;
        sound_player.play()?;

        // break interval
        view::flush_break_interval(&mut stdout)?;
        if handle_input_on_interval(&mut stdout, &receiver)? {
            return Ok(());
        }

        // break timer
        let break_sec = if round % 4 == 0 { args.long_break_sec } else { args.short_break_sec };
        if start_timer(break_sec, round, &receiver, &mut stdout, view::flush_break_timer)? {
            return Ok(());
        }

        notification::send("it's time to work again!! \u{1F4AA}")?;
        sound_player.play()?;

        // work interval
        view::flush_work_interval(&mut stdout)?;
        if handle_input_on_interval(&mut stdout, &receiver)? {
            return Ok(());
        }

        round += 1;
    }
}

fn start_timer(remaining_sec: u16,
               current_round: u64,
               receiver: &Receiver<key_handler::KeyAction>,
               stdout: &mut RawTerminal<Stdout>,
               flush_fn: fn(s: &mut RawTerminal<Stdout>, t: u16, c: u64) -> Result<(), failure::Error>)
               -> Result<bool, failure::Error> {
    let mut quited = false;
    let mut paused = false;
    let mut remaining_sec = remaining_sec;
    while remaining_sec != 0 {
        match handle_input_on_timer(receiver) {
            key_handler::KeyAction::Quit => {
                view::release_raw_mode(stdout)?;
                quited = true;
                break;
            }
            key_handler::KeyAction::Pause => paused = !paused,
            _ => ()
        }
        if !paused {
            flush_fn(stdout, remaining_sec, current_round)?;
            remaining_sec -= 1;
        }
        spin_sleep::sleep(Duration::from_secs(1));
    }
    Ok(quited)
}

fn handle_input_on_timer(receiver: &Receiver<key_handler::KeyAction>) -> key_handler::KeyAction {
    match receiver.try_recv() {
        Ok(key_handler::KeyAction::Quit) => key_handler::KeyAction::Quit,
        Ok(key_handler::KeyAction::Pause) => key_handler::KeyAction::Pause,
        _ => key_handler::KeyAction::None,
    }
}

fn handle_input_on_interval(stdout: &mut RawTerminal<Stdout>, receiver: &Receiver<key_handler::KeyAction>)
                            -> Result<bool, failure::Error> {
    let mut quited = false;
    loop {
        match receiver.try_recv() {
            Ok(message) => match message {
                key_handler::KeyAction::Ok => break,
                key_handler::KeyAction::Quit => {
                    view::release_raw_mode(stdout)?;
                    quited = true;
                    break;
                }
                _ => (),
            },
            _ => (),
        }
        spin_sleep::sleep(Duration::from_millis(100));
    }
    Ok(quited)
}
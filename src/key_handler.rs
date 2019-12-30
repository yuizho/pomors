use std::io::stdin;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

pub const ENTER: &str = "enter";
pub const SPACE: &str = "space";
pub const TERMINATE: &str = "terminate";

pub enum KeyAction {
    Quit,
    Pose,
    Ok,
}

pub fn run() -> Receiver<KeyAction> {
    let (sender, receiver) = mpsc::channel::<KeyAction>();
    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    sender.send(KeyAction::Ok).unwrap();
                }
                Key::Char(' ') => {
                    sender.send(KeyAction::Pose).unwrap();
                }
                Key::Char('q') => {
                    sender.send(KeyAction::Quit).unwrap();
                    break;
                }
                Key::Ctrl(c) => {
                    if c == 'c' {
                        sender.send(KeyAction::Quit).unwrap();
                        break;
                    }
                }
                _ => (),
            }
        }
    });
    receiver
}
use std::io::stdin;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

pub enum KeyAction {
    Quit,
    Pause,
    Ok,
    None,
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
                    sender.send(KeyAction::Pause).unwrap();
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
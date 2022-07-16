use std::io::Write;

use failure::ResultExt;
use termion::{clear, color};

pub fn flush_work_timer(stdout: &mut impl Write, remaining_sec: u16, current_round: u64)
                        -> Result<(), failure::Error> {
    write!(
        stdout,
        "{timer_cursor}{color}{clear}\u{1F345} {timer} (Round {current_round}){desc_cursor}[Q]: quit, [Space]: pause/resume, [S]: skip work",
        timer_cursor = termion::cursor::Goto(2, 1),
        color = color::Fg(color::Red),
        clear = clear::All,
        timer = convert_to_min(remaining_sec),
        current_round = current_round,
        desc_cursor = termion::cursor::Goto(2, 2)
    ).context("failed to show work timer")?;
    stdout.flush().context("failed to flush work timer")?;
    Ok(())
}

pub fn flush_break_timer(stdout: &mut impl Write, remaining_sec: u16, current_round: u64)
                         -> Result<(), failure::Error> {
    write!(
        stdout,
        "{timer_cursor}{color}{clear}\u{2615} {timer} (Round {current_round}){desc_cursor}[Q]: quit, [Space]: pause/resume, [S]: skip break",
        timer_cursor = termion::cursor::Goto(2, 1),
        color = color::Fg(color::Green),
        clear = clear::All,
        timer = convert_to_min(remaining_sec),
        current_round = current_round,
        desc_cursor = termion::cursor::Goto(2, 2)
    ).context("failed to show break timer")?;
    stdout.flush().context("failed to flush break timer")?;
    Ok(())
}

pub fn flush_break_interval(stdout: &mut impl Write) -> Result<(), failure::Error> {
    write!(
        stdout,
        "{msg_cursor}{color}{clear}\u{1F389} press Enter to take a break{desc_cursor}[Q]: quit, [Enter]: start",
        msg_cursor = termion::cursor::Goto(2, 1),
        color = color::Fg(color::Green),
        clear = clear::All,
        desc_cursor = termion::cursor::Goto(2, 2)
    ).context("failed to show break interval")?;
    stdout.flush().context("failed to flush break interval")?;
    Ok(())
}

pub fn flush_work_interval(stdout: &mut impl Write) -> Result<(), failure::Error> {
    write!(
        stdout,
        "{msg_cursor}{color}{clear}\u{1F514} press Enter to work!!{desc_cursor}[Q]: quit, [Enter]: start",
        msg_cursor = termion::cursor::Goto(2, 1),
        color = color::Fg(color::Red),
        clear = clear::All,
        desc_cursor = termion::cursor::Goto(2, 2)
    ).context("failed to show work interval")?;
    stdout.flush().context("failed to flush work interval")?;
    Ok(())
}

pub fn release_raw_mode(stdout: &mut impl Write) -> Result<(), failure::Error> {
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::cursor::Show
    ).context("failed to release raw mode")?;
    Ok(())
}

fn convert_to_min(duration: u16) -> String {
    let min = duration / 60;
    let sec = duration % 60;
    format!("{:02}:{:02}", min, sec)
}

#[cfg(test)]
mod tests {
    use crate::view::*;

    #[test]
    fn flush_work_timer_works_fine() {
        let mut buf = Vec::<u8>::new();
        let actual_resp = flush_work_timer(&mut buf, 4, 1);
        let actual_view = String::from_utf8(buf.to_vec()).unwrap();

        assert!(actual_resp.is_ok());
        assert!(actual_view.contains("00:04 (Round 1)"));
        assert!(actual_view.contains("[Q]: quit, [Space]: pause/resume, [S]: skip work"));
    }

    #[test]
    fn flush_break_timer_works_fine() {
        let mut buf = Vec::<u8>::new();
        let actual_resp = flush_break_timer(&mut buf, 604, 2);
        let actual_view = String::from_utf8(buf.to_vec()).unwrap();

        assert!(actual_resp.is_ok());
        assert!(actual_view.contains("10:04 (Round 2)"));
        assert!(actual_view.contains("[Q]: quit, [Space]: pause/resume, [S]: skip break"));
    }

    #[test]
    fn flush_break_interval_works_fine() {
        let mut buf = Vec::<u8>::new();
        let actual_resp = flush_break_interval(&mut buf);
        let actual_view = String::from_utf8(buf.to_vec()).unwrap();

        assert!(actual_resp.is_ok());
        assert!(actual_view.contains("press Enter to take a break"));
        assert!(actual_view.contains("[Q]: quit, [Enter]: start"));
    }

    #[test]
    fn flush_work_interval_works_fine() {
        let mut buf = Vec::<u8>::new();
        let actual_resp = flush_work_interval(&mut buf);
        let actual_view = String::from_utf8(buf.to_vec()).unwrap();

        assert!(actual_resp.is_ok());
        assert!(actual_view.contains("press Enter to work!!"));
        assert!(actual_view.contains("[Q]: quit, [Enter]: start"));
    }
}

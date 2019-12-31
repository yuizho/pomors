use failure::ResultExt;
use notify_rust::Notification;

pub fn send(message: &str) -> Result<(), failure::Error> {
    Notification::new()
        .summary("\u{1F345} pomors")
        .body(message)
        .show()
        .context("failed to show notification")?;
    Ok(())
}
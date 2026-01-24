//! Flash message handling for Inertia.js.
//!
//! Flash messages are temporary messages that persist across a single redirect.
//! They are stored in the session and cleared after being read.

use actix_session::Session;

use crate::dto::auth::FlashProps;

/// Session key for flash message
const FLASH_KEY: &str = "flash_message";

/// Extract flash message from session, clearing it in the process.
/// Returns `Some(FlashProps)` if a flash message exists, `None` otherwise.
pub fn extract_flash(session: &Session) -> Option<FlashProps> {
    session
        .get::<FlashProps>(FLASH_KEY)
        .ok()
        .flatten()
        .and_then(|flash| {
            // Clear the flash after reading it
            let _ = session.remove(FLASH_KEY);
            Some(flash)
        })
}

/// Set a flash message in the session.
/// Call this before redirecting to persist the message across the redirect.
pub fn set_flash(session: &Session, flash: FlashProps) {
    let _ = session.insert(FLASH_KEY, flash);
}

/// Clear any existing flash message in the session.
pub fn clear_flash(session: &Session) {
    let _ = session.remove(FLASH_KEY);
}

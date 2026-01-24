use actix_session::Session;

/// Session key for authenticated flag
const AUTH_SESSION_KEY: &str = "authenticated";

/// Session key for user ID
const USER_ID_SESSION_KEY: &str = "user_id";

/// Session key for access token (for validation)
const ACCESS_TOKEN_SESSION_KEY: &str = "access_token";

/// Check if the user is authenticated based on session state.
pub fn is_authenticated(session: &Session) -> bool {
    session
        .get::<bool>(AUTH_SESSION_KEY)
        .ok()
        .flatten()
        .unwrap_or(false)
}

/// Get the authenticated user's ID from the session.
pub fn get_user_id(session: &Session) -> Option<String> {
    session.get::<String>(USER_ID_SESSION_KEY).ok().flatten()
}

/// Get the access token from the session for validation.
pub fn get_access_token(session: &Session) -> Option<String> {
    session
        .get::<String>(ACCESS_TOKEN_SESSION_KEY)
        .ok()
        .flatten()
}

/// Set the authenticated state in the session.
/// Call this after successful login.
pub fn set_authenticated(session: &Session, user_id: &str, access_token: &str) {
    let _ = session.insert(AUTH_SESSION_KEY, true);
    let _ = session.insert(USER_ID_SESSION_KEY, user_id);
    let _ = session.insert(ACCESS_TOKEN_SESSION_KEY, access_token);
}

/// Clear the authenticated state from the session.
/// Call this on logout or when token validation fails.
pub fn clear_authenticated(session: &Session) {
    let _ = session.remove(AUTH_SESSION_KEY);
    let _ = session.remove(USER_ID_SESSION_KEY);
    let _ = session.remove(ACCESS_TOKEN_SESSION_KEY);
}

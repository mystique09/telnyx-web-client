use actix_session::Session;
use serde::{Deserialize, Serialize};

/// Session key for authenticated flag
const AUTH_SESSION_KEY: &str = "authenticated";

/// Session key for user ID
const USER_ID_SESSION_KEY: &str = "user_id";
/// Legacy session key for user ID kept for backward compatibility
const LEGACY_USER_ID_SESSION_KEY: &str = "id";

/// Session key for access token (for validation)
const ACCESS_TOKEN_SESSION_KEY: &str = "access_token";
/// Session key for consolidated auth payload
const AUTH_DATA_SESSION_KEY: &str = "auth_data";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUserData {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAuthData {
    pub user: SessionUserData,
    pub access_token: String,
}

fn get_auth_data(session: &Session) -> Option<SessionAuthData> {
    session
        .get::<SessionAuthData>(AUTH_DATA_SESSION_KEY)
        .ok()
        .flatten()
}

/// Check if the user is authenticated based on session state.
pub fn is_authenticated(session: &Session) -> bool {
    get_auth_data(session).is_some()
        || session
            .get::<bool>(AUTH_SESSION_KEY)
            .ok()
            .flatten()
            .unwrap_or(false)
}

/// Get the authenticated user's ID from the session.
pub fn get_user_id(session: &Session) -> Option<String> {
    if let Some(auth_data) = get_auth_data(session) {
        return Some(auth_data.user.id);
    }

    session
        .get::<String>(USER_ID_SESSION_KEY)
        .ok()
        .flatten()
        .or_else(|| {
            session
                .get::<String>(LEGACY_USER_ID_SESSION_KEY)
                .ok()
                .flatten()
        })
}

/// Get the access token from the session for validation.
pub fn get_access_token(session: &Session) -> Option<String> {
    if let Some(auth_data) = get_auth_data(session) {
        return Some(auth_data.access_token);
    }

    session
        .get::<String>(ACCESS_TOKEN_SESSION_KEY)
        .ok()
        .flatten()
}

/// Set the authenticated state in the session.
/// Call this after successful login.
pub fn set_authenticated(session: &Session, user_id: &str, email: &str, access_token: &str) {
    let auth_data = SessionAuthData {
        user: SessionUserData {
            id: user_id.to_string(),
            email: email.to_string(),
        },
        access_token: access_token.to_string(),
    };

    // New canonical session payload.
    let _ = session.insert(AUTH_DATA_SESSION_KEY, auth_data);
}

/// Clear the authenticated state from the session.
/// Call this on logout or when token validation fails.
pub fn clear_authenticated(session: &Session) {
    let _ = session.remove(AUTH_SESSION_KEY);
    let _ = session.remove(AUTH_DATA_SESSION_KEY);
    let _ = session.remove(USER_ID_SESSION_KEY);
    let _ = session.remove(LEGACY_USER_ID_SESSION_KEY);
    let _ = session.remove(ACCESS_TOKEN_SESSION_KEY);
}

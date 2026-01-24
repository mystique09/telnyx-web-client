use std::sync::Arc;

use actix_inertia::inertia_responder::InertiaResponder;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use garde::Report;
use serde::Serialize;

use crate::{Empty, dto::LoginRequest, inertia::response_with_html};
use application::usecases::login_usecase::LoginUsecase;

#[derive(Debug, Serialize)]
struct LoginPageProps {
    pub errors: Option<LoginErrorProps>,
    pub flash: Option<FlashProps>,
}

#[derive(Debug, Serialize)]
struct FlashProps {
    pub r#type: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
struct LoginErrorProps {
    pub email: Option<String>,
    pub password: Option<String>,
    pub general: Option<String>,
}

impl From<&Report> for LoginErrorProps {
    fn from(value: &Report) -> Self {
        let mut error_props = Self {
            email: None,
            password: None,
            general: None,
        };
        for (path, err) in value.iter() {
            let msg = err.message();
            let path_str = path.to_string();
            if path_str.contains("email") {
                error_props.email = Some(msg.to_string());
            } else if path_str.contains("password") {
                error_props.password = Some(msg.to_string());
            } else {
                error_props.general = Some(msg.to_string());
            }
        }
        error_props
    }
}

/// Render login page - GET /login
pub async fn render_login(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("Login", LoginPageProps { errors: None, flash: None }).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "Login".to_string())
    }
}

/// Process login form - POST /login
/// Inertia.js form flow:
/// - Success: Set auth cookies and redirect to / with flash message
/// - Failure: Return Login page with props.errors populated
pub async fn handle_login(
    req: HttpRequest,
    login_req: web::Json<LoginRequest>,
    login_usecase: web::Data<Arc<LoginUsecase>>,
) -> impl Responder {
    let cmd = login_req.into_inner().into();

    match login_usecase.execute(cmd).await {
        Ok(result) => {
            // Success: Set auth cookies and redirect to home
            let mut response = HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/"))
                .append_header((
                    actix_web::http::header::SET_COOKIE,
                    format!(
                        "access_token={}; HttpOnly; Secure; Path=/; SameSite=Lax; Max-Age={}",
                        result.access_token,
                        3600
                    ),
                ))
                .append_header((
                    actix_web::http::header::SET_COOKIE,
                    format!(
                        "refresh_token={}; HttpOnly; Secure; Path=/; SameSite=Lax; Max-Age={}",
                        result.refresh_token,
                        604800
                    ),
                ))
                .finish();
            response
        }
        Err(ref e) => {
            // Failure: Return Login page with errors
            let errors = match e {
                application::usecases::UsecaseError::Validation(report) => {
                    LoginErrorProps::from(report)
                }
                _ => LoginErrorProps {
                    email: None,
                    password: None,
                    general: Some(e.to_http_message()),
                },
            };

            InertiaResponder::new(
                "Login",
                LoginPageProps {
                    errors: Some(errors),
                    flash: None,
                },
            )
            .respond_to(&req)
        }
    }
}

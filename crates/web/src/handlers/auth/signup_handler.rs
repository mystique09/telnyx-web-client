use std::sync::Arc;

use actix_inertia::inertia_responder::InertiaResponder;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use crate::{Empty, dto::SignupRequest, inertia::response_with_html};
use application::usecases::create_user_usecase::CreateUserUsecase;

#[derive(Debug, Serialize)]
struct SignupPageProps {
    pub errors: Option<SignupErrorProps>,
    pub flash: Option<FlashProps>,
}

#[derive(Debug, Serialize)]
struct FlashProps {
    pub r#type: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
struct SignupErrorProps {
    pub email: Option<String>,
    pub password: Option<String>,
    pub general: Option<String>,
}

pub async fn render_signup(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("Signup", SignupPageProps { errors: None, flash: None }).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "Signup".to_string())
    }
}

/// Process signup form - POST /signup
/// Inertia.js form flow:
/// - Success: Return HTTP 303 redirect to /login?registered=true
/// - Failure: Return Signup page with props.errors populated
pub async fn handle_signup(
    req: HttpRequest,
    signup_req: web::Json<SignupRequest>,
    create_user: web::Data<Arc<CreateUserUsecase>>,
) -> impl Responder {
    let cmd = signup_req.into_inner().into();

    match create_user.execute(cmd).await {
        Ok(_result) => {
            // Success: Redirect to login page with registered flag
            HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/login?registered=true"))
                .finish()
        }
        Err(e) => {
            // Failure: Return Signup page with errors
            let errors = match e {
                application::usecases::UsecaseError::Validation(report) => {
                    let mut error_props = SignupErrorProps {
                        email: None,
                        password: None,
                        general: None,
                    };
                    for (path, err) in report.iter() {
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
                application::usecases::UsecaseError::EmailAlreadyTaken => SignupErrorProps {
                    email: Some("An account with this email already exists".to_string()),
                    password: None,
                    general: None,
                },
                _ => SignupErrorProps {
                    email: None,
                    password: None,
                    general: Some("An error occurred. Please try again.".to_string()),
                },
            };

            InertiaResponder::new(
                "Signup",
                SignupPageProps {
                    errors: Some(errors),
                    flash: None,
                },
            )
            .respond_to(&req)
        }
    }
}

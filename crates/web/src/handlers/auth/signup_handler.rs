use std::sync::Arc;

use actix_inertia::inertia_responder::InertiaResponder;
use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Serialize;

use crate::flash::{extract_flash, set_flash};
use crate::{
    dto::auth::{FlashProps, SignupErrorProps, SignupRequest},
    inertia::Page,
};
use application::usecases::create_user_usecase::CreateUserUsecase;

#[derive(Debug, Serialize)]
struct SignupPageProps {
    pub errors: Option<SignupErrorProps>,
    pub flash: Option<FlashProps>,
}

pub async fn render_signup(req: HttpRequest, session: Session) -> impl Responder {
    let flash = extract_flash(&session);

    Page::builder()
        .req(req)
        .name("Signup")
        .props(SignupPageProps {
            errors: None,
            flash,
        })
        .build()
        .to_responder()
}

/// Process signup form - POST /auth/signup
/// Inertia.js form flow:
/// - Success: Set flash message and redirect to /auth/login
/// - Failure: Return Signup page with props.errors populated
pub async fn handle_signup(
    req: HttpRequest,
    signup_req: web::Json<SignupRequest>,
    create_user: web::Data<Arc<CreateUserUsecase>>,
    session: Session,
) -> impl Responder {
    let cmd = signup_req.into_inner().into();

    match create_user.execute(cmd).await {
        Ok(_result) => {
            // Set flash message and redirect to login
            set_flash(
                &session,
                FlashProps::success("Your account has been created successfully. Please log in."),
            );
            HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/auth/login"))
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

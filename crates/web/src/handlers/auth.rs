use std::sync::Arc;

use actix_inertia::inertia_responder::InertiaResponder;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Serialize;

use crate::types::WebError;
use crate::{Empty, dto::SignupRequest, inertia::response_with_html};
use application::usecases::create_user_usecase::CreateUserUsecase;
use application::usecases::login_usecase::LoginUsecase;

/// Login page props
#[derive(Debug, Serialize)]
struct LoginPageProps {
    pub errors: Option<LoginErrorProps>,
}

#[derive(Debug, Serialize)]
struct LoginErrorProps {
    pub email: Option<String>,
    pub password: Option<String>,
    pub general: Option<String>,
}

/// Render login page - GET /login
pub async fn render_login(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("Login", LoginPageProps { errors: None }).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "Login".to_string())
    }
}

/// Process login form - POST /login
/// Inertia.js form flow:
/// - Success: return HTTP 303 redirect to /dashboard
/// - Failure: return Login page with props.errors populated
pub async fn handle_login(
    req: HttpRequest,
    login_req: web::Json<LoginRequest>,
    login_usecase: web::Data<Arc<LoginUsecase>>,
) -> impl Responder {
    let cmd = login_req.into_inner().into();

    match login_usecase.execute(cmd).await {
        Ok(_result) => {
            // Success: Set auth cookie and redirect to dashboard
            HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/dashboard"))
                .finish()
        }
        Err(ref e) => {
            // Failure: Return Login page with errors
            let errors = match e {
                application::usecases::UsecaseError::Validation(report) => {
                    let mut error_props = LoginErrorProps {
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
                },
            )
            .respond_to(&req)
        }
    }
}

/// Render signup page - GET /signup
pub async fn render_signup(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("Signup", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "Signup".to_string())
    }
}

/// Process signup form - POST /signup
pub async fn handle_signup(
    signup_req: web::Json<SignupRequest>,
    create_user: web::Data<Arc<CreateUserUsecase>>,
) -> Result<web::Json<SignupSuccessResponse>, WebError> {
    let cmd = signup_req.into_inner().into();
    let result = create_user.execute(cmd).await?;

    Ok(web::Json(SignupSuccessResponse {
        id: result.id.to_string(),
        email: result.email,
    }))
}

/// Render forgot password page - GET /forgot-password
pub async fn render_forgot_password(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("ForgotPassword", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "ForgotPassword".to_string())
    }
}

/// Render reset password page - GET /reset-password
pub async fn render_reset_password(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("ResetPassword", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "ResetPassword".to_string())
    }
}

/// Signup success response
#[derive(Debug, Serialize)]
pub struct SignupSuccessResponse {
    pub id: String,
    pub email: String,
}

/// Login request DTO
#[derive(Debug, serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl From<LoginRequest> for application::commands::LoginCommand {
    fn from(req: LoginRequest) -> Self {
        Self {
            email: req.email,
            password: req.password,
        }
    }
}

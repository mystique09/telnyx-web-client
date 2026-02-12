use std::sync::Arc;

use actix_inertia::inertia_responder::InertiaResponder;
use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use garde::Report;
use serde::Serialize;

use crate::flash::extract_flash;
use crate::session::set_authenticated;
use crate::{
    dto::auth::{FlashProps, LoginErrorProps, LoginRequest},
    flash::set_flash,
    inertia::Page,
};
use application::usecases::login_usecase::LoginUsecase;
use domain::repositories::user_repository::UserRepository;
use domain::traits::password_hasher::PasswordHasher;
use domain::traits::token_service::TokenService;

#[derive(Debug, Serialize)]
struct LoginPageProps {
    pub errors: Option<LoginErrorProps>,
    pub flash: Option<FlashProps>,
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
pub async fn render_login(req: HttpRequest, session: Session) -> impl Responder {
    let flash = extract_flash(&session);

    Page::builder()
        .req(req)
        .name("Login")
        .props(LoginPageProps {
            errors: None,
            flash,
        })
        .build()
        .to_responder()
}

/// Process login form - POST /login
/// Inertia.js form flow:
/// - Success: Set auth cookies and redirect to / with flash message
/// - Failure: Return Login page with props.errors populated
pub async fn handle_login(
    req: HttpRequest,
    login_req: web::Json<LoginRequest>,
    user_repository: web::Data<Arc<dyn UserRepository>>,
    password_hasher: web::Data<Arc<dyn PasswordHasher>>,
    token_service: web::Data<Arc<dyn TokenService>>,
    session: Session,
) -> impl Responder {
    let cmd = login_req.into_inner().into();
    let login_usecase = LoginUsecase::builder()
        .user_repository(user_repository.get_ref().clone())
        .password_hasher(password_hasher.get_ref().clone())
        .token_service(token_service.get_ref().clone())
        .build();

    match login_usecase.execute(cmd).await {
        Ok(result) => {
            set_flash(
                &session,
                FlashProps::success("Welcome back! You have successfully logged in."),
            );

            set_authenticated(
                &session,
                &result.id.to_string(),
                &result.email,
                &result.access_token,
            );

            HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/"))
                .finish()
        }
        Err(ref e) => {
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

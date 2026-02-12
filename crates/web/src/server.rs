use std::sync::Arc;

use actix_files::Files;
use actix_inertia::{VersionMiddleware, inertia_responder::InertiaResponder};
use actix_session::{Session, SessionMiddleware};
use actix_web::{
    App, Error, HttpRequest, Responder, Result,
    body::MessageBody,
    cookie::Key,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    http::StatusCode,
    middleware::{Compress, ErrorHandlerResponse, ErrorHandlers, Logger, NormalizePath},
    web,
};
use serde::Serialize;
use tracing::error;

use crate::{
    Empty,
    dto::{DashboardAnalyticsProps, FlashProps},
    flash::{clear_flash, extract_flash},
    handlers::{
        auth::build_auth_service, conversations::build_conversations_service, inertia::version,
        phone_numbers::build_phone_numbers_service,
    },
    inertia::{Page, dist_dir, is_dev, response_with_html},
    middlewares::auth::ProtectedMiddleware,
    session::get_user_id,
};
use application::usecases::get_dashboard_home_usecase::GetDashboardHomeUsecase;
use domain::repositories::conversation_repository::ConversationRepository;
use domain::repositories::message_repository::MessageRepository;
use domain::repositories::phone_number_repository::PhoneNumberRepository;
use domain::repositories::user_repository::UserRepository;
use domain::traits::password_hasher::PasswordHasher;
use domain::traits::token_service::TokenService;

pub fn create_web_service(
    session_secret: String,
    user_repository: Arc<dyn UserRepository>,
    conversation_repository: Arc<dyn ConversationRepository>,
    message_repository: Arc<dyn MessageRepository>,
    phone_number_repository: Arc<dyn PhoneNumberRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    token_service: Arc<dyn TokenService>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let dist = dist_dir();

    let signing_key = Key::from(session_secret.as_bytes());

    let mut app = App::new()
        .wrap(
            SessionMiddleware::builder(
                actix_session::storage::CookieSessionStore::default(),
                signing_key,
            )
            .cookie_http_only(true)
            .cookie_secure(!is_dev())
            .build(),
        )
        .wrap(NormalizePath::trim())
        .wrap(Compress::default())
        .wrap(Logger::default())
        .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, error_404_error_handler))
        .app_data(web::Data::new(user_repository))
        .app_data(web::Data::new(password_hasher))
        .app_data(web::Data::new(conversation_repository))
        .app_data(web::Data::new(message_repository))
        .app_data(web::Data::new(phone_number_repository))
        .app_data(web::Data::new(token_service.clone()))
        .route("/", web::get().to(index).wrap(ProtectedMiddleware::new()))
        .service(build_conversations_service())
        .service(build_phone_numbers_service())
        .service(build_auth_service())
        .service(
            web::scope("/version")
                .wrap(VersionMiddleware::new("1".to_string()))
                .route("", web::get().to(version)),
        );

    // Production: serve built assets from dist/
    if !is_dev() {
        app = app
            .service(Files::new("/assets", dist.join("assets")).prefer_utf8(true))
            .service(Files::new("/", dist).prefer_utf8(true));
    }
    app
}

#[derive(Debug, Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
struct HomePageProps {
    pub flash: Option<FlashProps>,
    pub phone_numbers: Vec<crate::dto::PhoneNumberProps>,
    pub analytics: DashboardAnalyticsProps,
}

async fn index(
    req: HttpRequest,
    session: Session,
    conversation_repository: web::Data<Arc<dyn ConversationRepository>>,
    message_repository: web::Data<Arc<dyn MessageRepository>>,
    phone_number_repository: web::Data<Arc<dyn PhoneNumberRepository>>,
) -> impl Responder {
    let flash = extract_flash(&session);

    if flash.is_some() {
        clear_flash(&session);
    }

    let get_dashboard_home_usecase = GetDashboardHomeUsecase::builder()
        .conversation_repository(conversation_repository.get_ref().clone())
        .message_repository(message_repository.get_ref().clone())
        .phone_number_repository(phone_number_repository.get_ref().clone())
        .build();

    let (phone_numbers, analytics) = match session_user_id(&session) {
        Some(user_id) => match get_dashboard_home_usecase.execute(user_id).await {
            Ok(result) => {
                let phone_numbers = result
                    .phone_numbers
                    .iter()
                    .map(crate::dto::PhoneNumberProps::from)
                    .collect::<Vec<_>>();

                let analytics = DashboardAnalyticsProps::builder()
                    .total_conversations(result.analytics.total_conversations)
                    .total_messages(result.analytics.total_messages)
                    .total_phone_numbers(result.analytics.total_phone_numbers)
                    .build();

                (phone_numbers, analytics)
            }
            Err(err) => {
                error!(
                    "failed to load dashboard home data for user {}: {}",
                    user_id, err
                );

                (
                    Vec::new(),
                    DashboardAnalyticsProps::builder()
                        .total_conversations(0)
                        .total_messages(0)
                        .total_phone_numbers(0)
                        .build(),
                )
            }
        },
        None => (
            Vec::new(),
            DashboardAnalyticsProps::builder()
                .total_conversations(0)
                .total_messages(0)
                .total_phone_numbers(0)
                .build(),
        ),
    };

    Page::builder()
        .req(req)
        .name("App")
        .props(
            HomePageProps::builder()
                .maybe_flash(flash)
                .phone_numbers(phone_numbers)
                .analytics(analytics)
                .build(),
        )
        .build()
        .to_responder()
}

fn session_user_id(session: &Session) -> Option<uuid::Uuid> {
    get_user_id(session).and_then(|id| uuid::Uuid::parse_str(&id).ok())
}

fn error_404_error_handler<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let (req, _) = res.into_parts();

    let response = if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("NotFound", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "NotFound".to_string())
    };

    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(req, response).map_into_right_body(),
    ))
}

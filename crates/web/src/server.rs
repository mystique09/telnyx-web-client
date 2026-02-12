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
use application::usecases::create_user_usecase::CreateUserUsecase;
use application::usecases::login_usecase::LoginUsecase;
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

    // Create use case instances (in production, this would come from DI container)
    let create_user_usecase = Arc::new(
        CreateUserUsecase::builder()
            .user_repository(user_repository.clone())
            .password_hasher(password_hasher.clone())
            .build(),
    );

    let login_usecase = Arc::new(
        LoginUsecase::builder()
            .user_repository(user_repository)
            .password_hasher(password_hasher)
            .token_service(token_service.clone())
            .build(),
    );

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
        .app_data(web::Data::new(create_user_usecase))
        .app_data(web::Data::new(login_usecase))
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

    let (phone_numbers, analytics) = match session_user_id(&session) {
        Some(user_id) => {
            let (conversations_result, total_messages_result, phone_numbers_result) =
                futures_util::future::join3(
                    conversation_repository.list_by_user_id(&user_id),
                    message_repository.count_by_user_id(&user_id),
                    phone_number_repository.list_by_user_id(&user_id),
                )
                .await;

            let conversations = match conversations_result {
                Ok(items) => items,
                Err(err) => {
                    error!("failed to list conversations for user {}: {}", user_id, err);
                    Vec::new()
                }
            };

            let total_messages = match total_messages_result {
                Ok(total) => total,
                Err(err) => {
                    error!("failed to count messages for user {}: {}", user_id, err);
                    0
                }
            };

            let phone_numbers = match phone_numbers_result {
                Ok(items) => items
                    .iter()
                    .map(crate::dto::PhoneNumberProps::from)
                    .collect::<Vec<_>>(),
                Err(err) => {
                    error!("failed to list phone numbers for user {}: {}", user_id, err);
                    Vec::new()
                }
            };

            let analytics = DashboardAnalyticsProps::builder()
                .total_conversations(conversations.len() as u64)
                .total_messages(total_messages)
                .total_phone_numbers(phone_numbers.len() as u64)
                .build();

            (phone_numbers, analytics)
        }
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

use std::sync::Arc;

use actix_inertia::inertia_responder::InertiaResponder;
use actix_web::{HttpRequest, Responder, web};

use crate::{
    Empty,
    dto::{SignupRequest, auth::SignupSuccessResponse},
    inertia::response_with_html,
    types::WebError,
};
use application::usecases::create_user_usecase::CreateUserUsecase;

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

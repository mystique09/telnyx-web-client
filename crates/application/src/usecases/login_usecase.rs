use std::sync::Arc;
use std::time::Duration;

use garde::Validate;

use crate::commands::LoginCommand;
use crate::responses::LoginResult;
use crate::usecases::UsecaseError;
use domain::{
    repositories::user_repository::UserRepository,
    traits::password_hasher::PasswordHasher,
    traits::token_service::{PasetoClaimPurpose, PasetoClaims, TokenService},
};

#[derive(bon::Builder)]
pub struct LoginUsecase {
    user_repository: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    token_service: Arc<dyn TokenService>,
}

impl LoginUsecase {
    pub async fn execute(&self, cmd: LoginCommand) -> Result<LoginResult, UsecaseError> {
        cmd.validate()?;

        let user = match self.user_repository.find_by_email(&cmd.email).await {
            Ok(user) => user,
            Err(_) => return Err(UsecaseError::InvalidCredentials),
        };

        let password_valid = self.password_hasher.verify(&cmd.password, &user.hash).await;

        if password_valid.is_err() {
            return Err(UsecaseError::InvalidCredentials);
        }

        // Generate access token
        let access_token_exp = Duration::from_secs(3600);
        let access_token_claims = PasetoClaims::new(
            user.id,
            user.email.clone(),
            "user".to_string(),
            access_token_exp,
            PasetoClaimPurpose::AccessToken,
        );
        let (access_token, access_expires_at) = self
            .token_service
            .generate_token(access_token_claims, access_token_exp)
            .map_err(|_| UsecaseError::TokenGenerationFailed)?;

        // Generate refresh token
        let refresh_token_exp = Duration::from_secs(604800);
        let refresh_token_claims = PasetoClaims::new(
            user.id,
            user.email.clone(),
            "user".to_string(),
            refresh_token_exp,
            PasetoClaimPurpose::RefreshToken,
        );
        let (refresh_token, _) = self
            .token_service
            .generate_token(refresh_token_claims, refresh_token_exp)
            .map_err(|_| UsecaseError::TokenGenerationFailed)?;

        Ok(LoginResult {
            id: user.id,
            email: user.email,
            email_verified: user.email_verified,
            access_token,
            refresh_token,
            expires_at: access_expires_at,
        })
    }
}

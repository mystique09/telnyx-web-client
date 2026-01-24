use std::time::Duration;

use domain::traits::token_service::{
    PasetoClaimPurpose, PasetoClaims, TokenService, TokenServiceError,
};
use pasetors::{
    Local,
    claims::{Claims, ClaimsValidationRules},
    keys::SymmetricKey,
    token::UntrustedToken,
    version4::V4,
};
use time::{UtcDateTime, format_description::well_known::Rfc3339};

pub struct PasetoAuthenticationTokenService {
    symmetric_key: SymmetricKey<V4>,
}

impl PasetoAuthenticationTokenService {
    pub fn new(symmetric_key: &str) -> Result<Self, TokenServiceError> {
        let symmetric_key = SymmetricKey::<V4>::from(&symmetric_key.as_bytes())
            .map_err(|_| TokenServiceError::TokenGenerationFailed)?;

        Ok(Self { symmetric_key })
    }
}

impl TokenService for PasetoAuthenticationTokenService {
    fn generate_token(
        &self,
        claims: PasetoClaims,
        expiration: Duration,
    ) -> Result<(String, UtcDateTime), TokenServiceError> {
        let additional_data =
            serde_json::to_value(&claims).map_err(|_| TokenServiceError::JsonSerializationError)?;
        let now = time::UtcDateTime::now();
        let expiration = now + expiration;

        let mut paseto_claims =
            Claims::new().map_err(|_| TokenServiceError::TokenGenerationFailed)?;
        paseto_claims
            .add_additional("data", additional_data)
            .map_err(|_| TokenServiceError::TokenGenerationFailed)?;
        paseto_claims
            .expiration(expiration.format(&Rfc3339).unwrap().as_str())
            .map_err(|_| TokenServiceError::TokenGenerationFailed)?;
        paseto_claims
            .not_before(now.format(&Rfc3339).unwrap().as_str())
            .map_err(|_| TokenServiceError::TokenGenerationFailed)?;
        paseto_claims
            .issued_at(now.format(&Rfc3339).unwrap().as_str())
            .map_err(|_| TokenServiceError::TokenGenerationFailed)?;
        paseto_claims
            .issuer("reforged")
            .map_err(|_| TokenServiceError::TokenGenerationFailed)?;
        paseto_claims
            .audience("reforged")
            .map_err(|_| TokenServiceError::TokenGenerationFailed)?;

        let token = pasetors::local::encrypt(&self.symmetric_key, &paseto_claims, None, None)
            .map_err(|_| TokenServiceError::TokenGenerationFailed)?;

        Ok((token, expiration))
    }

    fn validate_token(
        &self,
        token: String,
        purpose: PasetoClaimPurpose,
    ) -> Result<PasetoClaims, TokenServiceError> {
        let mut validation_rules = ClaimsValidationRules::new();
        validation_rules.validate_issuer_with("reforged");
        validation_rules.validate_audience_with("reforged");

        let untrustued_token = UntrustedToken::<Local, V4>::try_from(&token)
            .map_err(|_| TokenServiceError::TokenInvalid)?;

        let trusted_token = pasetors::local::decrypt(
            &self.symmetric_key,
            &untrustued_token,
            &validation_rules,
            None,
            None,
        )
        .map_err(|_| TokenServiceError::TokenInvalid)?;

        let claims = trusted_token.payload_claims();

        match claims {
            Some(claims) => {
                let additional_data = claims.get_claim("data");
                if additional_data.is_none() {
                    return Err(TokenServiceError::TokenExpired);
                }

                let additional_data = additional_data.unwrap();

                let paste_claims: PasetoClaims = serde_json::from_value(additional_data.clone())
                    .map_err(|e| {
                        eprintln!("Error deserializing user claims: {}", e);
                        TokenServiceError::JsonDeserializationError
                    })?;

                if paste_claims.purpose != purpose {
                    return Err(TokenServiceError::TokenInvalid);
                }

                let expiration = paste_claims.exp;

                let issued_at = claims
                    .get_claim("iat")
                    .ok_or(TokenServiceError::TokenInvalid)?
                    .as_str()
                    .ok_or(TokenServiceError::TokenInvalid)?;

                let issued_at = time::UtcDateTime::parse(issued_at, &Rfc3339)
                    .map_err(|_| TokenServiceError::TokenInvalid)?;

                if time::UtcDateTime::now() > issued_at + expiration {
                    return Err(TokenServiceError::TokenExpired);
                }

                Ok(paste_claims)
            }
            None => Err(TokenServiceError::TokenInvalid),
        }
    }
}

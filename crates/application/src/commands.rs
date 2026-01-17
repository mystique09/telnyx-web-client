use garde::Validate;

#[derive(Debug, Validate, Clone)]
pub struct SignupCommand {
    #[garde(email)]
    pub email: String,

    #[garde(length(min = 8))]
    pub password: String,

    #[garde(skip)]
    pub password_confirmation: String,
}

impl SignupCommand {
    pub fn validate_passwords_match(&self) -> Result<(), garde::Error> {
        if self.password != self.password_confirmation {
            Err(garde::Error::new("Passwords do not match"))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Validate, Clone)]
pub struct LoginCommand {
    #[garde(email)]
    pub email: String,

    #[garde(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Validate, Clone)]
pub struct ForgotPasswordCommand {
    #[garde(email)]
    pub email: String,
}

#[derive(Debug, Validate, Clone)]
pub struct ResetPasswordCommand {
    #[garde(length(min = 32))]
    pub token: String,

    #[garde(length(min = 8))]
    pub password: String,

    #[garde(skip)]
    pub password_confirmation: String,
}

impl ResetPasswordCommand {
    pub fn validate_passwords_match(&self) -> Result<(), garde::Error> {
        if self.password != self.password_confirmation {
            Err(garde::Error::new("Passwords do not match"))
        } else {
            Ok(())
        }
    }
}

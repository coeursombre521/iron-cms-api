use crate::services::error::SecurityError;

pub trait PasswordHashService: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<String, SecurityError>;
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, SecurityError>;
}
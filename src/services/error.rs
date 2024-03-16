use serde::Serialize;

use crate::domain::error::CommonError;
use crate::error_codes::{
    ERR_CODE_SECURITY,
    ERR_CODE_ENV_VARIABLE
};

#[derive(Debug, Serialize)]
pub struct SecurityError {
    pub message: String,
    pub context: String,
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SecurityError: {}, Context: {}", self.message, self.context)
    }
}

impl std::error::Error for SecurityError { }

impl From<SecurityError> for CommonError {
    fn from(val: SecurityError) -> Self {
        CommonError {
            message: std::format!("{}", val),
            code: ERR_CODE_SECURITY,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct EnvVariableError {
    pub message: String,
    pub variable_name: String,
}

impl std::fmt::Display for EnvVariableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EnvVariableError: {}, Variable: {}", self.message, self.variable_name)
    }
}

impl std::error::Error for EnvVariableError { }

impl From<EnvVariableError> for CommonError {
    fn from(val: EnvVariableError) -> Self {
        CommonError {
            message: std::format!("{}", val),
            code: ERR_CODE_ENV_VARIABLE,
        }
    }
}
use serde::{Serialize, Deserialize};

/// Simple and effective response body for returning a message. Sometimes all
/// you need is a friendly message to convey info back to the client.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

/// Response body for returning a message along with a single serializable
/// object of any type. It's versatile and can handle a variety of scenarios.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageWithObjectResponse<T> {
    pub message: String,
    pub object_name: String,
    pub object: T,
}

/// Response body for returning a message along with a list of serializable
/// objects of any type. Doesn't handle pagination and it is not recommended for
/// large datasets, as it can be inefficient.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageWithListOfObjectsResponse<T> {
    pub message: String,
    pub objects: Vec<T>,
}

/// Struct for representing pagination parameters in responses. Essential for
/// endpoints that return paginated results. Helps the client navigate through
/// large datasets efficiently.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginationParams {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub has_next: bool,
}

/// Response body for returning a message along with a list of objects and
/// pagination parameters. Perfect for endpoints returning a list of objects
/// along with metadata like pagination details. It provides a comprehensive
/// response for collection endpoints.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageWithPaginationResponse<T> {
    pub message: String,
    pub objects: Vec<T>,
    pub pagination: PaginationParams,
}

/// Struct representing an error item with contextual information. This allows
/// clients to understand what went wrong and why.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorItem {
    pub context: String,
    pub message: String,
    pub error_code: Option<u32>,
}

/// Enum representing different types of error responses.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ErrorResponseType {
    General = 0,
    Validation = 1,
    NotFound = 2,
    Conflict = 3,
    Forbidden = 4,
    UnprocessableEntity = 5,
    InternalServerError = 6,
}

/// Implementation of conversion from integer to ErrorResponseType enum
impl From<i32> for ErrorResponseType {
    fn from(error_type: i32) -> Self {
        match error_type {
            0 => ErrorResponseType::General,
            1 => ErrorResponseType::Validation,
            2 => ErrorResponseType::NotFound,
            3 => ErrorResponseType::Conflict,
            4 => ErrorResponseType::Forbidden,
            5 => ErrorResponseType::UnprocessableEntity,
            6 => ErrorResponseType::InternalServerError,
            _ => ErrorResponseType::General,
        }
    }
}

/// Response body for returning an error message along with error type and list
/// of error items.  Super important for handling errors gracefully. With
/// different error types and detailed error items, clients can quickly identify
/// and respond to errors in a structured manner.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub error_type: ErrorResponseType,
    pub errors: Vec<ErrorItem>,
}


/// Enum representing different types of tokens.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TokenType {
    AccessToken = 0,
    RefreshToken = 1,
}

/// Implementation of conversion from integer to TokenType enum
impl From<i32> for TokenType {
    fn from(token_type: i32) -> Self {
        match token_type {
            0 => TokenType::AccessToken,
            1 => TokenType::RefreshToken,
            _ => TokenType::AccessToken,
        }
    }
}

/// Struct for representing an encrypted token payload. Because we use JWT as
/// the primary authorization method for endpoints, there is a need to encrypt
/// the token payload to prevent tampering, as JWT does not do that by default.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptedTokenPayload {
    pub token_type: TokenType,
    pub payload: String,
}
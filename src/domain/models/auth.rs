use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthLogin {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthRegister {
    pub email: String,
    pub name: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthSuccessfulResponse {
    pub token: String,
    pub refresh_token: String,
}
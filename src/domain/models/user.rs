use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Role {
    User = 0,
    Admin = 1,
    SuperAdmin = 2,
}

impl From<i32> for Role {
    fn from(role: i32) -> Self {
        match role {
            0 => Role::User,
            1 => Role::Admin,
            2 => Role::SuperAdmin,
            _ => Role::User,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub role: Role,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub reset_token: Option<String>,
    pub reset_token_expiry: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>, 
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUserPlainText {
    pub role: Option<Role>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub reset_token: Option<String>,
    pub reset_token_expiry: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUserHashed {
    pub role: Option<Role>,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub reset_token: Option<String>,
    pub reset_token_expiry: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUserPlainText {
    pub role: Option<Role>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub confirm_password: Option<String>,
    pub reset_token: Option<String>,
    pub reset_token_expiry: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUserHashed {
    pub role: Option<Role>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub reset_token: Option<String>,
    pub reset_token_expiry: Option<NaiveDateTime>,
}
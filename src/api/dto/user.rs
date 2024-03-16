use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::domain::models::user::{
    User,
    CreateUserPlainText,
    UpdateUserPlainText,
    CreateUserHashed,
    UpdateUserHashed
};
use crate::domain::repositories::repository::ResultPaging;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: Uuid,
    pub role: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub reset_token: Option<String>,
    pub reset_token_expiry: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserPlainTextDto {
    pub role: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub reset_token: Option<String>,
    pub reset_token_expiry: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserPlainTextDto {
    pub role: Option<i32>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub confirm_password: Option<String>,
    pub reset_token: Option<String>,
    pub reset_token_expiry: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserHashedDto {
    pub role: Option<i32>,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub reset_token: Option<String>,
    pub reset_token_expiry: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserHashedDto {
    pub role: Option<i32>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub reset_token: Option<String>,
    pub reset_token_expiry: Option<NaiveDateTime>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            id: user.id,
            role: user.role as i32,
            name: user.name,
            email: user.email,
            password_hash: user.password_hash,
            reset_token: user.reset_token,
            reset_token_expiry: user.reset_token_expiry,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<CreateUserPlainTextDto> for CreateUserPlainText {
    fn from(dto: CreateUserPlainTextDto) -> Self {
        CreateUserPlainText {
            role: dto.role.map(|role| role.into()),
            name: dto.name,
            email: dto.email,
            password: dto.password,
            confirm_password: dto.confirm_password,
            reset_token: dto.reset_token,
            reset_token_expiry: dto.reset_token_expiry,
        }
    }
}

impl From<UpdateUserPlainTextDto> for UpdateUserPlainText {
    fn from(dto: UpdateUserPlainTextDto) -> Self {
        UpdateUserPlainText {
            role: dto.role.map(|role| role.into()),
            name: dto.name,
            email: dto.email,
            password: dto.password,
            confirm_password: dto.confirm_password,
            reset_token: dto.reset_token,
            reset_token_expiry: dto.reset_token_expiry,
        }
    }
}

impl From<CreateUserPlainText> for CreateUserPlainTextDto {
    fn from(create_user: CreateUserPlainText) -> Self {
        CreateUserPlainTextDto {
            role: create_user.role.map(|role| role as i32),
            name: create_user.name,
            email: create_user.email,
            password: create_user.password,
            confirm_password: create_user.confirm_password,
            reset_token: create_user.reset_token,
            reset_token_expiry: create_user.reset_token_expiry,
        }
    }
}

impl From<UpdateUserPlainText> for UpdateUserPlainTextDto {
    fn from(update_user: UpdateUserPlainText) -> Self {
        UpdateUserPlainTextDto {
            role: update_user.role.map(|role| role as i32),
            name: update_user.name,
            email: update_user.email,
            password: update_user.password,
            confirm_password: update_user.confirm_password,
            reset_token: update_user.reset_token,
            reset_token_expiry: update_user.reset_token_expiry,
        }
    }
}

impl From<CreateUserHashedDto> for CreateUserHashed {
    fn from(dto: CreateUserHashedDto) -> Self {
        CreateUserHashed {
            role: dto.role.map(|role| role.into()),
            name: dto.name,
            email: dto.email,
            password_hash: dto.password_hash,
            reset_token: dto.reset_token,
            reset_token_expiry: dto.reset_token_expiry,
        }
    }
}

impl From<UpdateUserHashedDto> for UpdateUserHashed {
    fn from(dto: UpdateUserHashedDto) -> Self {
        UpdateUserHashed {
            role: dto.role.map(|role| role.into()),
            name: dto.name,
            email: dto.email,
            password_hash: dto.password_hash,
            reset_token: dto.reset_token,
            reset_token_expiry: dto.reset_token_expiry,
        }
    }
}

impl From<UpdateUserHashed> for UpdateUserHashedDto {
    fn from(update_user: UpdateUserHashed) -> Self {
        UpdateUserHashedDto {
            role: update_user.role.map(|role| role as i32),
            name: update_user.name,
            email: update_user.email,
            password_hash: update_user.password_hash,
            reset_token: update_user.reset_token,
            reset_token_expiry: update_user.reset_token_expiry,
        }
    }
}

impl From<CreateUserHashed> for CreateUserHashedDto {
    fn from(create_user: CreateUserHashed) -> Self {
        CreateUserHashedDto {
            role: create_user.role.map(|role| role as i32),
            name: create_user.name,
            email: create_user.email,
            password_hash: create_user.password_hash,
            reset_token: create_user.reset_token,
            reset_token_expiry: create_user.reset_token_expiry,
        }
    }
}

impl From<ResultPaging<User>> for ResultPaging<UserDto> {
    fn from(result_paging: ResultPaging<User>) -> Self {
        ResultPaging {
            items: result_paging.items.into_iter().map(UserDto::from).collect(),
            total: result_paging.total,
        }
    }
}

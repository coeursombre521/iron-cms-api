use diesel;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::domain::models::user::{User, CreateUserHashed, UpdateUserHashed, Role};
use crate::infrastructure::schema::users;

#[derive(Queryable)]
pub struct UserDiesel {
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

impl From<UserDiesel> for User {
    fn from(user: UserDiesel) -> Self {
        User {
            id: user.id,
            role: user.role.into(),
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

impl From<User> for UserDiesel {
    fn from(user: User) -> Self {
        UserDiesel {
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

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserDiesel {
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

impl From<CreateUserDiesel> for CreateUserHashed {
    fn from(user: CreateUserDiesel) -> Self {
        CreateUserHashed {
            role: Some(user.role.into()), 
            name: user.name,
            email: user.email,
            password_hash: user.password_hash,
            reset_token: user.reset_token,
            reset_token_expiry: user.reset_token_expiry,
        }
    }
}

impl From<CreateUserHashed> for CreateUserDiesel {
    fn from(user: CreateUserHashed) -> Self {
        CreateUserDiesel {
            id: Uuid::new_v4(),
            role: user.role.unwrap_or(Role::User) as i32,
            name: user.name,
            email: user.email,
            password_hash: user.password_hash,
            reset_token: user.reset_token,
            reset_token_expiry: user.reset_token_expiry,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}

impl From<CreateUserDiesel> for User {
    fn from(user: CreateUserDiesel) -> Self {
        User {
            id: user.id,
            role: user.role.into(),
            name: user.name,
            email: user.email,
            password_hash: user.password_hash,
            reset_token: None,
            reset_token_expiry: None,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUserDiesel {
    pub role: Option<i32>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub reset_token: Option<String>,
    pub reset_token_expiry: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<UpdateUserDiesel> for UpdateUserHashed {
    fn from(user: UpdateUserDiesel) -> Self {
        UpdateUserHashed {
            role: user.role.map(|role| role.into()),
            name: user.name,
            email: user.email,
            password_hash: user.password_hash,
            reset_token: user.reset_token,
            reset_token_expiry: user.reset_token_expiry,
        }
    }
}

impl From<UpdateUserHashed> for UpdateUserDiesel {
    fn from(user: UpdateUserHashed) -> Self {
        UpdateUserDiesel {
            role: user.role.map(|role| role as i32),
            name: user.name,
            email: user.email,
            password_hash: user.password_hash,
            reset_token: user.reset_token,
            reset_token_expiry: user.reset_token_expiry,
            updated_at: Some(chrono::Utc::now().naive_utc()),
        }
    }
}

impl From<UpdateUserDiesel> for User {
    fn from(user: UpdateUserDiesel) -> Self {
        User {
            id: Uuid::new_v4(),
            role: Role::User,
            name: user.name.unwrap_or_default(),
            email: user.email.unwrap_or_default(),
            password_hash: user.password_hash.unwrap_or_default(),
            reset_token: None,
            reset_token_expiry: None,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: Some(chrono::Utc::now().naive_utc()),
        }
    }
}

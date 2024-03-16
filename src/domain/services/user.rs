use uuid::Uuid;
use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::user::{User, CreateUserPlainText, UpdateUserPlainText};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user::UserQueryParams;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create(&self, new_user: CreateUserPlainText) -> Result<User, CommonError>;
    async fn list(&self, params: UserQueryParams) -> Result<ResultPaging<User>, CommonError>;
    async fn get(&self, user_id: Uuid) -> Result<User, CommonError>;
    async fn update(&self, user_id: Uuid, updated_user: UpdateUserPlainText) -> Result<User, CommonError>;
    async fn delete(&self, user_id: Uuid) -> Result<bool, CommonError>;
}

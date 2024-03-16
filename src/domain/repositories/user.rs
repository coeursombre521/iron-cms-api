use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::repositories::repository::{QueryParams, ResultPaging, RepositoryResult, DEFAULT_LIMIT, DEFAULT_OFFSET};
use crate::domain::models::user::{User, CreateUserHashed, UpdateUserHashed};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl QueryParams for UserQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, new_user: &CreateUserHashed) -> RepositoryResult<User>;
    async fn list(&self, params: UserQueryParams) -> RepositoryResult<ResultPaging<User>>;
    async fn get(&self, user_id: Uuid) -> RepositoryResult<User>;
    async fn update(&self, user_id: Uuid, updated_user: &UpdateUserHashed) -> RepositoryResult<User>;
    async fn delete(&self, user_id: Uuid) -> RepositoryResult<bool>;
}

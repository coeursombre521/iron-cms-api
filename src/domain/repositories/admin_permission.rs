use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::repositories::repository::{QueryParams, ResultPaging, RepositoryResult, DEFAULT_LIMIT, DEFAULT_OFFSET};
use crate::domain::models::admin_permission::{AdminPermission, CreateAdminPermission, UpdateAdminPermission};

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminPermissionQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl QueryParams for AdminPermissionQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait AdminPermissionRepository: Send + Sync {
    async fn create(&self, new_admin_permission: &CreateAdminPermission) -> RepositoryResult<AdminPermission>;
    async fn list(&self, params: AdminPermissionQueryParams) -> RepositoryResult<ResultPaging<AdminPermission>>;
    async fn get(&self, admin_permission_id: Uuid) -> RepositoryResult<AdminPermission>;
    async fn update(&self, admin_permission_id: Uuid, updated_admin_permission: &UpdateAdminPermission) -> RepositoryResult<AdminPermission>;
    async fn delete(&self, admin_permission_id: Uuid) -> RepositoryResult<bool>;
}

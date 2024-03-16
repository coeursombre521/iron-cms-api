use uuid::Uuid;
use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::admin_permission::{AdminPermission, CreateAdminPermission, UpdateAdminPermission};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::admin_permission::AdminPermissionQueryParams;

#[async_trait]
pub trait AdminPermissionService: Send + Sync {
    async fn create(&self, new_admin_permission: CreateAdminPermission) -> Result<AdminPermission, CommonError>;
    async fn list(&self, params: AdminPermissionQueryParams) -> Result<ResultPaging<AdminPermission>, CommonError>;
    async fn get(&self, admin_permission_id: Uuid) -> Result<AdminPermission, CommonError>;
    async fn update(&self, admin_permission_id: Uuid, updated_admin_permission: UpdateAdminPermission) -> Result<AdminPermission, CommonError>;
    async fn delete(&self, admin_permission_id: Uuid) -> Result<bool, CommonError>;
}

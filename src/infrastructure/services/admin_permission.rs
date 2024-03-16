use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::error::CommonError;
use crate::domain::models::admin_permission::{AdminPermission, CreateAdminPermission, UpdateAdminPermission};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::admin_permission::{AdminPermissionQueryParams, AdminPermissionRepository};
use crate::domain::services::admin_permission::AdminPermissionService;

#[derive(Clone)]
pub struct AdminPermissionServiceImpl {
    pub repository: Arc<dyn AdminPermissionRepository>,
}

impl AdminPermissionServiceImpl {
    pub fn new(repository: Arc<dyn AdminPermissionRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl AdminPermissionService for AdminPermissionServiceImpl {
    async fn create(&self, new_admin_permission: CreateAdminPermission) -> Result<AdminPermission, CommonError> {
        let cloned = new_admin_permission.clone();
        self.repository.create(&cloned)
            .await
            .map_err(CommonError::from)
    }

    async fn list(&self, params: AdminPermissionQueryParams) -> Result<ResultPaging<AdminPermission>, CommonError> {
        self.repository.list(params)
            .await
            .map_err(CommonError::from)
    }

    async fn get(&self, admin_permission_id: Uuid) -> Result<AdminPermission, CommonError> {
        self.repository.get(admin_permission_id)
            .await
            .map_err(CommonError::from)
    }

    async fn update(&self, admin_permission_id: Uuid, updated_admin_permission: UpdateAdminPermission) -> Result<AdminPermission, CommonError> {
        let cloned = updated_admin_permission.clone();
        self.repository.update(admin_permission_id, &cloned)
            .await
            .map_err(CommonError::from)
    }

    async fn delete(&self, admin_permission_id: Uuid) -> Result<bool, CommonError> {
        self.repository.delete(admin_permission_id)
            .await
            .map_err(CommonError::from)
    }
}
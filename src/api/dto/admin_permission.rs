use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::admin_permission::{AdminPermission, CreateAdminPermission, UpdateAdminPermission};
use crate::domain::repositories::repository::ResultPaging;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminPermissionDto {
    pub id: String,
    pub user_id: String,
    pub permission: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAdminPermissionDto {
    pub user_id: String,
    pub permission: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAdminPermissionDto {
    pub user_id: String,
    pub permission: i32,
}

impl From<AdminPermission> for AdminPermissionDto {
    fn from(admin_permission: AdminPermission) -> Self {
        AdminPermissionDto {
            id: admin_permission.id.to_string(),
            user_id: admin_permission.user_id.to_string(),
            permission: admin_permission.permission as i32,
        }
    }
}

impl From<CreateAdminPermissionDto> for CreateAdminPermission {
    fn from(dto: CreateAdminPermissionDto) -> Self {
        CreateAdminPermission {
            user_id: Uuid::parse_str(&dto.user_id).unwrap(),
            permission: dto.permission.into(),
        }
    }
}

impl From<UpdateAdminPermissionDto> for UpdateAdminPermission {
    fn from(dto: UpdateAdminPermissionDto) -> Self {
        UpdateAdminPermission {
            user_id: Uuid::parse_str(&dto.user_id).unwrap(),
            permission: dto.permission.into(),
        }
    }
}

impl From<AdminPermission> for UpdateAdminPermissionDto {
    fn from(admin_permission: AdminPermission) -> Self {
        UpdateAdminPermissionDto {
            user_id: admin_permission.user_id.to_string(),
            permission: admin_permission.permission as i32,
        }
    }
}

impl From<AdminPermission> for CreateAdminPermissionDto {
    fn from(admin_permission: AdminPermission) -> Self {
        CreateAdminPermissionDto {
            user_id: admin_permission.user_id.to_string(),
            permission: admin_permission.permission as i32,
        }
    }
}

impl From<ResultPaging<AdminPermission>> for ResultPaging<AdminPermissionDto> {
    fn from(result: ResultPaging<AdminPermission>) -> Self {
        ResultPaging {
            items: result.items.into_iter().map(AdminPermissionDto::from).collect(),
            total: result.total,
        }
    }
}

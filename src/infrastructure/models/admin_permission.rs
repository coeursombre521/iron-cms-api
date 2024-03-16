use diesel;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::domain::models::admin_permission::{AdminPermission, CreateAdminPermission, UpdateAdminPermission};
use crate::infrastructure::schema::admin_permissions;

#[derive(Queryable)]
pub struct AdminPermissionDiesel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub permission: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<AdminPermissionDiesel> for AdminPermission {
    fn from(permission: AdminPermissionDiesel) -> Self {
        AdminPermission {
            id: permission.id,
            user_id: permission.user_id,
            permission: permission.permission.into(),
            created_at: permission.created_at,
            updated_at: permission.updated_at,
        }
    }
}

impl From<AdminPermission> for AdminPermissionDiesel {
    fn from(permission: AdminPermission) -> Self {
        AdminPermissionDiesel {
            id: permission.id,
            user_id: permission.user_id,
            permission: permission.permission as i32,
            created_at: permission.created_at,
            updated_at: permission.updated_at,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = admin_permissions)]
pub struct CreateAdminPermissionDiesel {
    pub user_id: Uuid,
    pub permission: i32,
}

impl From<CreateAdminPermissionDiesel> for CreateAdminPermission {
    fn from(permission: CreateAdminPermissionDiesel) -> Self {
        CreateAdminPermission {
            user_id: permission.user_id,
            permission: permission.permission.into(),
        }
    }
}

impl From<CreateAdminPermission> for CreateAdminPermissionDiesel {
    fn from(permission: CreateAdminPermission) -> Self {
        CreateAdminPermissionDiesel {
            user_id: permission.user_id,
            permission: permission.permission as i32,
        }
    }
}

impl From<CreateAdminPermissionDiesel> for AdminPermission {
    fn from(permission: CreateAdminPermissionDiesel) -> Self {
        AdminPermission {
            id: Uuid::new_v4(),
            user_id: permission.user_id,
            permission: permission.permission.into(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = admin_permissions)]
pub struct UpdateAdminPermissionDiesel {
    pub user_id: Uuid,
    pub permission: i32,
}

impl From<UpdateAdminPermissionDiesel> for UpdateAdminPermission {
    fn from(permission: UpdateAdminPermissionDiesel) -> Self {
        UpdateAdminPermission {
            user_id: permission.user_id,
            permission: permission.permission.into(),
        }
    }
}

impl From<UpdateAdminPermission> for UpdateAdminPermissionDiesel {
    fn from(permission: UpdateAdminPermission) -> Self {
        UpdateAdminPermissionDiesel {
            user_id: permission.user_id,
            permission: permission.permission as i32,
        }
    }
}

impl From<UpdateAdminPermissionDiesel> for AdminPermission {
    fn from(permission: UpdateAdminPermissionDiesel) -> Self {
        AdminPermission {
            id: Uuid::new_v4(),
            user_id: permission.user_id,
            permission: permission.permission.into(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
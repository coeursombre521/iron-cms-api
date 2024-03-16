use std::sync::Arc;
use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::models::admin_permission::{AdminPermission, CreateAdminPermission, UpdateAdminPermission};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::domain::repositories::admin_permission::{AdminPermissionQueryParams, AdminPermissionRepository};
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::database::postgresql::DBConn;
use crate::infrastructure::models::admin_permission::{AdminPermissionDiesel, CreateAdminPermissionDiesel, UpdateAdminPermissionDiesel};

pub struct AdminPermissionRepositoryImpl {
    pool: Arc<DBConn>,
}

impl AdminPermissionRepositoryImpl {
    pub fn new(db: Arc<DBConn>) -> Self {
        Self { pool: db }
    }
}

#[async_trait]
impl AdminPermissionRepository for AdminPermissionRepositoryImpl {
    async fn create(&self, new_admin_permission: &CreateAdminPermission) -> RepositoryResult<AdminPermission> {
        use crate::infrastructure::schema::admin_permissions::dsl::admin_permissions;
        let new_admin_permission_diesel = CreateAdminPermissionDiesel::from(new_admin_permission.clone());
        let conn = self.pool.clone();
        let result = run(move || {
            let mut conn = conn.get().unwrap();
            diesel::insert_into(admin_permissions)
                .values(&new_admin_permission_diesel)
                .get_result::<AdminPermissionDiesel>(&mut conn)
        })
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(AdminPermission::from)?;
        Ok(result)
    }

    async fn list(&self, query_params: AdminPermissionQueryParams) -> RepositoryResult<ResultPaging<AdminPermission>> {
        use crate::infrastructure::schema::admin_permissions::dsl::admin_permissions;
        let pool = self.pool.clone();
        let builder = admin_permissions.limit(query_params.limit()).offset(query_params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<AdminPermissionDiesel>(&mut conn)
        })
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(ResultPaging {
            total: result.len() as i64,
            items: result.into_iter().map(AdminPermission::from).collect(),
        })
    }

    async fn get(&self, admin_permission_id: Uuid) -> RepositoryResult<AdminPermission> {
        use crate::infrastructure::schema::admin_permissions::dsl::{admin_permissions, id};
        let mut conn = self.pool.get().unwrap();
        run(move || admin_permissions.filter(id.eq(admin_permission_id)).first::<AdminPermissionDiesel>(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| -> AdminPermission { AdminPermission::from(v) })
    }

    async fn update(&self, admin_permission_id: Uuid, updated_admin_permission: &UpdateAdminPermission) -> RepositoryResult<AdminPermission> {
        use crate::infrastructure::schema::admin_permissions::dsl::{admin_permissions, id};
        let updated_admin_permission_diesel = UpdateAdminPermissionDiesel::from(updated_admin_permission.clone());
        let pool = self.pool.clone();
        run(move || {
            let mut conn = pool.get().unwrap();
            diesel::update(admin_permissions.filter(id.eq(admin_permission_id)))
                .set(updated_admin_permission_diesel)
                .get_result::<AdminPermissionDiesel>(&mut conn)
        })
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| -> AdminPermission { AdminPermission::from(v) })
    }

    async fn delete(&self, admin_permission_id: Uuid) -> RepositoryResult<bool> {
        use crate::infrastructure::schema::admin_permissions::dsl::{admin_permissions, id};
        let pool = self.pool.clone();
        run(move || {
            let mut conn = pool.get().unwrap();
            diesel::delete(admin_permissions.filter(id.eq(admin_permission_id))).execute(&mut conn)
        })
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| v > 0)
    }
}
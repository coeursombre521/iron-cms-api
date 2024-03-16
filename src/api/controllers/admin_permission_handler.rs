use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

use crate::api::dto::admin_permission::{AdminPermissionDto, CreateAdminPermissionDto, UpdateAdminPermissionDto};
use crate::domain::error::ApiError;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::admin_permission::AdminPermissionQueryParams;
use crate::domain::services::admin_permission::AdminPermissionService;

pub async fn create_admin_permission_handler(
    admin_permission_service: web::Data<dyn AdminPermissionService>,
    post_data: web::Json<CreateAdminPermissionDto>
) -> Result<web::Json<AdminPermissionDto>, ApiError> {
    let admin_permission = admin_permission_service.create(post_data.into_inner().into()).await?;
    Ok(web::Json(admin_permission.into()))
}

pub async fn list_admin_permission_handler(
    admin_permission_service: web::Data<dyn AdminPermissionService>,
    params: web::Query<AdminPermissionQueryParams>,
) -> Result<web::Json<ResultPaging<AdminPermissionDto>>, ApiError> {
    let admin_permissions = admin_permission_service.list(params.into_inner()).await?;
    Ok(web::Json(admin_permissions.into()))
}

pub async fn get_admin_permission_handler(
    admin_permission_service: web::Data<dyn AdminPermissionService>,
    admin_permission_id: web::Path<Uuid>,
) -> Result<web::Json<AdminPermissionDto>, ApiError> {
    let admin_permission = admin_permission_service.get(admin_permission_id.into_inner()).await?;
    Ok(web::Json(admin_permission.into()))
}

pub async fn update_admin_permission_handler(
    admin_permission_service: web::Data<dyn AdminPermissionService>,
    admin_permission_id: web::Path<Uuid>,
    put_data: web::Json<UpdateAdminPermissionDto>,
) -> Result<web::Json<AdminPermissionDto>, ApiError> {
    let admin_permission = admin_permission_service.update(admin_permission_id.into_inner(), put_data.into_inner().into()).await?;
    Ok(web::Json(admin_permission.into()))
}

pub async fn delete_admin_permission_handler(
    admin_permission_service: web::Data<dyn AdminPermissionService>,
    admin_permission_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    admin_permission_service.delete(admin_permission_id.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}


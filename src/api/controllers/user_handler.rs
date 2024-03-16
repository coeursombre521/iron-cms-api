use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

use crate::api::dto::user::{UserDto, CreateUserPlainTextDto, UpdateUserPlainTextDto};
use crate::domain::error::ApiError;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user::UserQueryParams;
use crate::domain::services::user::UserService;

pub async fn create_user_handler(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<CreateUserPlainTextDto>,
) -> Result<web::Json<UserDto>, ApiError> {
    let user = user_service.create(post_data.into_inner().into()).await?;
    Ok(web::Json(user.into()))
}

pub async fn list_user_handler(
    user_service: web::Data<dyn UserService>,
    params: web::Query<UserQueryParams>,
) -> Result<web::Json<ResultPaging<UserDto>>, ApiError> {
    let users = user_service.list(params.into_inner()).await?;
    Ok(web::Json(users.into()))
}

pub async fn get_user_handler(
    user_service: web::Data<dyn UserService>,
    user_id: web::Path<Uuid>,
) -> Result<web::Json<UserDto>, ApiError> {
    let user = user_service.get(user_id.into_inner()).await?;
    Ok(web::Json(user.into()))
}

pub async fn update_user_handler(
    user_service: web::Data<dyn UserService>,
    user_id: web::Path<Uuid>,
    put_data: web::Json<UpdateUserPlainTextDto>,
) -> Result<web::Json<UserDto>, ApiError> {
    let user = user_service.update(user_id.into_inner(), put_data.into_inner().into()).await?;
    Ok(web::Json(user.into()))
}

pub async fn delete_user_handler(
    user_service: web::Data<dyn UserService>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    user_service.delete(user_id.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}

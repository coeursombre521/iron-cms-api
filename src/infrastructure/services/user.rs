use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::error::CommonError;
use crate::domain::models::user::{
    User,
    CreateUserPlainText,
    CreateUserHashed,
    UpdateUserPlainText,
    UpdateUserHashed
};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user::{UserQueryParams, UserRepository};
use crate::domain::services::user::UserService;
use crate::services::constants;
use crate::services::error::SecurityError;
use crate::services::traits::password_hash::PasswordHashService;
use crate::services::utils::format;

#[derive(Clone)]
pub struct UserServiceImpl<'a> {
    pub repository: Arc<dyn UserRepository>,
    hash_service: Arc<dyn PasswordHashService + 'a>
}

impl<'a> UserServiceImpl<'a> {
    pub fn new(repository: Arc<dyn UserRepository>, hash_service: Arc<dyn PasswordHashService + 'a>) -> Self {
        Self {
            repository,
            hash_service
        }
    }

    fn password_fields_are_both_specified(&self, password: &Option<String>, confirm_password: &Option<String>) -> bool {
        password.is_some() && confirm_password.is_some()
    }

    fn check_if_passwords_match(&self, password: &str, confirm_password: &str) -> Result<(), CommonError> {
        if password != confirm_password {
            return Err(CommonError::from(SecurityError {
                message: format::format_error_string(constants::SEC_ERR_PASS_NOT_MATCH, "`password` and `confirm_password` fields do not match"),
                context: constants::ERR_CONTEXT_LOGIN.to_string(),
            }));
        }
        Ok(())
    }

    fn hash_password(&self, password: &str) -> Result<String, CommonError> {
        match self.hash_service.hash_password(password) {
            Ok(hash) => Ok(hash),
            Err(err) => Err(CommonError::from(err)),
        }
    }
}

#[async_trait]
impl<'a> UserService for UserServiceImpl<'a> {
    async fn create(&self, new_user: CreateUserPlainText) -> Result<User, CommonError> {
        self.check_if_passwords_match(&new_user.password, &new_user.confirm_password)
            .map_err(CommonError::from)?;

        let hashed = CreateUserHashed {
            role: new_user.role,
            name: new_user.name,
            email: new_user.email,
            password_hash: self.hash_password(&new_user.password)?,
            reset_token: new_user.reset_token,
            reset_token_expiry: new_user.reset_token_expiry
        };

        self.repository.create(&hashed)
            .await
            .map_err(CommonError::from)
    }

    async fn list(&self, params: UserQueryParams) -> Result<ResultPaging<User>, CommonError> {
        self.repository.list(params)
            .await
            .map_err(CommonError::from)
    }

    async fn get(&self, user_id: Uuid) -> Result<User, CommonError> {
        self.repository.get(user_id)
            .await
            .map_err(CommonError::from)
    }

    async fn update(&self, user_id: Uuid, update_user: UpdateUserPlainText) -> Result<User, CommonError> {
        let mut hashed = UpdateUserHashed {
            role: update_user.role,
            name: update_user.name,
            email: update_user.email,
            password_hash: None,
            reset_token: update_user.reset_token,
            reset_token_expiry: update_user.reset_token_expiry
        };

        if self.password_fields_are_both_specified(&update_user.password, &update_user.confirm_password) {
            self.check_if_passwords_match(&update_user.password.clone().unwrap(), &update_user.confirm_password.unwrap())
                .map_err(CommonError::from)?;

            hashed.password_hash = Some(self.hash_password(&update_user.password.unwrap())?);
        }

        self.repository.update(user_id, &hashed)
            .await
            .map_err(CommonError::from)
    }

    async fn delete(&self, user_id: Uuid) -> Result<bool, CommonError> {
        self.repository.delete(user_id)
            .await
            .map_err(CommonError::from)
    }
}
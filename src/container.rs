use std::sync::Arc;

use crate::domain::repositories::admin_permission::AdminPermissionRepository;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::admin_permission::AdminPermissionService;
use crate::domain::services::user::UserService;
use crate::infrastructure::database::postgresql::db_pool;
use crate::infrastructure::repositories::admin_permission::AdminPermissionRepositoryImpl;
use crate::infrastructure::repositories::user::UserRepositoryImpl;
use crate::infrastructure::services::admin_permission::AdminPermissionServiceImpl;
use crate::infrastructure::services::user::UserServiceImpl;
use crate::services::concrete::argon2id_hash::Argon2IdHashService;

pub struct Container {
    pub admin_permission_service: Arc<dyn AdminPermissionService>,
    pub user_service: Arc<dyn UserService>, 
}

impl Container {
    pub fn new() -> Self {
        let admin_permission_repository: Arc<dyn AdminPermissionRepository> = Arc::new(
            AdminPermissionRepositoryImpl::new(Arc::new(db_pool()))
        );
        let user_repository: Arc<dyn UserRepository> = Arc::new(
            UserRepositoryImpl::new(Arc::new(db_pool()))
        );
        let admin_permission_service: Arc<dyn AdminPermissionService> = Arc::new(
            AdminPermissionServiceImpl::new(admin_permission_repository)
        );
        let user_service: Arc<dyn UserService> = Arc::new(
            UserServiceImpl::new(
                user_repository,
                Arc::new(Argon2IdHashService::new())
            )
        );
        Container {
            admin_permission_service,
            user_service,
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
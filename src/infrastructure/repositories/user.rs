use std::sync::Arc;
use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::models::user::{User, CreateUserHashed, UpdateUserHashed};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::domain::repositories::user::{UserQueryParams, UserRepository};
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::database::postgresql::DBConn;
use crate::infrastructure::models::user::{UserDiesel, CreateUserDiesel, UpdateUserDiesel};

pub struct UserRepositoryImpl {
    pool: Arc<DBConn>,
}

impl UserRepositoryImpl {
    pub fn new(db: Arc<DBConn>) -> Self {
        Self { pool: db }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, new_user: &CreateUserHashed) -> RepositoryResult<User> {
        use crate::infrastructure::schema::users::dsl::users;
        let new_user_diesel = CreateUserDiesel::from(new_user.clone());
        let conn = self.pool.clone();
        let result = run(move || {
            let mut conn = conn.get().unwrap();
            diesel::insert_into(users)
                .values(&new_user_diesel)
                .get_result::<UserDiesel>(&mut conn)
        })
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(User::from)?;
        Ok(result)
    }

    async fn list(&self, params: UserQueryParams) -> RepositoryResult<ResultPaging<User>> {
        use crate::infrastructure::schema::users::dsl::users;
        let pool = self.pool.clone();
        let builder = users.limit(params.limit()).offset(params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<UserDiesel>(&mut conn)
        })
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(ResultPaging {
            total: result.len() as i64,
            items: result.into_iter().map(User::from).collect(),
        })
    }

    async fn get(&self, user_id: Uuid) -> RepositoryResult<User> {
        use crate::infrastructure::schema::users::dsl::{users, id};
        let mut conn = self.pool.get().unwrap();
        run(move || users.filter(id.eq(user_id)).first::<UserDiesel>(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| -> User { User::from(v) })
    }

    async fn update(&self, user_id: Uuid, updated_user: &UpdateUserHashed) -> RepositoryResult<User> {
        use crate::infrastructure::schema::users::dsl::{users, id};
        let updated_user_diesel = UpdateUserDiesel::from(updated_user.clone());
        let pool = self.pool.clone();
        run(move || {
            let mut conn = pool.get().unwrap();
            diesel::update(users.filter(id.eq(user_id)))
                .set(updated_user_diesel)
                .get_result::<UserDiesel>(&mut conn)
        })
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| -> User { User::from(v) })
    }

    async fn delete(&self, user_id: Uuid) -> RepositoryResult<bool> {
        use crate::infrastructure::schema::users::dsl::{users, id};
        let pool = self.pool.clone();
        run(move || {
            let mut conn = pool.get().unwrap();
            diesel::delete(users.filter(id.eq(user_id))).execute(&mut conn)
        })
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| v > 0)
    }
}
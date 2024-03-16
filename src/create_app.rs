use actix_web::{App, web};
use actix_web::Error;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use tracing_actix_web::TracingLogger;

use crate::api::controllers::admin_permission_handler::{
    create_admin_permission_handler,
    list_admin_permission_handler,
    get_admin_permission_handler,
    update_admin_permission_handler,
    delete_admin_permission_handler 
};
use crate::api::controllers::user_handler::{
    create_user_handler,
    list_user_handler,
    get_user_handler,
    update_user_handler,
    delete_user_handler
};
use crate::container::Container;

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let container = Container::new();
    let admin_permission_service = container.admin_permission_service.clone();
    let user_service = container.user_service.clone();
    
    App::new()
        .app_data(web::Data::from(admin_permission_service.clone()))
        .app_data(web::Data::from(user_service.clone()))
        .wrap(TracingLogger::default())
        .service(
            web::scope("/api").service(
                web::scope("/admin_permissions")
                    .route("", web::post().to(create_admin_permission_handler))
                    .route("", web::get().to(list_admin_permission_handler))
                    .route("/{admin_permission_id}", web::get().to(get_admin_permission_handler))
                    .route("/{admin_permission_id}", web::put().to(update_admin_permission_handler))
                    .route("/{admin_permission_id}", web::delete().to(delete_admin_permission_handler))
            ).service(
                web::scope("/users")
                    .route("", web::post().to(create_user_handler))
                    .route("", web::get().to(list_user_handler))
                    .route("/{user_id}", web::get().to(get_user_handler))
                    .route("/{user_id}", web::put().to(update_user_handler))
                    .route("/{user_id}", web::delete().to(delete_user_handler))
            )
        )
}
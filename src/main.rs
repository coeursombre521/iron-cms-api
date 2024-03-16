use actix_web::HttpServer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use tonguetrek_backend::{
    create_app,
    constants,
    env_check::check_env_variables,
    services::utils::envutil::get_env_var_as_str,
    services::utils::envutil::get_env_var_as_type
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    
    check_env_variables();

    let domain = get_env_var_as_str(constants::ENV_SERVER_DOMAIN).unwrap();
    let port = get_env_var_as_type::<u16>(constants::ENV_SERVER_PORT).unwrap();

    HttpServer::new(move || {
        create_app::create_app()
    })
        .bind((domain, port))?
        .run()
        .await
}

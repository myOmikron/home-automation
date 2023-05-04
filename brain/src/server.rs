use std::net::SocketAddr;

use actix_files::Files;
use actix_toolbox::tb_middleware::{
    setup_logging_mw, DBSessionStore, LoggingMiddlewareConfig, PersistentSession, SessionMiddleware,
};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Key;
use actix_web::http::StatusCode;
use actix_web::middleware::{Compress, ErrorHandlers};
use actix_web::web::{scope, Data, JsonConfig, PayloadConfig};
use actix_web::{App, HttpServer};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use log::info;
use rorm::Database;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::Config;
use crate::handler::*;
use crate::middleware::{handle_not_found, json_extractor_error, AuthenticationRequired};
use crate::swagger::ApiDoc;

/// Start the server
pub async fn start_server(conf: &Config, db: Database) -> Result<(), String> {
    let socket_addr = SocketAddr::new(conf.server.listen_address, conf.server.listen_port);

    if conf.server.secret_key.is_empty() {
        return Err("SecretKey is missing. Generate one using the keygen subcommand.".to_string());
    }
    let key = Key::from(
        &BASE64_STANDARD
            .decode(conf.server.secret_key.clone())
            .map_err(|e| format!("Error decoding secret key: {e}"))?,
    );

    info!("Starting to listen on http://{socket_addr}");

    let frontend_path = conf.server.frontend_path.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(PayloadConfig::default())
            .app_data(JsonConfig::default().error_handler(json_extractor_error))
            .app_data(Data::new(db.clone()))
            .wrap(setup_logging_mw(LoggingMiddlewareConfig::default()))
            .wrap(Compress::default())
            .wrap(
                SessionMiddleware::builder(DBSessionStore::new(db.clone()), key.clone())
                    .session_lifecycle(PersistentSession::session_ttl(
                        PersistentSession::default(),
                        Duration::hours(24),
                    ))
                    .build(),
            )
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, handle_not_found))
            .wrap(setup_logging_mw(LoggingMiddlewareConfig::default()))
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi()))
            .service(scope("/api/v1/auth").service(login).service(logout))
            .service(
                scope("/api/v1")
                    .wrap(AuthenticationRequired)
                    .service(test)
                    .service(get_me),
            )
            .service(Files::new("/", &frontend_path).index_file("index.html"))
    })
    .bind(socket_addr)
    .map_err(|e| format!("Could not bind to {socket_addr}: {e}"))?
    .run()
    .await
    .map_err(|e| format!("Could not start server: {e}"))?;
    Ok(())
}

use std::net::SocketAddr;

use actix_files::Files;
use actix_toolbox::tb_middleware::{setup_logging_mw, LoggingMiddlewareConfig};
use actix_web::web::{scope, Data};
use actix_web::{App, HttpServer};
use log::info;
use rorm::Database;

use crate::config::Config;
use crate::handler;

/// Start the server
pub async fn start_server(conf: &Config, db: Database) -> Result<(), String> {
    let socket_addr = SocketAddr::new(conf.server.listen_address, conf.server.listen_port);

    info!("Starting to listen on http://{socket_addr}");

    let frontend_path = conf.server.frontend_path.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .wrap(setup_logging_mw(LoggingMiddlewareConfig::default()))
            .service(Files::new("/", &frontend_path).index_file("index.html"))
            .service(scope("/api/v1"))
    })
    .bind(socket_addr)
    .map_err(|e| format!("Could not bind to {socket_addr}: {e}"))?
    .run()
    .await
    .map_err(|e| format!("Could not start server: {e}"))?;
    Ok(())
}

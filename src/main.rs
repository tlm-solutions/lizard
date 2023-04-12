#[deny(missing_docs)]
mod routes;
mod structs;
//mod state;

use structs::Args;

use clap::Parser;
use log::{debug, info};

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use redis::Client;

/// this function reads the uri specifier for the redis instance from env variables
pub fn get_redis_uri() -> String {
    let default_redis_port = "6379".to_string();
    let default_redis_host = "127.0.0.1".to_string();

    format!(
        "redis://{}:{}",
        std::env::var("REDIS_HOST").unwrap_or(default_redis_host),
        std::env::var("REDIS_PORT").unwrap_or(default_redis_port)
    )
}

/// returns the redis connection pool
pub fn connect_to_redis() -> Option<Client> {
    let redis_uri = get_redis_uri();
    Client::open(redis_uri).ok()
}

pub fn get_prometheus() -> PrometheusMetrics {
    PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .expect("Failed to create prometheus metric endpoint")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = Args::parse();

    if args.swagger {
        println!("{}", routes::ApiDoc::openapi().to_pretty_json().unwrap());
        return Ok(());
    }

    info!("Starting the reptiloid service to misinform our users ... ");

    let host = args.host.as_str();
    let port = args.port;

    debug!("Listening on: {}:{}", host, port);

    let prometheus = get_prometheus();
    let redis_connectionpool = connect_to_redis().expect("cannot connect to the redis");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        App::new()
            .wrap(cors)
            .wrap(prometheus.clone())
            .wrap(Logger::default())
            .app_data(redis_connectionpool.clone())
            .service(
                web::scope("/v1")
                    .service(routes::vehicles::vehicles_list)
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", routes::ApiDoc::openapi()),
            )
    })
    .bind((host, port))?
    .run()
    .await
}

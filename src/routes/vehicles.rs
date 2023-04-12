use crate::routes::ServerError;

use tlms::locations::waypoint::Waypoint;

use actix_web::{get, web, HttpRequest};
use log::{error, info};
use redis::{Client, Commands};

/// will return a list of all vehicles inside this region with their last seen position
#[utoipa::path(
    get,
    path = "/v1/vehicles/{region}/",
    params(
        ("region" = i64, Path, description = "Identitier of the region")
    ),
    responses(
        (status = 200, description = "list of vehicles"),
        (status = 500, description = "postgres pool error"),
    ),
)]
//#[get("/vehicles/{region}")]
pub async fn vehicles_list(
    _req: HttpRequest,
    path: web::Path<(i64,)>,
    redis_pool: web::Data<Client>,
) -> Result<web::Json<Vec<Waypoint>>, ServerError> {
    let mut redis_connection = match redis_pool.get_connection() {
        Ok(value) => value,
        Err(e) => {
            error!("cannot fetch redis connection {:?}", e);
            return Err(ServerError::InternalError);
        }
    };

    let waypoint_string: String = match redis_connection.get(format!("r{}", path.0)) {
        Ok(value) => value,
        Err(e) => {
            error!("cannot find region with this key {:?}", e);
            return Err(ServerError::BadClientData);
        }
    };

    info!("found redis value {}", &waypoint_string);

    let waypoints = match serde_json::from_str(&waypoint_string) {
        Ok(value) => value,
        Err(e) => {
            error!(
                "cannot deserialize value from redis with error {:?} and value {}",
                e, &waypoint_string
            );
            return Err(ServerError::InternalError);
        }
    };

    Ok(web::Json(waypoints))
}

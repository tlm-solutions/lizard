use crate::routes::ServerError;

use tlms::locations::waypoint::Waypoint;

use actix_web::{web, HttpRequest, get};
use log::error;
use redis::{cluster::ClusterClient, Commands};

/// will return a list of all vehicles inside this region with their last seen position
#[utoipa::path(
    get,
    path = "/vehicles/{region}/",
    params(
        ("id" = i64, Path, description = "Identitier of the region")
    ),
    responses(
        (status = 200, description = "list of vehicles"),
        (status = 500, description = "postgres pool error"),
    ),
)]
#[get("/vehicles/{region}/")]
pub async fn vehicles_list(
    _req: HttpRequest,
    path: web::Path<(i64,)>,
    redis_pool: web::Data<ClusterClient>,
) -> Result<web::Json<Vec<Waypoint>>, ServerError> {
    let mut redis_connection = match redis_pool.get_connection() {
        Ok(value) => value,
        Err(e) => {
            error!("cannot fetch redis connection {:?}", e);
            return Err(ServerError::InternalError);
        }
    };

    let waypoints: Vec<String> = match redis_connection.get(format!("r{}", path.0)) {
        Ok(value) => value,
        Err(e) => {
            error!("cannot find region with this key {:?}", e);
            return Err(ServerError::BadClientData);
        }
    };

    Ok(web::Json(
        waypoints
            .iter()
            .map(|x| serde_json::from_str(x).unwrap())
            .collect(),
    ))
}

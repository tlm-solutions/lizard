use crate::routes::ServerError;

use tlms::locations::waypoint::Waypoint;

use actix_web::{get, web, HttpRequest};
use log::error;
use redis::{Client, Commands};
use serde::Serialize;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use gtfs_realtime::{FeedEntity, FeedMessage};

#[derive(Debug, Clone, Serialize)]
pub struct LizardResults {
    pub vehicle_positions: FeedMessage,
    pub trip_updates: FeedMessage,
}

/// will return a list of all vehicles inside this region with their last seen position
#[utoipa::path(
    get,
    path = "/v1/gtfs/{region}/",
    params(
        ("region" = i64, Path, description = "Identitier of the region")
    ),
    responses(
        (status = 200, description = "gtfs information for the specified region"),
        (status = 500, description = "postgres pool error"),
    ),
)]
#[get("/gtfs/{region}")]
pub async fn gtfs_live_data(
    _req: HttpRequest,
    path: web::Path<(i64,)>,
    redis_pool: web::Data<Client>,
) -> Result<web::Json<LizardResults>, ServerError> {
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

    let waypoints: Vec<Waypoint> = match serde_json::from_str(&waypoint_string) {
        Ok(value) => value,
        Err(e) => {
            error!(
                "cannot deserialize value from redis with error {:?} and value {}",
                e, &waypoint_string
            );
            return Err(ServerError::InternalError);
        }
    };

    let feed_entities = waypoints.into_iter().map(|waypoint| {
        let vehicle_number = waypoint.line * 1000 + waypoint.run;

        let position_entity = FeedEntity {
            id: vehicle_number.to_string(),
            stop: None,
            trip_modifications: None,
            is_deleted: None,
            trip_update: None,
            vehicle: Some(gtfs_realtime::VehiclePosition {
                trip: Some(gtfs_realtime::TripDescriptor {
                    modified_trip: None,
                    trip_id: Some(vehicle_number.to_string()),
                    route_id: Some(waypoint.line.to_string()),
                    direction_id: None,
                    start_time: None,
                    start_date: None,
                    schedule_relationship: None,
                }),
                vehicle: None,
                position: Some(gtfs_realtime::Position {
                    latitude: waypoint.lat as f32,
                    longitude: waypoint.lon as f32,
                    bearing: None,
                    odometer: None,
                    speed: None,
                }),
                current_status: None,
                current_stop_sequence: None,
                stop_id: None,
                timestamp: Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs(),
                ),
                congestion_level: None,
                occupancy_percentage: None,
                occupancy_status: None,
                multi_carriage_details: vec![],
            }),
            alert: None,
            shape: None,
        };

        let trip_entity: FeedEntity = FeedEntity {
            id: vehicle_number.to_string(),
            vehicle: None,
            alert: None,
            shape: None,
            stop: None,
            trip_modifications: None,
            is_deleted: None,
            trip_update: Some(gtfs_realtime::TripUpdate {
                trip: gtfs_realtime::TripDescriptor {
                    modified_trip: None,
                    trip_id: Some(vehicle_number.to_string()),
                    route_id: Some(waypoint.line.to_string()),
                    direction_id: None,
                    start_time: None,
                    start_date: None,
                    schedule_relationship: None,
                },
                vehicle: Some(gtfs_realtime::VehicleDescriptor {
                    id: Some(vehicle_number.to_string()),
                    label: None,
                    license_plate: None,
                    wheelchair_accessible: None,
                }),
                stop_time_update: Vec::new(),
                timestamp: Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs(),
                ),
                delay: waypoint.delayed.map(|x| x as i32),
                trip_properties: None,
            }),
        };

        (position_entity, trip_entity)
    });

    let (vehicle_positions, trip_updates) = feed_entities.unzip();

    Ok(web::Json(LizardResults {
        vehicle_positions: gtfs_realtime::FeedMessage {
            entity: vehicle_positions,
            header: gtfs_realtime::FeedHeader {
                timestamp: Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs(),
                ),
                gtfs_realtime_version: String::from("2.0"),
                incrementality: None,
            },
        },
        trip_updates: gtfs_realtime::FeedMessage {
            entity: trip_updates,
            header: gtfs_realtime::FeedHeader {
                timestamp: Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs(),
                ),
                gtfs_realtime_version: String::from("2.0"),
                incrementality: None,
            },
        },
    }))
}

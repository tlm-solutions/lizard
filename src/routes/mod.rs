pub mod gtfs;
pub mod vehicles;

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

const DEFAULT_OFFSET: i64 = 0;
const DEFAULT_LIMIT: i64 = i64::MAX;

/// let the user specify offset and limit for querying the database
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ListRequest {
    pub offset: i64,
    pub limit: i64,
}

/// returns the user how many entries were found
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ListResponse<T> {
    pub count: i64,
    pub elements: Vec<T>,
}

/// Stats about the regions
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct Stats {
    pub telegram_count: i64,
    pub last_day_receive_rate: f32,
    pub last_month_receive_rate: f32,
}

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,
}

impl Default for ListRequest {
    fn default() -> Self {
        ListRequest {
            offset: DEFAULT_OFFSET,
            limit: DEFAULT_LIMIT,
        }
    }
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::BadClientData => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeactivateRequest {
    pub id: Uuid,
    pub deactivated: bool,
}

#[derive(OpenApi)]
#[openapi(paths(vehicles::vehicles_list), components(schemas()))]
pub struct ApiDoc;

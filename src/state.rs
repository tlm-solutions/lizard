
use tlms::grpc::GrpcGpsPoint;
use std::collections::HashMap;
use chrono::{Duration, NaiveDateTime};

const HISTORY_SIZE_TIME : Duration = Duration::mins(60);

struct History {
    pub last_update: NaiveDateTime,
    pub history: Vec<GrpcGpsPoint>
}


struct State {
    regions: HashMap<i64, RegionWayPoints>


}

struct RegionWayPoints {
    points: HashMap<(i32, i32), History>
}


impl History {
    pub fn update(&mut self, waypoint: GrpcGpsPoint) {
        self.last_update = NaiveDateTime::from_timestamp(0, waypoint.time());

    }

}

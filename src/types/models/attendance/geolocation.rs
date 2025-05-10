use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct GeoLocation {
    pub lat: f64,
    pub long: f64,
}

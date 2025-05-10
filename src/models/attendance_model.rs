use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::models::attendance::{
    attendance_method::AttendanceMethod, attendance_status::AttendanceStatus,
    attendance_type::AttendanceType, geolocation::GeoLocation,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attendance {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub user_id: ObjectId,
    pub organization_id: ObjectId,

    pub attendance_type: AttendanceType,
    pub status: AttendanceStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_in: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_out: Option<DateTime<Utc>>,

    pub method: AttendanceMethod,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<GeoLocation>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

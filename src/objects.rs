use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Equipment {
    pub db_id: Uuid,
    pub db_rev: Uuid,
    pub id: String,
    pub info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Sensor {
    pub db_id: Uuid,
    pub db_rev: Uuid,
    pub id: String,
    pub info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ParameterType {
    pub db_id: Uuid,
    pub db_rev: Uuid,
    pub id: String,
    pub unit: String,
    pub info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Measurement {
    pub db_id: Uuid,
    pub db_rev: Uuid,
    pub ts: DateTime<FixedOffset>,
    pub equipment_db_id: Uuid,
    pub index: i64,
    pub rssi: f64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Parameter {
    pub db_id: Uuid,
    pub db_rev: Uuid,
    pub measurement_db_id: Uuid,
    pub parameter_type_db_id: Uuid,
    pub sensor_db_id: Uuid,
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MeasurementData {
    pub measurement_db_id: Uuid,
    pub ts: DateTime<FixedOffset>,
    pub index: i64,
    pub rssi: f64,
    pub equipment_db_id: Uuid,
    pub parameter_db_id: Uuid,
    pub parameter_type_db_id: Uuid,
    pub sensor_db_id: Uuid,
    pub value: f64,
}

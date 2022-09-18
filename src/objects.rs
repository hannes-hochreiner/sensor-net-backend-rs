use chrono::{DateTime, Utc};
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
    pub ts: DateTime<Utc>,
    pub equipment_id: Uuid,
    pub index: i64,
    pub rssi: f64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Parameter {
    pub db_id: Uuid,
    pub db_rev: Uuid,
    pub measurement_id: Uuid,
    pub parameter_type_id: Uuid,
    pub sensor_id: Uuid,
    pub value: f64,
}

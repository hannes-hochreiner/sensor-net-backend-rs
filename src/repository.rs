use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::objects::{Equipment, Measurement, Parameter, ParameterType, Sensor};
use crate::sensor_net_backend_error::SensorNetBackendError;

#[derive(Debug, Clone)]
pub struct Repository {
    pool: Pool<Postgres>,
}

impl Repository {
    pub async fn new(config: String) -> Result<Self, SensorNetBackendError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&config)
            .await?;

        Ok(Repository { pool })
    }

    pub async fn get_all_sensors(&self) -> Result<Vec<Sensor>, SensorNetBackendError> {
        sqlx::query_as("SELECT * FROM sensors")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_all_equipment(&self) -> Result<Vec<Equipment>, SensorNetBackendError> {
        sqlx::query_as("SELECT * FROM equipment")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_all_parameter_types(
        &self,
    ) -> Result<Vec<ParameterType>, SensorNetBackendError> {
        sqlx::query_as("SELECT * FROM parameter_types")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_all_measurements(&self) -> Result<Vec<Measurement>, SensorNetBackendError> {
        sqlx::query_as("SELECT * FROM measurements")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_all_parameters(&self) -> Result<Vec<Parameter>, SensorNetBackendError> {
        sqlx::query_as("SELECT * FROM parameters")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }
}

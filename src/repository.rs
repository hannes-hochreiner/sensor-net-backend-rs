use chrono::{DateTime, FixedOffset};
use sqlx::Transaction;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

use crate::objects::{Equipment, Measurement, MeasurementData, Parameter, ParameterType, Sensor};
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

    pub async fn get_parameter_values(
        &self,
        start_time: &DateTime<FixedOffset>,
        end_time: &DateTime<FixedOffset>,
        equipment_db_id: &Uuid,
        sensor_db_id: &Uuid,
        parameter_type_db_id: &Uuid,
    ) -> Result<Vec<(DateTime<FixedOffset>, f64)>, SensorNetBackendError> {
        sqlx::query_as("SELECT m.ts, p.value as value FROM measurements m LEFT JOIN parameters p ON m.db_id = p.measurement_db_id WHERE m.ts >= $1 AND m.ts < $2 AND m.equipment_db_id = $3 AND p.sensor_db_id = $4 AND p.parameter_type_db_id = $5 ORDER BY m.ts ASC")
            .bind(start_time)
            .bind(end_time)
            .bind(equipment_db_id)
            .bind(sensor_db_id)
            .bind(parameter_type_db_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_measurement_data_by_start_end_time(
        &self,
        start_time: &DateTime<FixedOffset>,
        end_time: &DateTime<FixedOffset>,
    ) -> Result<Vec<MeasurementData>, SensorNetBackendError> {
        sqlx::query_as("SELECT m.db_id as measurement_db_id, m.ts, m.index, m.rssi, m.equipment_db_id, p.db_id as parameter_db_id, p.parameter_type_db_id, p.sensor_db_id, p.value FROM measurements m LEFT JOIN parameters p ON m.db_id = p.measurement_db_id WHERE m.ts >= $1 AND m.ts < $2")
            .bind(start_time)
            .bind(end_time)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_measurement_data_latest(
        &self,
    ) -> Result<Vec<MeasurementData>, SensorNetBackendError> {
        sqlx::query_as("SELECT m.db_id as measurement_db_id, m.ts, m.index, m.rssi, m.equipment_db_id, p.db_id as parameter_db_id, p.parameter_type_db_id, p.sensor_db_id, p.value FROM measurements m RIGHT JOIN (SELECT max(ts) as max_ts, equipment_db_id FROM measurements GROUP BY equipment_db_id) max_m ON m.equipment_db_id = max_m.equipment_db_id AND m.ts = max_m.max_ts LEFT JOIN parameters p ON m.db_id = p.measurement_db_id")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
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

    async fn select_equipment_by_id(
        &self,
        id: &str,
        trans: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Equipment>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM equipment WHERE id = $1")
            .bind(id)
            .fetch_optional(trans)
            .await
    }

    async fn insert_equipment(
        &self,
        equipment: &Equipment,
        trans: &mut Transaction<'_, Postgres>,
    ) -> Result<Equipment, sqlx::Error> {
        sqlx::query_as(
            "INSERT INTO equipment (db_id, db_rev, id, info) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(equipment.db_id)
        .bind(equipment.db_rev)
        .bind(equipment.id.clone())
        .bind(equipment.info.clone())
        .fetch_one(trans)
        .await
    }

    async fn insert_measurement(
        &self,
        measurement: &Measurement,
        trans: &mut Transaction<'_, Postgres>,
    ) -> Result<Measurement, sqlx::Error> {
        sqlx::query_as(
            "INSERT INTO measurements (db_id, db_rev, ts, equipment_db_id, index, rssi) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(measurement.db_id)
        .bind(measurement.db_rev)
        .bind(measurement.ts)
        .bind(measurement.equipment_db_id)
        .bind(measurement.index)
        .bind(measurement.rssi)
        .fetch_one(trans)
        .await
    }

    async fn get_or_create_measurement(
        &self,
        ts: &DateTime<FixedOffset>,
        equipment_db_id: &Uuid,
        index: i64,
        rssi: f64,
        trans: &mut Transaction<'_, Postgres>,
    ) -> Result<Measurement, sqlx::Error> {
        match self
            .select_measurement_by_ts_equipment_db_id_index(ts, equipment_db_id, index, trans)
            .await?
        {
            Some(meas) => Ok(meas),
            None => Ok(self
                .insert_measurement(
                    &Measurement {
                        db_id: uuid::Uuid::new_v4(),
                        db_rev: uuid::Uuid::new_v4(),
                        ts: *ts,
                        equipment_db_id: *equipment_db_id,
                        index,
                        rssi,
                    },
                    trans,
                )
                .await?),
        }
    }

    async fn get_or_create_equipment(
        &self,
        id: String,
        info: Option<String>,
        trans: &mut Transaction<'_, Postgres>,
    ) -> Result<Equipment, sqlx::Error> {
        match self.select_equipment_by_id(&id, trans).await? {
            Some(equ) => Ok(equ),
            None => Ok(self
                .insert_equipment(
                    &Equipment {
                        db_id: uuid::Uuid::new_v4(),
                        db_rev: uuid::Uuid::new_v4(),
                        id: id,
                        info: info,
                    },
                    trans,
                )
                .await?),
        }
    }

    async fn get_or_create_sensor(
        &self,
        id: String,
        info: Option<String>,
        trans: &mut Transaction<'_, Postgres>,
    ) -> Result<Sensor, sqlx::Error> {
        match self.select_sensor_by_id(&id, trans).await? {
            Some(equ) => Ok(equ),
            None => Ok(self
                .insert_sensor(
                    &Sensor {
                        db_id: uuid::Uuid::new_v4(),
                        db_rev: uuid::Uuid::new_v4(),
                        id: id,
                        info: info,
                    },
                    trans,
                )
                .await?),
        }
    }

    async fn get_or_create_parameter_type(
        &self,
        id: String,
        unit: String,
        info: Option<String>,
        trans: &mut Transaction<'_, Postgres>,
    ) -> Result<ParameterType, sqlx::Error> {
        match self
            .select_parameter_type_by_id_unit(&id, &unit, trans)
            .await?
        {
            Some(equ) => Ok(equ),
            None => Ok(self
                .insert_parameter_type(
                    &ParameterType {
                        db_id: uuid::Uuid::new_v4(),
                        db_rev: uuid::Uuid::new_v4(),
                        id: id,
                        unit: unit,
                        info: info,
                    },
                    trans,
                )
                .await?),
        }
    }

    async fn select_measurement_by_ts_equipment_db_id_index(
        &self,
        ts: &DateTime<chrono::FixedOffset>,
        equipment_db_id: &uuid::Uuid,
        index: i64,
        trans: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Measurement>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM measurements WHERE ts = $1 AND equipment_db_id = $2 AND index = $3",
        )
        .bind(ts)
        .bind(equipment_db_id)
        .bind(index)
        .fetch_optional(trans)
        .await
    }

    async fn select_sensor_by_id(
        &self,
        id: &str,
        trans: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Sensor>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM sensors WHERE id = $1")
            .bind(id)
            .fetch_optional(trans)
            .await
    }

    async fn select_parameter_type_by_id_unit(
        &self,
        id: &str,
        unit: &str,
        trans: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<ParameterType>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM parameter_types WHERE id = $1 AND unit = $2")
            .bind(id)
            .bind(unit)
            .fetch_optional(trans)
            .await
    }

    async fn insert_sensor(
        &self,
        sensor: &Sensor,
        trans: &mut Transaction<'_, Postgres>,
    ) -> Result<Sensor, sqlx::Error> {
        sqlx::query_as(
            "INSERT INTO sensors (db_id, db_rev, id, info) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(sensor.db_id)
        .bind(sensor.db_rev)
        .bind(sensor.id.clone())
        .bind(sensor.info.clone())
        .fetch_one(trans)
        .await
    }

    async fn insert_parameter(
        &self,
        parameter: &Parameter,
        trans: &mut Transaction<'_, Postgres>,
    ) -> Result<Parameter, sqlx::Error> {
        sqlx::query_as(
            "INSERT INTO parameters (db_id, db_rev, measurement_db_id, parameter_type_db_id, sensor_db_id, value) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(parameter.db_id)
        .bind(parameter.db_rev)
        .bind(parameter.measurement_db_id)
        .bind(parameter.parameter_type_db_id)
        .bind(parameter.sensor_db_id)
        .bind(parameter.value)
        .fetch_one(trans)
        .await
    }

    async fn insert_parameter_type(
        &self,
        parameter_type: &ParameterType,
        trans: &mut Transaction<'_, Postgres>,
    ) -> Result<ParameterType, sqlx::Error> {
        sqlx::query_as(
            "INSERT INTO parameter_types (db_id, db_rev, id, unit, info) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(parameter_type.db_id)
        .bind(parameter_type.db_rev)
        .bind(parameter_type.id.clone())
        .bind(parameter_type.unit.clone())
        .bind(parameter_type.info.clone())
        .fetch_one(trans)
        .await
    }

    pub async fn put_message(&self, value: serde_json::Value) -> anyhow::Result<()> {
        let mut trans = self.pool.begin().await?;

        // get index
        let index = value["message"]["index"]
            .as_i64()
            .ok_or(anyhow::anyhow!("error parsing index"))?;

        // get rssi
        let rssi = value["rssi"]
            .as_str()
            .ok_or(anyhow::anyhow!("error parsing rssi string"))?
            .parse::<f64>()?;

        // get timestamp
        let ts = chrono::DateTime::parse_from_rfc3339(
            value["timestamp"]
                .as_str()
                .ok_or(anyhow::anyhow!("error parsing timestamp"))?,
        )?;

        log::debug!(
            "put_message: index: {}, rssi: {}, timestamp: {}",
            index,
            rssi,
            ts
        );

        // get equipment
        let equipment_id = value["message"]["mcuId"]
            .as_str()
            .ok_or(anyhow::anyhow!("error parsing mcuId"))?;
        let equipment = self
            .get_or_create_equipment(String::from(equipment_id), None, &mut trans)
            .await?;

        log::debug!("put_message: equipment: {:?}", equipment);

        // get measurement
        let measurement = self
            .get_or_create_measurement(&ts, &equipment.db_id, index, rssi, &mut trans)
            .await?;

        log::debug!("put_message: measurement: {:?}", measurement);

        for meas in value["message"]["measurements"]
            .as_array()
            .ok_or(anyhow::anyhow!("error parsing measurement array"))?
        {
            let sensor_id = meas["sensorId"]
                .as_str()
                .ok_or(anyhow::anyhow!("error parsing sensorId"))?;
            let sensor = self
                .get_or_create_sensor(String::from(sensor_id), None, &mut trans)
                .await?;

            log::debug!("put_message: sensor: {:?}", sensor);

            let parameters = meas["parameters"]
                .as_object()
                .ok_or(anyhow::anyhow!("error getting parameters"))?;

            for (parameter_name, parameter_value) in parameters {
                let value = parameter_value["value"]
                    .as_f64()
                    .ok_or(anyhow::anyhow!("error parsing parameter value"))?;
                let unit = parameter_value["unit"]
                    .as_str()
                    .ok_or(anyhow::anyhow!("error parsing parameter unit"))?;
                let parameter_type = self
                    .get_or_create_parameter_type(
                        parameter_name.into(),
                        unit.into(),
                        None,
                        &mut trans,
                    )
                    .await?;

                log::debug!("put_message: parameter_type: {:?}", parameter_type);

                let parameter = Parameter {
                    db_id: Uuid::new_v4(),
                    db_rev: Uuid::new_v4(),
                    measurement_db_id: measurement.db_id,
                    parameter_type_db_id: parameter_type.db_id,
                    sensor_db_id: sensor.db_id,
                    value,
                };

                self.insert_parameter(&parameter, &mut trans).await?;

                log::debug!("put_message: inserted parameter: {:?}", parameter);
            }
        }

        trans.commit().await?;
        Ok(())
    }
}

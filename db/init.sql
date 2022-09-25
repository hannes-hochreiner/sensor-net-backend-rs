CREATE TABLE equipment (
  db_id uuid PRIMARY KEY,
  db_rev uuid NOT NULL,
  id varchar(1024) UNIQUE NOT NULL,
  info varchar(1024)
);

CREATE TABLE sensors (
  db_id uuid PRIMARY KEY,
  db_rev uuid NOT NULL,
  id varchar(1024) UNIQUE NOT NULL,
  info varchar(1024)
);

CREATE TABLE parameter_types (
  db_id uuid PRIMARY KEY,
  db_rev uuid NOT NULL,
  id varchar(1024) UNIQUE NOT NULL,
  unit varchar(1024) UNIQUE NOT NULL,
  info varchar(1024)
);

CREATE TABLE measurements (
  db_id uuid PRIMARY KEY,
  db_rev uuid NOT NULL,
  ts timestamp with time zone NOT NULL,
  equipment_db_id uuid REFERENCES equipment (db_id) NOT NULL,
  index BIGINT NOT NULL,
  rssi float NOT NULL
);

CREATE TABLE parameters (
  db_id uuid PRIMARY KEY,
  db_rev uuid NOT NULL,
  measurement_db_id uuid REFERENCES measurements (db_id) NOT NULL,
  parameter_type_db_id uuid REFERENCES parameter_types (db_id) NOT NULL,
  sensor_db_id uuid REFERENCES sensors (db_id) NOT NULL,
  value float NOT NULL
);

CREATE UNIQUE INDEX measurement_unique ON measurements(ts, equipment_db_id, index);
CREATE UNIQUE INDEX parameter_unique ON parameters(measurement_db_id, parameter_type_db_id, sensor_db_id);

CREATE ROLE api_user LOGIN PASSWORD 'api_user';

GRANT SELECT, INSERT, UPDATE ON sensors TO api_user;

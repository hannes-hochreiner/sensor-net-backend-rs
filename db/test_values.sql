INSERT into equipment (db_id, db_rev, id) VALUES ('71c06230-456d-41df-a8bf-91e8ddf89a39', 'f362853b-4a73-4b04-8d01-97e5d80003bb', 'DCBA-4321');
INSERT into equipment (db_id, db_rev, id, info) VALUES ('0c566fa8-9435-46d0-9648-4486e0c25746', '4733d11c-5c52-4966-835c-8d4001868092', 'ABCD-1234', 'living room');
INSERT into sensors (db_id, db_rev, id, info) VALUES ('3dbfdfa1-3f45-428e-99af-4de2535cefc6', 'ef2a68de-4d70-418e-bfc2-10cfc1994e05', '05123-102312', 'Temp, RH%');
INSERT into parameter_types (db_id, db_rev, id, unit, info) VALUES ('3a911abe-8994-42c1-b0e4-24e08a86db70', '9fc55368-dc22-424b-9b50-cb18dcbf9cc3', 'Temperature', '˚C', 'temperature in degrees celcius');
INSERT into parameter_types (db_id, db_rev, id, unit) VALUES ('b2029889-4b6c-445b-b2e1-fd6ea55db5db', 'a7c78f1c-b258-47d4-aaf6-23cbf1f15496', 'Relative Humidity', 'RH%');
INSERT INTO measurements (db_id, db_rev, ts, equipment_db_id, index, rssi) VALUES ('e5650f6f-30db-484a-8320-4139accc2c94', '0745148a-8894-4b37-be38-2e7179339e9a', '2022-09-18T19:42:13+02:00', '0c566fa8-9435-46d0-9648-4486e0c25746', 123, -76.5);
INSERT INTO measurements (db_id, db_rev, ts, equipment_db_id, index, rssi) VALUES ('c73a2937-86a5-4b07-bb0d-5beec58e609b', '3e3dfcea-a520-485c-989c-23554aed9960', '2022-09-18T19:52:13+02:00', '0c566fa8-9435-46d0-9648-4486e0c25746', 123, -86.5);
INSERT INTO measurements (db_id, db_rev, ts, equipment_db_id, index, rssi) VALUES ('36c78f0a-e422-48c7-b37c-51492adde80f', 'fac9344e-fbce-490b-85f9-0391f3463e9e', '2022-09-18T20:02:13+02:00', '0c566fa8-9435-46d0-9648-4486e0c25746', 123, -72.5);
INSERT INTO parameters (db_id, db_rev, measurement_db_id, parameter_type_db_id, sensor_db_id, value) VALUES ('36e04d7c-06b2-464a-951c-624e378b9e4b', 'd4b96c32-241b-4481-87de-1d0debfdd566', 'e5650f6f-30db-484a-8320-4139accc2c94', '3a911abe-8994-42c1-b0e4-24e08a86db70', '3dbfdfa1-3f45-428e-99af-4de2535cefc6', 27.5);
INSERT INTO parameters (db_id, db_rev, measurement_db_id, parameter_type_db_id, sensor_db_id, value) VALUES ('f4290842-6024-4e74-89d3-ec75ad361b72', '975faa19-7be6-4bb7-847e-a55203e62aed', 'c73a2937-86a5-4b07-bb0d-5beec58e609b', '3a911abe-8994-42c1-b0e4-24e08a86db70', '3dbfdfa1-3f45-428e-99af-4de2535cefc6', 23.5);
INSERT INTO parameters (db_id, db_rev, measurement_db_id, parameter_type_db_id, sensor_db_id, value) VALUES ('0d62f8db-a21b-48fa-8bbe-d7eb2db6b869', 'dff8aab8-15d7-4015-9314-ecde9dfc0680', '36c78f0a-e422-48c7-b37c-51492adde80f', '3a911abe-8994-42c1-b0e4-24e08a86db70', '3dbfdfa1-3f45-428e-99af-4de2535cefc6', 25.5);
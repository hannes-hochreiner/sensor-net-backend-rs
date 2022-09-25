[![CI](https://github.com/hannes-hochreiner/sensor-net-backend-rs/actions/workflows/main.yml/badge.svg)](https://github.com/hannes-hochreiner/sensor-net-backend-rs/actions/workflows/main.yml)

# SensorNet Backend

This is a re-write of the SensorNet backend in Rust.

## Environment Variables

| Name | Description | Values | Default Value |
|---|---|---|---|
| RUST_LOG | Logging level | error,warn,info,debug,trace | error |
| HYPER_BIND_ADDRESS | address for the server | \<ip address>:\<port> | 127.0.0.1:8000 |
| DB_CONNECTION | connection string as used by sqlx | n/a | postgres://postgres:password@127.0.0.1:5432 |

## API
For all requests, the response will have the status 200 and, when appropriate, contain a JSON object with the property "result" containing an array of result objects.
In the case of errors, the response will have the status 500 and will contain a JSON object with property "error" containing the error.

### PUT /message
The message is expected as a JSON object in the body.

*Example Message*
```json
{
  "type": "rfm",
  "rssi": "-87",
  "timestamp": "2020-04-18T15:59:56.071Z",
  "message": {
    "mcuId": "005a0000-33373938-17473634",
    "index": 1524,
    "measurements": [
      {
        "sensorId": "be01",
        "parameters": {
          "temperature": { "value": 25.68000030517578, "unit": "°C" },
          "relativeHumidity": { "value": 33.9677734375, "unit": "%" },
          "pressure": { "value": 1001.1699829101562, "unit": "mbar" }
        }
      }
    ]
  }
}
```
*Example Query*
```shell
curl -H 'Content-Type: application/json' -X PUT -d '{"type":"rfm","rssi":"-87","timestamp":"2020-04-18T15:59:56.071Z","message":{"mcuId": "mcuId1","index": 1524,"measurements":[{"sensorId":"be01","parameters":{"temperature":{"value":25.68000030517578,"unit":"°C"},"relativeHumidity":{"value":33.9677734375,"unit":"%"},"pressure":{"value":1001.1699829101562,"unit":"mbar"}}}]}}' localhost:8080/message
```
### GET /measurement_data?startTime=\<startTime as string>&endTime=\<endTime as string>
Start and end times are required.
They are expected to be ISO 8601 string.
*Example Query*
```shell
curl localhost:8080/measurement_data?startTime=2020-04-12T00:00:00.000Z\&endTime=2020-12-12T00:00:00.000Z
```
### GET /equipment
*Example Query*
```shell
curl localhost:8080/equipment
```
### GET /sensor
*Example Query*
```shell
curl localhost:8080/sensor
```
### GET /parameter_type
*Example Query*
```shell
curl localhost:8080/parameter_type
```

## References

* [Making requests with client certificates using Hyper](https://stackoverflow.com/questions/44059266/how-to-make-a-request-with-client-certificate-in-rust)

## License

This work is licensed under the MIT license.

`SPDX-License-Identifier: MIT`
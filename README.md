[![CI](https://github.com/hannes-hochreiner/sensor-net-backend-rs/actions/workflows/main.yml/badge.svg)](https://github.com/hannes-hochreiner/sensor-net-backend-rs/actions/workflows/main.yml)

# SensorNet Backend

This is a re-write of the SensorNet backend in Rust.

## Environment Variables

| Name | Description | Values | Default Value |
|---|---|---|---|
| RUST_LOG | Logging level | error,warn,info,debug,trace | error |
| HYPER_BIND_ADDRESS | address for the server | \<ip address>:\<port> | 127.0.0.1:8000 |
| DB_CONNECTION | connection string as used by sqlx | n/a | postgres://postgres:password@127.0.0.1:5432 |

## References

* [Making requests with client certificates using Hyper](https://stackoverflow.com/questions/44059266/how-to-make-a-request-with-client-certificate-in-rust)

## License

This work is licensed under the MIT license.

`SPDX-License-Identifier: MIT`
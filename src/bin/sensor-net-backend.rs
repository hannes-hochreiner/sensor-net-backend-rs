extern crate sensor_net_backend_rs;
use anyhow::{anyhow, Result};
use chrono::{DateTime, FixedOffset, Timelike};
use hyper::body::Buf;
use hyper::header::CONTENT_TYPE;
use hyper::http::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use plotters::prelude::{ChartBuilder, IntoDrawingArea, SVGBackend};
use plotters::series::LineSeries;
use plotters::style::{Color, ShapeStyle, BLUE, WHITE};
use sensor_net_backend_rs::repository::Repository;
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn service(req: Request<Body>, repo: Repository) -> Result<Response<Body>, Infallible> {
    match router(req, &repo).await {
        Ok(resp) => Ok(resp),
        Err(err) => {
            log::error!("service error: {}", err);

            let mut server_error = Response::default();

            *server_error.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            *server_error.body_mut() = Body::from(format!("Server error: {}", err));
            Ok(server_error)
        }
    }
}

async fn router(req: Request<Body>, repo: &Repository) -> Result<Response<Body>> {
    let path_and_query = req
        .uri()
        .path_and_query()
        .ok_or(anyhow::anyhow!("error parsing path and query"))?;
    let path = path_and_query.path().split("/").collect::<Vec<&str>>();
    let query: HashMap<Cow<str>, Cow<str>> = url::form_urlencoded::parse(
        match path_and_query.query() {
            Some(val) => val,
            None => "",
        }
        .as_bytes(),
    )
    .collect();

    match (req.method(), &path[1..]) {
        (&Method::GET, &["equipment"]) => Ok(Response::new(Body::from(serde_json::to_string(
            &repo.get_all_equipment().await?,
        )?))),
        (&Method::GET, &["sensors"]) => Ok(Response::new(Body::from(serde_json::to_string(
            &repo.get_all_sensors().await?,
        )?))),
        (&Method::GET, &["parameter_types"]) => Ok(Response::new(Body::from(
            serde_json::to_string(&repo.get_all_parameter_types().await?)?,
        ))),
        (&Method::GET, &["measurements"]) => Ok(Response::new(Body::from(serde_json::to_string(
            &repo.get_all_measurements().await?,
        )?))),
        (&Method::GET, &["parameters"]) => Ok(Response::new(Body::from(serde_json::to_string(
            &repo.get_all_parameters().await?,
        )?))),
        (&Method::PUT, &["message"]) => Ok(Response::new(Body::from(serde_json::to_string(
            &repo
                .put_message(serde_json::from_reader(
                    hyper::body::aggregate(req.into_body()).await?.reader(),
                )?)
                .await?,
        )?))),
        (&Method::GET, &["measurement_data"]) => {
            let start_time = chrono::DateTime::parse_from_rfc3339(
                query
                    .get("startTime")
                    .ok_or(anyhow!("start time not found"))?,
            )?;
            let end_time = chrono::DateTime::parse_from_rfc3339(
                query.get("endTime").ok_or(anyhow!("end time not found"))?,
            )?;

            Ok(Response::new(Body::from(serde_json::to_string(
                &repo
                    .get_measurement_data_by_start_end_time(&start_time, &end_time)
                    .await?,
            )?)))
        }
        (&Method::GET, &["measurement_data", "latest"]) => Ok(Response::new(Body::from(
            serde_json::to_string(&repo.get_measurement_data_latest().await?)?,
        ))),
        (&Method::GET, &["plot"]) => {
            log::debug!("plot");
            let start_time = chrono::DateTime::parse_from_rfc3339(
                query
                    .get("startTime")
                    .ok_or(anyhow!("start time not found"))?,
            )?;
            log::debug!("got starttime");
            let end_time = chrono::DateTime::parse_from_rfc3339(
                query.get("endTime").ok_or(anyhow!("end time not found"))?,
            )?;
            let equipment_db_id = uuid::Uuid::parse_str(
                query
                    .get("equipmentDbId")
                    .ok_or(anyhow!("equipment db id not found"))?,
            )?;
            let sensor_db_id = uuid::Uuid::parse_str(
                query
                    .get("sensorDbId")
                    .ok_or(anyhow!("sensor db id not found"))?,
            )?;
            let parameter_type_db_id = uuid::Uuid::parse_str(
                query
                    .get("parameterTypeDbId")
                    .ok_or(anyhow!("parameter type db id not found"))?,
            )?;

            log::debug!("got all parameters");

            let vals = repo
                .get_parameter_values(
                    &start_time,
                    &end_time,
                    &equipment_db_id,
                    &sensor_db_id,
                    &parameter_type_db_id,
                )
                .await?;

            log::debug!("got {} vals", vals.len());

            let mut resp = Response::new(plot_data(&vals)?.into());
            resp.headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("image/svg+xml"));
            Ok(resp)
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::info!("starting...");

    let repo = Repository::new(match env::var("DB_CONNECTION") {
        Ok(s) => s,
        Err(_) => String::from("postgres://postgres:password@127.0.0.1:5432"),
    })
    .await?;

    let addr: SocketAddr = match env::var("HYPER_BIND_ADDRESS") {
        Ok(s) => s,
        Err(_) => String::from("127.0.0.1:8000"),
    }
    .parse()?;

    log::info!("starting server at http://{}", addr);

    // A `Service` is needed for every connection, so this
    // creates one from our `service` function.
    let make_svc = make_service_fn(|_conn| {
        let repo = repo.clone();
        // service_fn converts our function into a `Service`
        async { Ok::<_, Infallible>(service_fn(move |req| service(req, repo.clone()))) }
    });

    let server = Server::bind(&addr).serve(make_svc);
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        log::error!("server error: {}", e);
        return Err(anyhow!(e));
    }

    Ok(())
}

fn plot_data(data: &Vec<(DateTime<FixedOffset>, f64)>) -> anyhow::Result<String> {
    let mut buffer = String::new();
    {
        let original_style = ShapeStyle {
            color: BLUE.mix(1.0),
            filled: false,
            stroke_width: 5,
        };
        let mut min_max_x: (Option<DateTime<FixedOffset>>, Option<DateTime<FixedOffset>>) =
            (None, None);
        let mut min_max_y: (Option<f64>, Option<f64>) = (None, None);

        for datum in data {
            match min_max_x.0 {
                None => min_max_x.0 = Some(datum.0),
                Some(val) => {
                    if val > datum.0 {
                        min_max_x.0 = Some(datum.0)
                    }
                }
            }
            match min_max_x.1 {
                None => min_max_x.1 = Some(datum.0),
                Some(val) => {
                    if val < datum.0 {
                        min_max_x.1 = Some(datum.0)
                    }
                }
            }
            match min_max_y.0 {
                None => min_max_y.0 = Some(datum.1),
                Some(val) => {
                    if val > datum.1 {
                        min_max_y.0 = Some(datum.1)
                    }
                }
            }
            match min_max_y.1 {
                None => min_max_y.1 = Some(datum.1),
                Some(val) => {
                    if val < datum.1 {
                        min_max_y.1 = Some(datum.1)
                    }
                }
            }
        }

        let root = SVGBackend::with_string(&mut buffer, (640, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(
                min_max_x.0.ok_or(anyhow::anyhow!("no x min found"))?
                    ..min_max_x.1.ok_or(anyhow::anyhow!("no x max found"))?,
                min_max_y.0.ok_or(anyhow::anyhow!("no y min found"))?
                    ..min_max_y.1.ok_or(anyhow::anyhow!("no y max found"))?,
            )
            .unwrap();

        chart
            .configure_mesh()
            .x_label_style(("sans-serif", 20))
            .y_label_style(("sans-serif", 20))
            .x_labels(5)
            .x_label_formatter(&datetime_formatter)
            .draw()
            .unwrap();
        chart
            .draw_series(LineSeries::new(data.clone(), original_style))
            .unwrap();
    }

    Ok(buffer)
}

fn datetime_formatter(datetime: &DateTime<FixedOffset>) -> String {
    format!("{:02}:{:02}", datetime.hour(), datetime.minute())
}

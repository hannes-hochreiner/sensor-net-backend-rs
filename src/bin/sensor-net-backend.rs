extern crate sensor_net_backend_rs;
use anyhow::{anyhow, Result};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use log::info;
use sensor_net_backend_rs::repository::Repository;
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
            let mut server_error = Response::default();
            *server_error.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            *server_error.body_mut() = Body::from(format!("Server error: {}", err));
            Ok(server_error)
        }
    }
}

async fn router(req: Request<Body>, repo: &Repository) -> Result<Response<Body>> {
    let path = req.uri().path().split("/").collect::<Vec<&str>>();

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

    info!("starting server at http://{}", addr);

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
        eprintln!("server error: {}", e);
        return Err(anyhow!(e));
    }

    Ok(())
}

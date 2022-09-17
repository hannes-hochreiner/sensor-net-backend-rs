use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use log::info;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let headers: String = req
        .headers()
        .into_iter()
        .map(|(name, value)| {
            String::from(format!(
                "{}: {}\n",
                name.as_str(),
                String::from_utf8_lossy(value.as_bytes())
            ))
        })
        .collect();
    Ok(Response::new(format!("Hello, World\n\n{}", headers).into()))
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr: SocketAddr = match env::var("HYPER_BIND_ADDRESS") {
        Ok(s) => s,
        Err(_) => String::from("127.0.0.1:8000"),
    }
    .parse()
    .unwrap();

    info!("starting server at {}", addr);

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use log::info;
use std::convert::Infallible;
use std::net::SocketAddr;

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

    info!("starting server at 0.0.0.0:3000");
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

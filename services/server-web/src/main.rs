use axum::{routing::get, Router};
use shuttle_runtime::__internals::Context;
use shuttle_runtime::SecretStore;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let secret = secrets.get("MYSQL_URL").context("secret was not found")?;
    println!("config:{:?}",secret);

    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}

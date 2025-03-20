use server_web::init_router;
use shuttle_runtime::SecretStore;
use shuttle_runtime::__internals::Context;

//sea-orm-cli generate entity -u mysql://root:Lsw%400516@47.95.179.146:3306/fish -o creates/lib-entity/src/mysql --with-serde both

#[shuttle_runtime::main]
async fn axum(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let secret = secrets.get("MYSQL_URL").context("secret was not found")?;
    println!("config:{:?}", secret);

    let router = init_router(&secret).await?;

    Ok(router.into())
}

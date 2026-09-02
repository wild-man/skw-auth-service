use skw_lib_shared::{
    prelude::{
        axum::*,
        consts::{INTERNAL_URL_PREFIX, SERVICE_BIND_ADDRESS, SERVICE_NAME_PATH},
        log::*,
        postgres::*,
    },
    *,
};

use tokio::net::TcpListener;

mod internal;
mod models;

#[derive(Clone)]
struct AppState {
    db_pools: PostgresPools<PostgresConnected>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service_name = APP.config.expect_string(SERVICE_NAME_PATH);
    let bind_addr = APP.config.expect_string(SERVICE_BIND_ADDRESS);

    let db_pools = APP.postgres.connect().await?;

    let state = AppState { db_pools };

    // let router = Router::new()
    //     .route("/register", post(register))
    //     .route("/check_signature", post(check_signature))
    //     .with_state(state);

    let router = Router::new()
        .route(INTERNAL_URL_PREFIX, post(internal::http_handler))
        .with_state(state);

    info!("{} listening on http://{}", service_name, bind_addr);
    let listener = TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

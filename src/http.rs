use skw_lib_shared::{
    prelude::{
        axum::*,
        consts::{INTERNAL_URL_PREFIX, SERVICE_BIND_ADDRESS, SERVICE_NAME_PATH},
        log::*,
        postgres::*,
        uuid::*,
    },
    *,
};

use tokio::net::TcpListener;

mod external;
mod internal;
mod key;
mod models;

const SERVICE_ROOT_KEY_PATH: &str = "/service/rootKey";

#[derive(Clone)]
struct AppState {
    db_pools: PostgresPools<PostgresConnected>,
    root_entity_uuid: Uuid,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service_name = APP.config.expect_string(SERVICE_NAME_PATH);
    let bind_addr = APP.config.expect_string(SERVICE_BIND_ADDRESS);
    let root_entity_uuid = Uuid::parse_str(APP.config.expect_str(SERVICE_ROOT_KEY_PATH)).expect("service.rootKey must be a valid uuid");

    let db_pools = APP.postgres.connect().await?;

    let state = AppState {
        db_pools,
        root_entity_uuid,
    };

    let router = Router::new()
        .route(INTERNAL_URL_PREFIX, post(internal::http_handler))
        .route("/", post(external::http_handler))
        .with_state(state);

    info!("{} listening on http://{}", service_name, bind_addr);
    let listener = TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

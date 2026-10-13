use skw_lib_http_protos::{
    JsonRpcServiceRequest,
    auth::{RpcMethod, RpcPayload},
};
use skw_lib_shared::{
    AppError,
    prelude::postgres::{PostgresConnected, PostgresPools},
};

#[allow(unused)]
use skw_lib_shared::prelude::{
    APP,
    chrono::*,
    iggy::{RpcRequestMeta, run_service_consumers_all_topic_types},
    jsonrpc::ServiceHttpError,
    log::*,
    redis::{AsyncCommands, ConnectionManager},
    serde::*,
    uuid::*,
};

mod key;

#[allow(dead_code)]
mod models;

const SERVICE_ROOT_KEY_PATH: &str = "/service/rootKey";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let root_entity_uuid = Uuid::parse_str(APP.config.expect_str(SERVICE_ROOT_KEY_PATH)).expect("service.rootKey must be a valid uuid");
    let db_pools = APP.postgres.connect().await?;

    run_service_consumers_all_topic_types::<JsonRpcServiceRequest<RpcMethod>, _, _, _>(
        move |method, meta| handle_message(method, meta, db_pools.clone(), root_entity_uuid),
        None,
        Some(100), //@todo maybe need to set per topic_type
    )
    .await?;

    Ok(())
}

async fn handle_message(
    method: RpcMethod,
    _meta: RpcRequestMeta,
    db_pools: PostgresPools<PostgresConnected>,
    root_entity_uuid: Uuid,
) -> Result<RpcPayload, AppError> {
    debug!("{:?}", method);
    match method {
        RpcMethod::CreateKey { store_private_key } => {
            let pool = db_pools.master().await?;

            let k = key::create(pool, root_entity_uuid, store_private_key).await?;

            Ok(RpcPayload::CreateKey {
                id: k.id,
                public_key: k.public_key,
                private_key: k.private_key,
                enabled: k.enabled,
            })
        }
    }
}

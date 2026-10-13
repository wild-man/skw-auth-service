use crate::AppState;
use crate::key;
use skw_lib_http_protos::auth::RpcMethod;
use skw_lib_http_protos::auth::RpcPayload;
use skw_lib_shared::prelude::{AppError, axum::*, jsonrpc::*, serde::*};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct HttpRequest<M> {
    #[serde(flatten)]
    pub method: M,
}

// #[axum::debug_handler]
pub async fn http_handler(
    State(state): State<AppState>,
    _headers: HeaderMap,
    AxumJson(req): AxumJson<HttpRequest<RpcMethod>>,
) -> impl IntoAxumResponse {
    match req.method {
        RpcMethod::CreateKey { store_private_key } => match create_key(&state, store_private_key).await {
            Ok(p) => JsonRpcResponse::new(req, p).into_response(),
            Err(e) => JsonRpcErrorResponse::new(req, e).into_response(),
        },
    }
}

async fn create_key(state: &AppState, store_private_key: bool) -> Result<RpcPayload, AppError> {
    let pool = state.db_pools.master().await?;
    let k = key::create(pool, state.root_entity_uuid, store_private_key).await?;

    Ok(RpcPayload::CreateKey {
        id: k.id,
        public_key: k.public_key,
        private_key: k.private_key,
        enabled: k.enabled,
    })
}

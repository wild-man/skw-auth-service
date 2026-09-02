use crate::AppState;
use crate::models::entities::Entity;
use skw_lib_http_protos::auth::internal::*;
use skw_lib_shared::prelude::{AppError, axum::*, jsonrpc::*, serde::*, uuid::*};

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
    AxumJson(req): AxumJson<HttpRequest<HttpMethod>>,
) -> impl IntoAxumResponse {
    match req.method {
        HttpMethod::GetKeyById { id } => match internal_get_key(&state, id).await {
            Ok(p) => JsonRpcResponse::new(req, p).into_response(),
            Err(e) => JsonRpcErrorResponse::new(req, e).into_response(),
        },
    }
}

async fn internal_get_key(state: &AppState, key_uuid: Uuid) -> Result<HttpPayload, AppError> {
    let pool = state.db_pools.slave().await?;
    dbg!(&key_uuid);

    let e = Entity::get_by_id(pool, key_uuid)
        .await?
        .ok_or_else(|| ServiceHttpError::NotFound)?;

    Ok(HttpPayload::GetKeyById {
        id: e.id,
        public_key: e.public_key,
        enabled: e.enabled,
    })
}

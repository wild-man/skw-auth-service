//! Shared `CreateKey` logic used by both `auth-http-service` (axum) and
//! `auth-stream-service` (Iggy) binaries.

use crate::models::entities::Entity;
use ed25519_dalek::SigningKey;
use ed25519_dalek::pkcs8::{EncodePrivateKey, EncodePublicKey};
use rand::rngs::OsRng;
use skw_lib_shared::prelude::{AppError, base64::*, uuid::*};
use sqlx::PgPool;

pub struct CreatedKey {
    pub id: Uuid,
    pub public_key: String,
    pub private_key: String,
    pub enabled: bool,
}

pub async fn create(pool: &PgPool, root_id: Uuid, store_private_key: bool) -> Result<CreatedKey, AppError> {
    let signing_key = SigningKey::generate(&mut OsRng);
    let public_key_der = signing_key
        .verifying_key()
        .to_public_key_der()
        .map_err(|e| AppError::Custom(e.to_string()))?;

    let private_key_der = signing_key
        .to_pkcs8_der()
        .map_err(|e| AppError::Custom(e.to_string()))?;

    let public_key = BASE64_STANDARD.encode(public_key_der.as_bytes());
    let private_key = BASE64_STANDARD.encode(private_key_der.as_bytes());

    let stored_private_key = store_private_key.then(|| private_key.clone());
    let e = Entity::insert(
        pool,
        root_id,
        public_key,
        stored_private_key,
        true,
        vec![],
        root_id,
    )
    .await?;

    Ok(CreatedKey {
        id: e.id,
        public_key: e.public_key,
        private_key,
        enabled: e.enabled,
    })
}

//! `entities` table model (schema `skw-auth-service`).
//!
//! Entities form a self-parented tree: the root's `parent_id` equals its own
//! `id`. Public/private keys are stored as the bare base64 body of the SPKI /
//! PKCS#8 PEM (armor stripped); `private_key` is only present when the caller
//! opted to have it stored.

use skw_lib_shared::prelude::postgres::PostgresError;
use skw_lib_shared::prelude::serde::*;
use skw_lib_shared::prelude::uuid::*;
use sqlx::PgPool;

/// A row of the `entities` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Primary key (`uuidv7()` on insert).
    pub id: Uuid,
    /// Parent in the entity tree; the root points at itself.
    pub parent_id: Uuid,
    /// ed25519 public key, base64 SPKI body (no PEM armor).
    pub public_key: String,
    /// ed25519 private key, base64 PKCS#8 body; `None` when not stored.
    pub private_key: Option<String>,
    /// Whether the entity may authenticate.
    pub enabled: bool,
    /// Role ids granted to this entity (`roles.id`).
    pub roles: Vec<i64>,
    /// Entity that created this row.
    pub created_id: Uuid,
}

/// Column list matching the `EntityRow` tuple decode order below.
const ENTITY_COLUMNS: &str = "id, parent_id, public_key, private_key, enabled, roles, created_id";

type EntityRow = (Uuid, Uuid, String, Option<String>, bool, Vec<i64>, Uuid);

impl From<EntityRow> for Entity {
    fn from((id, parent_id, public_key, private_key, enabled, roles, created_id): EntityRow) -> Self {
        Entity {
            id,
            parent_id,
            public_key,
            private_key,
            enabled,
            roles,
            created_id,
        }
    }
}

impl Entity {
    /// Fetch a single entity by its `id`, or `None` if no such row exists.
    ///
    /// The `search_path` is pinned to the service schema at connection time, so
    /// the unqualified `entities` reference resolves to `skw-auth-service.entities`.
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Entity>, PostgresError> {
        let row: Option<EntityRow> = sqlx::query_as(&format!(
            "SELECT {ENTITY_COLUMNS} FROM entities WHERE id = $1"
        ))
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(Entity::from))
    }

    /// Insert a new entity row, letting Postgres generate `id` (`DEFAULT uuidv7()`).
    pub async fn insert(pool: &PgPool, parent_id: Uuid, public_key: String, private_key: Option<String>, enabled: bool, roles: Vec<i64>, created_id: Uuid) -> Result<Entity, PostgresError> {
        let row: EntityRow = sqlx::query_as(&format!(
            "INSERT INTO entities (parent_id, public_key, private_key, enabled, roles, created_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING {ENTITY_COLUMNS}"
        ))
        .bind(parent_id)
        .bind(public_key)
        .bind(private_key)
        .bind(enabled)
        .bind(roles)
        .bind(created_id)
        .fetch_one(pool)
        .await?;

        Ok(Entity::from(row))
    }
}

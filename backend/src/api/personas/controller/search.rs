use axum::extract::Query;
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::DbBackend;
use sea_orm::FromQueryResult;
use sea_orm::Statement;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::utils::internal_error;
use crate::database::pool::Pool;

use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct SearchArg {
    pub query: String,
}

pub async fn search_persona(
    State(pool): State<Arc<Pool>>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Value>>, (StatusCode, String)> {
    // Generate querie
    if let Some(query) = query.get("query".into()) {
        let wildcard = format!("%{}%", query);
        let querie = sea_orm::query::JsonValue::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            SELECT *
            FROM personas
            WHERE unaccent(nombre || ' ' || apellido_1 || ' ' || apellido_2) ILIKE '%' || unaccent($1) || '%'
               OR unaccent(rut) ILIKE '%' || unaccent($1) || '%';
            "#,
            [wildcard.into()],
        ))
        .all(pool.get_db())
        .await
        .map_err(internal_error)?;
        Ok(querie.into())
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            "Missing \"query\" parameter".into(),
        ))
    }
}

use crate::database::provider::{self, Entity as Provider};
use axum::{
    extract::{Path, Extension, Query},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, EntityTrait, IntoActiveModel};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    soft: bool,
}

pub async fn delete_provider(
    Path(provider_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<QueryParams>
) -> Result<(), StatusCode> {
    let mut provider = if let Some(provider) = Provider::find_by_id(provider_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
        {
            provider.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

    if query_params.soft {
        let now = chrono::Utc::now().naive_local();
        provider.deleted_at = Set(Some(now.into()));
        Provider::update(provider)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        Provider::delete(provider)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Provider::delete_many()
    //     .filter(provider::Column:ProviderId.eq(provider_id))
    //     .exec(&database)
    //     .await
    //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
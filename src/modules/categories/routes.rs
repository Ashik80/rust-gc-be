use axum::{extract::{Path, State}, routing::get, Json, Router};
use crate::errors::Error;
use crate::{models::category::Category, AppState};

pub fn category_routes() -> Router<AppState> {
    Router::new()
        .route("/categories", get(get_categories))
        .route("/categories/:id", get(get_category))
}

async fn get_categories(State(state): State<AppState>) -> Result<Json<Vec<Category>>, Error> {
    let categories = Category::get_all(state).await?;
    Ok(Json(categories))
}

async fn get_category(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Category>, Error> {
    let category = Category::get_by_id(&state, id).await?;
    Ok(Json(category))
}

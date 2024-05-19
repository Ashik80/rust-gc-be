use axum::{extract::{Path, State}, routing::{get, post}, Json, Router};
use crate::{errors::Error, mapper::product::ProductMapper, models::{category::Category, product::{Product, ProductCreateDto, ProductWithCategories}, product_categories::ProductCategory}, AppState};

pub fn product_routes() -> Router<AppState> {
    Router::new()
        .route("/products", get(get_products))
        .route("/products/:id", get(get_product))
        .route("/products", post(create_product))
}

async fn get_products(State(state): State<AppState>) -> Result<Json<Vec<ProductWithCategories>>, Error> {
    let response = Product::get_all(&state).await?;
    Ok(Json(response))
}

async fn get_product(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Product>, Error> {
    let response = Product::get_by_id(&state, id).await?;
    Ok(Json(response))
}

async fn create_product(
    State(state): State<AppState>, Json(product_dto): Json<ProductCreateDto>
) -> Result<Json<ProductWithCategories>, Error> {
    let product = Product::create(&state, &product_dto).await?;

    let product_categories = get_product_categories(&state, product.id, &product_dto).await?;

    ProductCategory::batch_create(&state, product_categories.0, product_categories.1.clone()).await?;

    let product = product.map_to_product_with_categories(&product_categories.1);

    Ok(Json(product))
}

async fn get_product_categories(
    state: &AppState, product_id: i32, product_dto: &ProductCreateDto
) -> Result<(Vec<i32>, Vec<i32>), Error> {
    let mut category_ids = Vec::new();
    let mut product_ids = Vec::new();

    for category_id in product_dto.categories.clone() {
        let category = Category::get_by_id(state, category_id).await?;
        category_ids.push(category.id);
        product_ids.push(product_id);
    }

    Ok((product_ids, category_ids))
}

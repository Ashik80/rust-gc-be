use axum::{extract::{Multipart, Path, State}, routing::post, Json, Router};
use crate::{errors::Error, models::product_image::ProductImage, AppState};

pub fn product_image_routes() -> Router<AppState> {
    Router::new().route("/product/:id/images", post(upload_images))
}

async fn upload_images(
    State(state): State<AppState>, Path(id): Path<i32>, mut multipart: Multipart
) -> Result<Json<Vec<ProductImage>>, Error> {
    let image_urls = ProductImage::save(&mut multipart).await?;

    let mut product_images = Vec::new();

    for image_url in image_urls {
        let product_image = ProductImage::save_to_db(&state, image_url, id).await?;
        product_images.push(product_image);
    }

    Ok(Json(product_images))
}

use std::{fs::File, io::Write};
use axum::extract::Multipart;
use serde::{Deserialize, Serialize};
use crate::{errors::{Error, FileUploadError, GeneralError}, AppState};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ProductImage {
    pub id: i32,
    pub image_url: String,
    pub product_id: i32,
}

struct ReturnId {
    id: i32
}

impl ProductImage {
    pub async fn save(multipart: &mut Multipart) -> Result<Vec<String>, Error> {
        let mut image_urls = Vec::new();

        while let Some(field) = multipart.next_field()
            .await
            .map_err(|_| Error::GeneralError(GeneralError::Unhandled("Failed to get next field".to_owned())))? {
            let name = field.name()
                .ok_or(Error::FileUploadError(FileUploadError::MultipartParseFailed))?
                .to_string();

            if name != "images" {
                continue;
            }

            let data = field.bytes()
                .await
                .map_err(|_| Error::FileUploadError(FileUploadError::MultipartParseFailed))?;

            let uuid = Uuid::new_v4();
            let filepath = format!("uploads/{}", uuid);

            let mut file_handle = File::create(&filepath)
                .map_err(|_| Error::FileUploadError(FileUploadError::FileNotCreated))?;

            file_handle.write(&data).map_err(|_| Error::FileUploadError(FileUploadError::FileNotWritten))?;

            image_urls.push(filepath);
        }

        Ok(image_urls)
    }

    pub async fn save_to_db(state: &AppState, image_url: String, product_id: i32) -> Result<ProductImage, Error> {
        let result = sqlx::query_as!(
            ReturnId,
            "INSERT into product_images (image_url, product_id) VALUES ($1, $2) RETURNING id",
            image_url, product_id
        ).fetch_one(&state.pool).await.map_err(|_| Error::GeneralError(GeneralError::BadRequest))?;

        Ok(ProductImage {
            id: result.id,
            image_url,
            product_id
        })
    }
}

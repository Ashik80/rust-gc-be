use sqlx::postgres::PgQueryResult;
use crate::{errors::{Error, GeneralError}, AppState};

#[derive(Debug)]
pub struct ProductCategory {
    pub product_id: i32,
    pub category_id: i32,
}

impl ProductCategory {
    pub async fn batch_create(
        state: &AppState, product_ids: Vec<i32>, category_ids: Vec<i32>
    ) -> Result<PgQueryResult, Error> {
        let result = sqlx::query_as!(
            ProductCategory,
            "INSERT INTO product_categories (product_id, category_id) SELECT * FROM UNNEST ($1::int[], $2::int[])",
            &product_ids, &category_ids
        )
            .execute(&state.pool)
            .await
            .map_err(|_| Error::GeneralError(
                GeneralError::Unhandled("Insertion failed for product categories".to_owned())
            ))?;

        Ok(result)
    }
}

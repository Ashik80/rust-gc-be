use serde::Serialize;
use crate::{errors::{Error, GeneralError}, AppState};

#[derive(Debug, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub parent_category_id: i32,
}

impl Category {
    pub async fn get_all(state: AppState) -> Result<Vec<Category>, Error> {
        let categories = sqlx::query_as!(Category, "SELECT * FROM categories")
            .fetch_all(&state.pool)
            .await
            .map_err(|_| Error::GeneralError(
                GeneralError::Unhandled("Query failed for getting categories".to_owned())
            ))?;
        Ok(categories)
    }

    pub async fn get_by_id(state: &AppState, id: i32) -> Result<Category, Error> {
        let category = sqlx::query_as!(Category, "SELECT * FROM categories where id = $1", id)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| Error::GeneralError(GeneralError::NotFound))?;
        Ok(category)
    }
}

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use crate::{errors::{Error, GeneralError}, mapper::product::ProductMapper, AppState};

#[derive(Serialize, Debug)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub stock_amount: String,
    pub price: BigDecimal,
}

#[derive(Serialize)]
pub struct ProductWithCategories {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub stock_amount: String,
    pub price: BigDecimal,
    pub categories: Vec<i32>
}

#[derive(Serialize)]
pub struct ProductWithCategoriesQueryResult {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub stock_amount: String,
    pub price: BigDecimal,
    pub categories: Option<Vec<i32>>,
}

#[derive(Deserialize)]
pub struct ProductCreateDto {
    pub title: String,
    pub description: String,
    pub stock_amount: String,
    pub price: BigDecimal,
    pub categories: Vec<i32>,
}

struct ReturnId {
    id: i32,
}

impl Product {
    pub async fn get_all(app_state: &AppState) -> Result<Vec<ProductWithCategories>, Error> {
        let products = sqlx::query_as!(
            ProductWithCategoriesQueryResult,
            "select p.*,
            ARRAY_AGG(pc.category_id) as categories
            from products p
            inner join product_categories pc
            on pc.product_id = p.id
            group by p.id"
        )
            .fetch_all(&app_state.pool)
            .await
            .map_err(|_| Error::GeneralError(GeneralError::Unhandled("Query failed for get products".to_owned())))?;

        let products = products.iter().map(|p| {
            let categories = match &p.categories {
                Some(category_ids) => category_ids.clone(),
                None => Vec::new()
            };
            p.map_to_product_with_categories(&categories)
        }).collect();

        Ok(products)
    }

    pub async fn get_by_id(app_state: &AppState, id: i32) -> Result<Product, Error> {
        let product = sqlx::query_as!(Product, "SELECT * FROM products WHERE id = $1", id)
            .fetch_one(&app_state.pool)
            .await
            .map_err(|_| Error::GeneralError(GeneralError::NotFound))?;
        Ok(product)
    }

    pub async fn create(state: &AppState, product_dto: &ProductCreateDto) -> Result<Product, Error> {
        let result = sqlx::query_as!(
            ReturnId,
            "INSERT INTO products (title, description, stock_amount, price) VALUES ($1, $2, $3, $4) RETURNING id",
            product_dto.title, product_dto.description, product_dto.stock_amount, product_dto.price
        )
            .fetch_one(&state.pool)
            .await
            .map_err(|_| Error::GeneralError(GeneralError::Unhandled("Inserting product failed".to_owned())))?;

        let product = Product {
            id: result.id,
            title: product_dto.title.clone(),
            description: product_dto.description.clone(),
            stock_amount: product_dto.stock_amount.clone(),
            price: product_dto.price.clone(),
        };

        Ok(product)
    }
}

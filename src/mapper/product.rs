use crate::models::product::{Product, ProductWithCategories, ProductWithCategoriesQueryResult};

pub trait ProductMapper {
    fn map_to_product_with_categories(&self, categoriy_ids: &[i32]) -> ProductWithCategories;
}

impl ProductMapper for Product {
    fn map_to_product_with_categories(&self, categoriy_ids: &[i32]) -> ProductWithCategories {
        ProductWithCategories {
            id: self.id,
            title: self.title.clone(),
            description: self.description.clone(),
            stock_amount: self.stock_amount.clone(),
            price: self.price.clone(),
            categories: categoriy_ids.to_vec()
        }
    }
}

impl ProductMapper for ProductWithCategoriesQueryResult {
    fn map_to_product_with_categories(&self, categoriy_ids: &[i32]) -> ProductWithCategories {
        ProductWithCategories {
            id: self.id,
            title: self.title.clone(),
            description: self.description.clone(),
            stock_amount: self.stock_amount.clone(),
            price: self.price.clone(),
            categories: categoriy_ids.to_vec()
        }
    }
}

//! SeaORM entity for GachaItems table
//! Corresponding to ResourceEnum::GachaItems

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_gacha_items")]
pub struct Model {
    #[sea_orm(primary_key, default = "gen_random_uuid()", auto_increment = false)]
    pub id: Uuid,
    
    #[sea_orm(unique, not_null)]
    pub item_code: String,
    
    #[sea_orm(not_null)]
    pub name: String,
    
    #[sea_orm(not_null)]
    pub description: String,
    
    #[sea_orm(not_null)]
    pub rarity: String,
    
    #[sea_orm(not_null)]
    pub type_: String,
    
    #[sea_orm(not_null)]
    pub category: String,
    
    #[sea_orm(not_null)]
    pub value: i32,
    
    #[sea_orm(not_null)]
    pub weight: f64,
    
    #[sea_orm(default = "0")]
    pub stock: i32,
    
    #[sea_orm(default = "false")]
    pub is_limited: bool,
    
    #[sea_orm(type = "jsonb", nullable)]
    pub metadata: Option<serde_json::Value>,
    
    #[sea_orm(not_null, default = "now()")]
    pub created_at: DateTime<Utc>,
    
    #[sea_orm(not_null, default = "now()")]
    pub updated_at: DateTime<Utc>,
    
    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, sea_orm::EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {
    // Default implementation - SeaORM will handle timestamps automatically
}

// Builder pattern for GachaItem creation
#[derive(Default, Serialize, Deserialize)]
pub struct GachaItemBuilder {
    item_code: Option<String>,
    name: Option<String>,
    description: Option<String>,
    rarity: Option<String>,
    type_: Option<String>,
    category: Option<String>,
    value: Option<i32>,
    weight: Option<f64>,
    stock: Option<i32>,
    is_limited: Option<bool>,
    metadata: Option<serde_json::Value>,
}

impl GachaItemBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn item_code(mut self, item_code: String) -> Self {
        self.item_code = Some(item_code);
        self
    }

    #[must_use]
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    #[must_use]
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    #[must_use]
    pub fn rarity(mut self, rarity: String) -> Self {
        self.rarity = Some(rarity);
        self
    }

    #[must_use]
    pub fn type_(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
    }

    #[must_use]
    pub fn category(mut self, category: String) -> Self {
        self.category = Some(category);
        self
    }

    #[must_use]
    pub fn value(mut self, value: i32) -> Self {
        self.value = Some(value);
        self
    }

    #[must_use]
    pub fn weight(mut self, weight: f64) -> Self {
        self.weight = Some(weight);
        self
    }

    #[must_use]
    pub fn stock(mut self, stock: i32) -> Self {
        self.stock = Some(stock);
        self
    }

    #[must_use]
    pub fn is_limited(mut self, is_limited: bool) -> Self {
        self.is_limited = Some(is_limited);
        self
    }

    #[must_use]
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Result<ActiveModel, String> {
        let mut active_model = <ActiveModel as std::default::Default>::default();

        if let Some(item_code) = self.item_code {
            active_model.item_code = Set(item_code);
        } else {
            return Err("Item code is required".to_string());
        }

        if let Some(name) = self.name {
            active_model.name = Set(name);
        } else {
            return Err("Name is required".to_string());
        }

        if let Some(description) = self.description {
            active_model.description = Set(description);
        } else {
            return Err("Description is required".to_string());
        }

        if let Some(rarity) = self.rarity {
            active_model.rarity = Set(rarity);
        } else {
            return Err("Rarity is required".to_string());
        }

        if let Some(type_) = self.type_ {
            active_model.type_ = Set(type_);
        } else {
            return Err("Type is required".to_string());
        }

        if let Some(category) = self.category {
            active_model.category = Set(category);
        } else {
            return Err("Category is required".to_string());
        }

        if let Some(value) = self.value {
            active_model.value = Set(value);
        } else {
            return Err("Value is required".to_string());
        }

        if let Some(weight) = self.weight {
            active_model.weight = Set(weight);
        } else {
            return Err("Weight is required".to_string());
        }

        if let Some(stock) = self.stock {
            active_model.stock = Set(stock);
        }

        if let Some(is_limited) = self.is_limited {
            active_model.is_limited = Set(is_limited);
        }

        if let Some(metadata) = self.metadata {
            active_model.metadata = Set(Some(metadata));
        }

        Ok(active_model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gacha_item_model_creation() {
        let item = GachaItemBuilder::new()
            .item_code("SWORD_001".to_string())
            .name("Legendary Sword".to_string())
            .description("A powerful legendary sword".to_string())
            .rarity("legendary".to_string())
            .type_("weapon".to_string())
            .category("sword".to_string())
            .value(100)
            .weight(0.01)
            .stock(10)
            .is_limited(true)
            .build();

        assert!(item.is_ok());
        let item_model = item.unwrap();
        assert_eq!(item_model.item_code, Set("SWORD_001".to_string()));
        assert_eq!(item_model.name, Set("Legendary Sword".to_string()));
        assert_eq!(item_model.description, Set("A powerful legendary sword".to_string()));
        assert_eq!(item_model.rarity, Set("legendary".to_string()));
        assert_eq!(item_model.type_, Set("weapon".to_string()));
        assert_eq!(item_model.category, Set("sword".to_string()));
        assert_eq!(item_model.value, Set(100));
        assert_eq!(item_model.weight, Set(0.01));
        assert_eq!(item_model.stock, Set(10));
        assert_eq!(item_model.is_limited, Set(true));
    }
}
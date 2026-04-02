use super::gacha_items::{ActiveModel, GachaItemBuilder};
use sea_orm::ActiveValue::Set;

impl GachaItemBuilder {
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

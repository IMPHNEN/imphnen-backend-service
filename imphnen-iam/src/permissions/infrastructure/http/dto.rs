use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use zod_rs::prelude::*;
use uuid::Uuid;
use crate::permissions::domain::PermissionEntity;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct PermissionsCreateRequestDto {
    #[zod(min_length(1))]
    pub name: String,
}

impl ZodValidate for PermissionsCreateRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct PermissionsUpdateRequestDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ZodValidate for PermissionsUpdateRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PermissionsItemDto {
    pub id: String,
    pub name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<PermissionEntity> for PermissionsItemDto {
    fn from(e: PermissionEntity) -> Self {
        Self {
            id: e.id.to_string(),
            name: e.name,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

impl PermissionsUpdateRequestDto {
    pub fn apply_to(self, mut entity: PermissionEntity, id: String) -> PermissionEntity {
        entity.id = Uuid::parse_str(&id).unwrap_or(entity.id);
        if let Some(name) = self.name {
            entity.name = name;
        }
        entity
    }
}

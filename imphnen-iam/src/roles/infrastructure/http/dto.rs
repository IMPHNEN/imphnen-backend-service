use imphnen_entities::{PermissionsEnum, PermissionsItemDto};
use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use utoipa::ToSchema;
use zod_rs::prelude::*;
use crate::roles::domain::RoleEntity;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct RolesCreateRequestDto {
    #[zod(min_length(1))]
    pub name: String,
    pub permissions: Vec<String>,
}

impl ZodValidate for RolesCreateRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct RolesUpdateRequestDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

impl ZodValidate for RolesUpdateRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesListItemDto {
    pub id: String,
    pub name: String,
    pub permissions_count: usize,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<RoleEntity> for RolesListItemDto {
    fn from(e: RoleEntity) -> Self {
        Self {
            permissions_count: e.permissions.len(),
            id: e.id.to_string(),
            name: e.name,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Default)]
pub struct RolesDetailItemDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub is_system_role: bool,
    pub is_default: bool,
    pub permissions: Vec<PermissionsItemDto>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<RoleEntity> for RolesDetailItemDto {
    fn from(e: RoleEntity) -> Self {
        let permissions_dto = e.permissions.iter().map(|p_str| {
            for enum_val in PermissionsEnum::iter() {
                if enum_val.to_string() == *p_str {
                    return PermissionsItemDto {
                        id: enum_val.id(),
                        name: p_str.clone(),
                        created_at: None,
                        updated_at: None,
                    };
                }
            }
            PermissionsItemDto {
                id: String::new(),
                name: p_str.clone(),
                created_at: None,
                updated_at: None,
            }
        }).collect();

        Self {
            id: e.id.to_string(),
            name: e.name,
            description: e.description,
            is_system_role: e.is_system_role,
            is_default: e.is_default,
            permissions: permissions_dto,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

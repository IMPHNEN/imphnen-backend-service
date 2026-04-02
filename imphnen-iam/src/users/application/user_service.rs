use std::sync::Arc;
use async_trait::async_trait;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use imphnen_utils::AppError;
use imphnen_libs::{hash_password, verify_password};
use crate::users::domain::{UserEntity, UserListItem, UserRepository, UserService};

pub struct UserServiceImpl {
    repo: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn list(&self, params: PaginationParams) -> Result<PaginatorResponse<UserListItem>, AppError> {
        self.repo.find_all(params).await
    }

    async fn get(&self, id: String) -> Result<UserEntity, AppError> {
        self.repo.find_by_id(&id).await
    }

    async fn get_me(&self, user_id: String) -> Result<UserEntity, AppError> {
        self.repo.find_by_id(&user_id).await
    }

    async fn get_by_email(&self, email: String) -> Result<UserEntity, AppError> {
        self.repo.find_by_email(email).await
    }

    async fn create(&self, entity: UserEntity) -> Result<UserEntity, AppError> {
        // Check for email conflict
        match self.repo.find_by_email(entity.email.clone()).await {
            Ok(_) => return Err(AppError::ConflictError("User already exists".into())),
            Err(AppError::NotFoundError(_)) => {}
            Err(e) => return Err(e),
        }
        let email = entity.email.clone();
        self.repo.create(entity).await?;
        self.repo.find_by_email(email).await
    }

    async fn update(&self, entity: UserEntity) -> Result<String, AppError> {
        let existing = self.repo.find_by_id(&entity.id).await?;
        if existing.is_deleted {
            return Err(AppError::NotFoundError("User not found".into()));
        }
        self.repo.update(entity).await
    }

    async fn delete(&self, id: String) -> Result<String, AppError> {
        let user = self.repo.find_by_id(&id).await?;
        if user.is_deleted {
            return Err(AppError::NotFoundError("User not found".into()));
        }
        self.repo.delete(id).await
    }

    async fn set_active_status(&self, id: String, is_active: bool) -> Result<String, AppError> {
        let mut user = self.repo.find_by_id(&id).await?;
        if user.is_deleted {
            return Err(AppError::NotFoundError("User not found".into()));
        }
        user.is_active = is_active;
        self.repo.update(user).await
    }

    async fn update_password(&self, email: String, old_password: String, new_password: String) -> Result<String, AppError> {
        let user = self.repo.find_by_email(email.clone()).await
            .map_err(|_| AppError::NotFoundError("User not found".into()))?;

        if user.is_deleted {
            return Err(AppError::NotFoundError("User not found".into()));
        }

        let is_valid = verify_password(&old_password, &user.password)
            .map_err(|_| AppError::BadRequestError("Password verification failed".into()))?;
        if !is_valid {
            return Err(AppError::BadRequestError("Old password is incorrect".into()));
        }

        let new_hash = hash_password(&new_password)
            .map_err(|_| AppError::InternalServerError("Failed to hash password".into()))?;

        let mut updated = user;
        updated.password = new_hash;
        self.repo.update(updated).await
    }
}

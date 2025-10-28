use super::hackathon_audit_schema::{AuditAction, HackathonAuditLogSchema};
use anyhow::Result;
use imphnen_libs::{AppState, MetaRequestDto, ResponseListSuccessDto};
use surrealdb::sql::Thing;
use tracing::{info, instrument};

#[derive(Clone)]
pub struct HackathonAuditRepository<'a> {
    pub state: &'a AppState,
}

impl<'a> HackathonAuditRepository<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    #[instrument(skip(self, log), err)]
    pub async fn log(&self, log: HackathonAuditLogSchema) -> Result<HackathonAuditLogSchema> {
        let table = "app_hackathon_audit_logs";
        let id = log.id.id.to_string();

        info!(
            action = %log.action,
            actor_id = %log.actor_id,
            resource_type = %log.resource_type,
            "Creating audit log entry"
        );

        let record: Option<HackathonAuditLogSchema> = self
            .state
            .surrealdb_ws
            .create((table, id.clone()))
            .content(log.clone())
            .await?;

        record.ok_or_else(|| anyhow::anyhow!("Failed to create audit log"))
    }

    #[instrument(skip(self), err)]
    pub async fn get_logs_by_hackathon(
        &self,
        hackathon_id: &Thing,
        meta: MetaRequestDto,
    ) -> Result<ResponseListSuccessDto<Vec<HackathonAuditLogSchema>>> {
        let table = "app_hackathon_audit_logs";
        let page = meta.page.unwrap_or(1);
        let per_page = meta.per_page.unwrap_or(50);
        let start = (page - 1) * per_page;
        
        let condition = format!("hackathon_id = {}", hackathon_id);
        
        let query = format!(
            "SELECT * FROM {} WHERE {} ORDER BY timestamp DESC LIMIT {} START {}",
            table, condition, per_page, start
        );
        
        let count_query = format!(
            "SELECT count() as count FROM {} WHERE {} GROUP ALL",
            table, condition
        );

        info!(query = %query, "Executing query to get audit logs");

        let logs: Vec<HackathonAuditLogSchema> = self.state.surrealdb_ws.query(&query).await?.take(0)?;
        let count_result: Vec<imphnen_entities::common_dto::CountResult> =
            self.state.surrealdb_ws.query(&count_query).await?.take(0)?;

        let total = count_result.first().map(|r| r.count).unwrap_or(0);

        Ok(ResponseListSuccessDto {
            data: logs,
            meta: Some(imphnen_libs::MetaResponseDto {
                page: Some(page),
                per_page: Some(per_page),
                total: Some(total),
            }),
        })
    }

    #[instrument(skip(self), err)]
    pub async fn get_logs_by_actor(
        &self,
        actor_id: &str,
        meta: MetaRequestDto,
    ) -> Result<ResponseListSuccessDto<Vec<HackathonAuditLogSchema>>> {
        let table = "app_hackathon_audit_logs";
        let page = meta.page.unwrap_or(1);
        let per_page = meta.per_page.unwrap_or(50);
        let start = (page - 1) * per_page;
        
        let condition = format!("actor_id = '{}'", actor_id);
        
        let query = format!(
            "SELECT * FROM {} WHERE {} ORDER BY timestamp DESC LIMIT {} START {}",
            table, condition, per_page, start
        );
        
        let count_query = format!(
            "SELECT count() as count FROM {} WHERE {} GROUP ALL",
            table, condition
        );

        let logs: Vec<HackathonAuditLogSchema> = self.state.surrealdb_ws.query(&query).await?.take(0)?;
        let count_result: Vec<imphnen_entities::common_dto::CountResult> =
            self.state.surrealdb_ws.query(&count_query).await?.take(0)?;

        let total = count_result.first().map(|r| r.count).unwrap_or(0);

        Ok(ResponseListSuccessDto {
            data: logs,
            meta: Some(imphnen_libs::MetaResponseDto {
                page: Some(page),
                per_page: Some(per_page),
                total: Some(total),
            }),
        })
    }

    #[instrument(skip(self), err)]
    pub async fn get_logs_by_action(
        &self,
        action: AuditAction,
        meta: MetaRequestDto,
    ) -> Result<ResponseListSuccessDto<Vec<HackathonAuditLogSchema>>> {
        let table = "app_hackathon_audit_logs";
        let page = meta.page.unwrap_or(1);
        let per_page = meta.per_page.unwrap_or(50);
        let start = (page - 1) * per_page;
        
        let condition = format!("action = '{}'", action.to_string());
        
        let query = format!(
            "SELECT * FROM {} WHERE {} ORDER BY timestamp DESC LIMIT {} START {}",
            table, condition, per_page, start
        );
        
        let count_query = format!(
            "SELECT count() as count FROM {} WHERE {} GROUP ALL",
            table, condition
        );

        let logs: Vec<HackathonAuditLogSchema> = self.state.surrealdb_ws.query(&query).await?.take(0)?;
        let count_result: Vec<imphnen_entities::common_dto::CountResult> =
            self.state.surrealdb_ws.query(&count_query).await?.take(0)?;

        let total = count_result.first().map(|r| r.count).unwrap_or(0);

        Ok(ResponseListSuccessDto {
            data: logs,
            meta: Some(imphnen_libs::MetaResponseDto {
                page: Some(page),
                per_page: Some(per_page),
                total: Some(total),
            }),
        })
    }

    #[instrument(skip(self), err)]
    pub async fn get_all_logs(
        &self,
        meta: MetaRequestDto,
    ) -> Result<ResponseListSuccessDto<Vec<HackathonAuditLogSchema>>> {
        let table = "app_hackathon_audit_logs";
        let page = meta.page.unwrap_or(1);
        let per_page = meta.per_page.unwrap_or(50);
        let start = (page - 1) * per_page;
        
        let query = format!(
            "SELECT * FROM {} ORDER BY timestamp DESC LIMIT {} START {}",
            table, per_page, start
        );
        
        let count_query = format!(
            "SELECT count() as count FROM {} GROUP ALL",
            table
        );

        let logs: Vec<HackathonAuditLogSchema> = self.state.surrealdb_ws.query(&query).await?.take(0)?;
        let count_result: Vec<imphnen_entities::common_dto::CountResult> =
            self.state.surrealdb_ws.query(&count_query).await?.take(0)?;

        let total = count_result.first().map(|r| r.count).unwrap_or(0);

        Ok(ResponseListSuccessDto {
            data: logs,
            meta: Some(imphnen_libs::MetaResponseDto {
                page: Some(page),
                per_page: Some(per_page),
                total: Some(total),
            }),
        })
    }
}

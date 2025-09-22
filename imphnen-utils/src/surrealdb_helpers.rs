use surrealdb::sql::Thing;
use anyhow::Result;

pub fn build_thing_condition(field: &str, thing: &Thing) -> String {
    format!("{} = type::thing('{}', '{}')", field, thing.tb, thing.id.to_raw())
}

pub fn build_multi_thing_condition(conditions: &[(&str, &Thing)]) -> String {
    conditions
        .iter()
        .map(|(field, thing)| build_thing_condition(field, thing))
        .collect::<Vec<_>>()
        .join(" AND ")
}

pub async fn execute_safe_update_query(
    db: &surrealdb::Surreal<surrealdb::engine::any::Any>,
    query: String,
) -> Result<()> {
    let mut result = db.query(query).await?;
    let _: Result<Vec<serde_json::Value>, _> = result.take(0);
    Ok(())
}

pub async fn execute_safe_count_query(
    db: &surrealdb::Surreal<surrealdb::engine::any::Any>,
    table: &str,
    conditions: &str,
) -> Result<u64> {
    let query = format!("SELECT COUNT() AS member_count FROM {} WHERE {}", table, conditions);
    let mut result = db.query(query).await?;
    
    let count_result: Vec<serde_json::Value> = result.take(0).unwrap_or_default();
    
    let count = if let Some(first_result) = count_result.first() {
        if let Some(count_val) = first_result.get("member_count") {
            count_val.as_u64().unwrap_or(0)
        } else {
            0
        }
    } else {
        0
    };
    
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_thing_from_enum;
    use imphnen_libs::ResourceEnum;

    #[test]
    fn test_build_thing_condition() {
        let team_thing = make_thing_from_enum(ResourceEnum::Teams, "test-id");
        let condition = build_thing_condition("team_id", &team_thing);
        assert_eq!(condition, "team_id = type::thing('app_teams', 'test-id')");
    }

    #[test]
    fn test_build_multi_thing_condition() {
        let team_thing = make_thing_from_enum(ResourceEnum::Teams, "team-id");
        let user_thing = make_thing_from_enum(ResourceEnum::Users, "user-id");
        
        let conditions = build_multi_thing_condition(&[
            ("team_id", &team_thing),
            ("user_id", &user_thing),
        ]);
        
        assert_eq!(
            conditions,
            "team_id = type::thing('app_teams', 'team-id') AND user_id = type::thing('app_users', 'user-id')"
        );
    }
}
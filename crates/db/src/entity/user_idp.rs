//! SeaORM `user_idp` entity — maps external identities to local users.

use async_trait::async_trait;
use sea_orm::{DeriveActiveEnum, EnumIter, Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user_idps")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub idp_id: i64,
    pub auth_key: String,
    pub source: Source,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub data: Option<Json>,
    pub created_at: i64,
    pub updated_at: i64,
    #[sea_orm(belongs_to, from = "idp_id", to = "id", on_delete = "Cascade")]
    pub idp: BelongsTo<super::idp::Entity>,
    #[sea_orm(belongs_to, from = "user_id", to = "id", on_delete = "Cascade")]
    pub user: BelongsTo<super::user::Entity>,
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    EnumIter,
    DeriveActiveEnum,
    utoipa::ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(rename_all = "snake_case")]
pub enum Source {
    #[sea_orm(string_value = "registration")]
    Registration,
    #[sea_orm(string_value = "binding")]
    Binding,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait, {
        let ts = time::OffsetDateTime::now_utc().unix_timestamp();
        self.updated_at = Set(ts);
        if insert {
            self.created_at = Set(ts);
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Source;

    #[test]
    fn source_uses_stable_string_values() {
        assert_eq!(
            serde_json::to_value(Source::Registration).unwrap(),
            "registration"
        );
        assert_eq!(
            serde_json::from_str::<Source>(r#""binding""#).unwrap(),
            Source::Binding
        );
    }
}

//! SeaORM `mod` entity — maps the `mod` table and its relations.

use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Serialize, Deserialize, FromJsonQueryResult, Eq, PartialEq, utoipa::ToSchema,
)]
pub struct Config {
    #[serde(alias = "registration_enabled")]
    pub local_registration_enabled: bool,
}

impl Default for Config {
    /// Returns the default value for this type.
    fn default() -> Self {
        Self {
            local_registration_enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn legacy_registration_setting_is_read_but_not_written() {
        let config: Config = serde_json::from_value(serde_json::json!({
            "registration_enabled": false,
        }))
        .unwrap();
        assert!(!config.local_registration_enabled);

        let value = serde_json::to_value(config).unwrap();
        assert_eq!(value["local_registration_enabled"], false);
        assert!(value.get("registration_enabled").is_none());
    }
}

//! Query parameters shared by the account mail endpoints.

#[derive(serde::Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct EmailLocaleQuery {
    /// Preferred email language. Falls back to en-US when omitted or
    /// unsupported.
    pub locale: Option<String>,
}

//! Shared email policy and account mail query parameters.

use serde_json::json;

use crate::traits::WebError;

#[derive(serde::Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct EmailLocaleQuery {
    /// Preferred email language. Falls back to en-US when omitted or
    /// unsupported.
    pub locale: Option<String>,
}

/// Normalizes domain rules supplied by the management API.
pub fn normalize_whitelist(rules: &mut [String]) -> Result<(), WebError> {
    for rule in rules {
        let normalized = rule.trim().to_ascii_lowercase();
        let domain = normalized.strip_prefix("*.").unwrap_or(&normalized);
        if !is_domain(domain) {
            return Err(WebError::BadRequest(json!("invalid_email_domain_rule")));
        }
        *rule = normalized;
    }

    Ok(())
}

fn is_domain(domain: &str) -> bool {
    domain.len() <= 253
        && domain.split('.').all(|label| {
            (1..=63).contains(&label.len())
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        })
}

/// Checks policy when creating or linking an email address.
pub fn validate_new_address(address: &str, rules: &[String]) -> Result<(), WebError> {
    garde::rules::email::apply(&address, ())
        .map_err(|_| WebError::BadRequest(json!("invalid_email")))?;

    if rules.is_empty() {
        return Ok(());
    }

    let (_, domain) = address
        .rsplit_once('@')
        .ok_or_else(|| WebError::BadRequest(json!("invalid_email")))?;
    let domain = domain.to_ascii_lowercase();
    let allowed = rules.iter().any(|rule| {
        let rule = rule.trim().to_ascii_lowercase();
        if let Some(suffix) = rule.strip_prefix("*.") {
            domain
                .strip_suffix(suffix)
                .is_some_and(|prefix| prefix.ends_with('.'))
        } else {
            domain == rule
        }
    });

    if allowed {
        Ok(())
    } else {
        Err(WebError::BadRequest(json!("email_domain_not_allowed")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_exact_and_wildcard_domain_rules() {
        let mut rules = [" EXAMPLE.EDU ".to_owned(), "*.Lab.Example.edu".to_owned()];
        normalize_whitelist(&mut rules).unwrap();
        assert_eq!(rules, ["example.edu", "*.lab.example.edu"]);

        for rule in [
            "",
            "*",
            "*.",
            "a.*.edu",
            "a..edu",
            "a-.edu",
            "@example.edu",
            "https://example.edu",
        ] {
            assert!(normalize_whitelist(&mut [rule.to_owned()]).is_err());
        }
    }

    #[test]
    fn matches_whole_domains_and_wildcard_label_boundaries() {
        let rules = ["example.edu".to_owned(), "*.lab.example.edu".to_owned()];
        for address in [
            "user@EXAMPLE.EDU",
            "user@team.lab.example.edu",
            "user@a.team.lab.example.edu",
        ] {
            validate_new_address(address, &rules).unwrap();
        }
        for address in [
            "user@example.edu.evil.net",
            "user@other.example.edu",
            "user@lab.example.edu",
            "user@evillab.example.edu",
        ] {
            assert!(matches!(
                validate_new_address(address, &rules),
                Err(WebError::BadRequest(code)) if code == "email_domain_not_allowed"
            ));
        }
        validate_new_address("user@anything.example", &[]).unwrap();
        assert!(validate_new_address("user@@example.edu", &rules).is_err());
    }
}

//! Typed flag ownership at the Rust boundary; Lua receives an opaque integer.

use crate::traits::CheckerError;

/// The owner of generated flags, independently of the user operating a team.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Subject {
    User(i64),
    Team(i64),
}

impl Subject {
    /// Preserve positive team ids and put positive user ids in the negative
    /// namespace. Zero and i64::MIN are never valid subject numbers.
    pub fn subject_id(self) -> Result<i64, CheckerError> {
        let (Self::User(id) | Self::Team(id)) = self;
        if id <= 0 {
            return Err(CheckerError::ScriptError(
                "checker subject requires a positive database id".to_owned(),
            ));
        }
        Ok(match self {
            Self::User(id) => -id,
            Self::Team(id) => id,
        })
    }

    pub(crate) fn from_subject_id(subject_id: i64) -> Option<Self> {
        match subject_id {
            1.. => Some(Self::Team(subject_id)),
            ..=-1 => subject_id.checked_neg().map(Self::User),
            0 => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Subject;

    #[test]
    fn subject_numbers_round_trip_without_collisions() {
        for id in [1, 42, i64::MAX] {
            let user = Subject::User(id).subject_id().unwrap();
            let team = Subject::Team(id).subject_id().unwrap();
            assert_eq!(team, id);
            assert_ne!(user, team);
            assert_eq!(Subject::from_subject_id(user), Some(Subject::User(id)));
            assert_eq!(Subject::from_subject_id(team), Some(Subject::Team(id)));
        }
    }

    #[test]
    fn rejects_invalid_database_ids_and_subject_numbers() {
        for id in [0, -1, i64::MIN] {
            assert!(Subject::User(id).subject_id().is_err());
            assert!(Subject::Team(id).subject_id().is_err());
        }
        assert_eq!(Subject::from_subject_id(0), None);
        assert_eq!(Subject::from_subject_id(i64::MIN), None);
    }
}

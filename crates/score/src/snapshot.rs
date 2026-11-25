//! Pure, immutable score snapshot built from ground-truth PostgreSQL facts.
//!
//! Only numeric identifiers and derived scores/ranks are stored here — team
//! profiles, usernames, and challenge titles stay in PostgreSQL so display
//! edits never invalidate or bloat the score cache.

use std::collections::{HashMap, HashSet};

use cds_db::{
    game_challenge::ScoreInput as ChallengeScoreInput,
    submission::ScoreInput as SubmissionScoreInput,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::math;

/// Standing entry for one `Passed` team.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankedTeam {
    pub team_id: i64,
    pub pts: i64,
    pub rank: i64,
}

/// Derived score metadata for one challenge within a game.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeScore {
    pub challenge_id: i64,
    /// Points awarded to the *next* solver (current curve value + next blood
    /// bonus, if any).
    pub pts: i64,
    /// Total number of valid (`Passed`-team) solves.
    pub solve_count: i64,
    /// Submission IDs of up to the first 3 solves in chronological order.
    pub blood_submission_ids: Vec<i64>,
}

/// Derived score and solve order (`1`-based blood rank) for one correct
/// submission.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmissionScore {
    pub pts: i64,
    pub rank: i64,
}

/// Complete, self-contained score projection for one competition at a specific
/// monotonic `version` (Valkey epoch).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameScoreSnapshot {
    pub version: u64,
    pub computed_at_ms: i64,
    /// All `Passed` teams ordered by `(pts DESC, last_solve_at ASC, team_id
    /// ASC)`.
    pub ranked_teams: Vec<RankedTeam>,
    /// Per-challenge dynamic score, solve count, and top-3 blood IDs.
    pub challenges: HashMap<i64, ChallengeScore>,
    /// Per-correct-submission `(pts, rank)` lookup.
    pub submission_scores: HashMap<i64, SubmissionScore>,
    /// Per-team sorted list of solved `challenge_id`s for `O(log C)` lookup in
    /// `POST /api/challenges/status`.
    pub team_solved_challenges: HashMap<i64, Vec<i64>>,
    /// In-memory O(1) index `team_id -> position in ranked_teams`, rebuilt on
    /// deserialization to keep Valkey payloads compact.
    #[serde(skip)]
    pub team_index: HashMap<i64, usize>,
}

impl GameScoreSnapshot {
    /// Rebuilds non-serialized lookup indexes after deserializing from Valkey.
    pub fn rebuild_indexes(&mut self) {
        self.team_index = self
            .ranked_teams
            .iter()
            .enumerate()
            .map(|(idx, team)| (team.team_id, idx))
            .collect();
    }

    /// Returns `(pts, rank)` for `team_id`, or `(0, 0)` if the team is not a
    /// ranked (`Passed`) team in this game.
    pub fn team_score(&self, team_id: i64) -> (i64, i64) {
        self.team_index
            .get(&team_id)
            .and_then(|&idx| self.ranked_teams.get(idx))
            .map(|entry| (entry.pts, entry.rank))
            .unwrap_or((0, 0))
    }

    /// Returns the current displayed points for `challenge_id`, or `0` if
    /// absent.
    pub fn challenge_pts(&self, challenge_id: i64) -> i64 {
        self.challenges
            .get(&challenge_id)
            .map(|c| c.pts)
            .unwrap_or(0)
    }

    /// Returns derived `(pts, rank)` for a submission, or `(0, 0)` if it is not
    /// a scored `correct` submission of a `Passed` team.
    pub fn submission_score(&self, submission_id: i64) -> (i64, i64) {
        self.submission_scores
            .get(&submission_id)
            .map(|s| (s.pts, s.rank))
            .unwrap_or((0, 0))
    }

    /// Returns whether `team_id` has a valid solve on `challenge_id`.
    pub fn team_solved(&self, team_id: i64, challenge_id: i64) -> bool {
        self.team_solved_challenges
            .get(&team_id)
            .is_some_and(|ids| ids.binary_search(&challenge_id).is_ok())
    }

    /// Returns the requested 1-based page slice of ranked teams and the total
    /// ranked team count.
    pub fn page_teams(&self, page: Option<u64>, size: Option<u64>) -> (&[RankedTeam], u64) {
        let total = self.ranked_teams.len() as u64;
        match (page, size) {
            (Some(_), Some(0)) => (&[], total),
            (Some(page), Some(size)) => {
                let start = (page.saturating_sub(1).saturating_mul(size)) as usize;
                if start >= self.ranked_teams.len() {
                    (&[], total)
                } else {
                    let end = start
                        .saturating_add(size as usize)
                        .min(self.ranked_teams.len());
                    (&self.ranked_teams[start..end], total)
                }
            }
            _ => (&self.ranked_teams, total),
        }
    }

    /// Encodes the snapshot into a compact MessagePack buffer for Valkey.
    pub fn encode(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(self)
    }

    /// Decodes a MessagePack snapshot and rebuilds transient lookup indexes.
    pub fn decode(bytes: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        let mut snapshot: Self = rmp_serde::from_slice(bytes)?;
        snapshot.rebuild_indexes();
        Ok(snapshot)
    }
}

/// Deterministically computes a complete [`GameScoreSnapshot`] from narrow
/// PostgreSQL inputs in a single pass.
///
/// Only submissions whose `team_id` belongs to `passed_team_ids` are scored,
/// ensuring banned or unapproved teams never affect dynamic decay or blood
/// bonuses.
pub fn build(
    version: u64,
    computed_at_ms: i64,
    mut submissions: Vec<SubmissionScoreInput>,
    mut challenges: Vec<ChallengeScoreInput>,
    passed_team_ids: Vec<i64>,
) -> GameScoreSnapshot {
    let passed_set: HashSet<i64> = passed_team_ids.iter().copied().collect();

    // Filter defensively even though SQL already joins `teams.state = Passed`,
    // and enforce a deterministic total order independent of DB scan order.
    submissions.retain(|s| passed_set.contains(&s.team_id));
    submissions.sort_by_key(|s| (s.challenge_id, s.created_at, s.id));
    challenges.sort_by_key(|c| c.challenge_id);

    let mut by_challenge: HashMap<i64, Vec<SubmissionScoreInput>> =
        HashMap::with_capacity(challenges.len());
    for submission in submissions {
        by_challenge
            .entry(submission.challenge_id)
            .or_default()
            .push(submission);
    }

    let mut challenge_scores: HashMap<i64, ChallengeScore> =
        HashMap::with_capacity(challenges.len());
    let mut submission_scores: HashMap<i64, SubmissionScore> = HashMap::new();
    let mut team_totals: HashMap<i64, (i64, Option<OffsetDateTime>)> =
        HashMap::with_capacity(passed_team_ids.len());
    let mut team_solved_challenges: HashMap<i64, Vec<i64>> =
        HashMap::with_capacity(passed_team_ids.len());

    for challenge in challenges {
        let solves = by_challenge
            .remove(&challenge.challenge_id)
            .unwrap_or_default();
        let solve_count = solves.len();
        let base_pts = math::curve(
            challenge.max_pts,
            challenge.min_pts,
            challenge.difficulty,
            solve_count as i64,
        );

        let mut blood_submission_ids = Vec::with_capacity(solve_count.min(3));
        for (index, submission) in solves.into_iter().enumerate() {
            let bonus = challenge.bonus_ratios.get(index).copied().unwrap_or(0);
            let pts = base_pts * (100 + bonus) / 100;
            let rank = index as i64 + 1;

            if index < 3 {
                blood_submission_ids.push(submission.id);
            }
            submission_scores.insert(submission.id, SubmissionScore { pts, rank });

            let total = team_totals.entry(submission.team_id).or_insert((0, None));
            total.0 += pts;
            total.1 = Some(total.1.map_or(submission.created_at, |last| {
                last.max(submission.created_at)
            }));

            team_solved_challenges
                .entry(submission.team_id)
                .or_default()
                .push(challenge.challenge_id);
        }

        // Displayed challenge value matches the legacy calculator: current
        // base score plus the bonus ratio for the next (solve_count-th) blood.
        let next_bonus = challenge
            .bonus_ratios
            .get(solve_count)
            .copied()
            .unwrap_or(0);
        let challenge_pts = base_pts * (100 + next_bonus) / 100;

        challenge_scores.insert(
            challenge.challenge_id,
            ChallengeScore {
                challenge_id: challenge.challenge_id,
                pts: challenge_pts,
                solve_count: solve_count as i64,
                blood_submission_ids,
            },
        );
    }

    // `challenges` was sorted by `challenge_id`, so each team's solved list is
    // already strictly ascending (dedup for defensive safety).
    for solved in team_solved_challenges.values_mut() {
        solved.sort_unstable();
        solved.dedup();
    }

    // Rank all Passed teams by (pts DESC, last_solve_at ASC, team_id ASC).
    let mut ranked: Vec<(i64, i64, Option<OffsetDateTime>)> = passed_team_ids
        .into_iter()
        .map(|team_id| {
            let (pts, last_solve_at) = team_totals.get(&team_id).copied().unwrap_or((0, None));
            (team_id, pts, last_solve_at)
        })
        .collect();
    ranked.sort_by(|(a_id, a_pts, a_time), (b_id, b_pts, b_time)| {
        b_pts
            .cmp(a_pts)
            .then_with(|| a_time.cmp(b_time))
            .then_with(|| a_id.cmp(b_id))
    });

    let ranked_teams: Vec<RankedTeam> = ranked
        .into_iter()
        .enumerate()
        .map(|(index, (team_id, pts, _))| RankedTeam {
            team_id,
            pts,
            rank: index as i64 + 1,
        })
        .collect();

    let mut snapshot = GameScoreSnapshot {
        version,
        computed_at_ms,
        ranked_teams,
        challenges: challenge_scores,
        submission_scores,
        team_solved_challenges,
        team_index: HashMap::new(),
    };
    snapshot.rebuild_indexes();
    snapshot
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ts(seconds: i64) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(seconds).unwrap()
    }

    fn challenge(challenge_id: i64) -> ChallengeScoreInput {
        ChallengeScoreInput {
            challenge_id,
            difficulty: 10,
            max_pts: 1_000,
            min_pts: 100,
            bonus_ratios: vec![10, 5, 0],
        }
    }

    fn solve(id: i64, challenge_id: i64, team_id: i64, seconds: i64) -> SubmissionScoreInput {
        SubmissionScoreInput {
            id,
            challenge_id,
            team_id,
            created_at: ts(seconds),
        }
    }

    #[test]
    fn builds_exact_scores_bloods_and_team_ranks() {
        let snap = build(
            3,
            1_700_000_000_000,
            vec![solve(2, 10, 2, 200), solve(1, 10, 1, 100)],
            vec![challenge(10)],
            vec![1, 2, 3],
        );

        let base = math::curve(1_000, 100, 10, 2);
        assert_eq!(snap.submission_score(1), (base * 110 / 100, 1));
        assert_eq!(snap.submission_score(2), (base * 105 / 100, 2));
        assert_eq!(snap.challenge_pts(10), base);
        assert_eq!(snap.challenges[&10].solve_count, 2);
        assert_eq!(snap.challenges[&10].blood_submission_ids, vec![1, 2]);
        assert_eq!(snap.team_score(1), (base * 110 / 100, 1));
        assert_eq!(snap.team_score(2), (base * 105 / 100, 2));
        assert_eq!(snap.team_score(3), (0, 3));
        assert!(snap.team_solved(1, 10));
        assert!(!snap.team_solved(3, 10));
    }

    #[test]
    fn excludes_banned_teams_from_decay_and_bloods() {
        // Team 99 is NOT in passed_team_ids (e.g. banned for cheating).
        let snap = build(
            1,
            0,
            vec![solve(1, 10, 99, 50), solve(2, 10, 1, 100)],
            vec![challenge(10)],
            vec![1],
        );

        // Team 1 receives full 1-solve base score AND first-blood +10% bonus.
        let base = math::curve(1_000, 100, 10, 1);
        assert_eq!(snap.challenges[&10].solve_count, 1);
        assert_eq!(snap.challenges[&10].blood_submission_ids, vec![2]);
        assert_eq!(snap.submission_score(2), (base * 110 / 100, 1));
        assert_eq!(snap.submission_score(1), (0, 0));
        assert_eq!(snap.team_score(99), (0, 0));
    }

    #[test]
    fn equal_timestamps_tiebreak_deterministically_by_id() {
        let snap = build(
            1,
            0,
            vec![solve(20, 10, 2, 100), solve(10, 10, 1, 100)],
            vec![challenge(10)],
            vec![2, 1],
        );
        assert_eq!(snap.submission_score(10).1, 1);
        assert_eq!(snap.submission_score(20).1, 2);
        assert_eq!(snap.team_score(1).1, 1);
        assert_eq!(snap.team_score(2).1, 2);
    }

    #[test]
    fn round_trips_through_messagepack_and_rebuilds_index() {
        let original = build(
            42,
            1_700_000_000_000,
            vec![solve(1, 10, 1, 100), solve(2, 10, 2, 200)],
            vec![challenge(10)],
            vec![1, 2, 3],
        );
        let bytes = original.encode().unwrap();
        let decoded = GameScoreSnapshot::decode(&bytes).unwrap();
        assert_eq!(decoded, original);
        assert_eq!(decoded.team_score(2), original.team_score(2));
    }

    #[test]
    fn scales_cleanly_to_5000_teams_and_50000_solves() {
        let challenges: Vec<_> = (1..=40).map(challenge).collect();
        let teams: Vec<i64> = (1..=5_000).collect();
        let submissions: Vec<_> = (1..=50_000_i64)
            .map(|id| solve(id, (id - 1) % 40 + 1, (id - 1) % 5_000 + 1, 1_000 + id))
            .collect();

        let started = std::time::Instant::now();
        let snap = build(1, 0, submissions, challenges, teams);
        let elapsed = started.elapsed();

        assert_eq!(snap.ranked_teams.len(), 5_000);
        assert_eq!(snap.submission_scores.len(), 50_000);
        assert_eq!(snap.page_teams(Some(1), Some(60)).0.len(), 60);
        assert_eq!(snap.page_teams(Some(84), Some(60)).0.len(), 20);
        assert!(
            elapsed < std::time::Duration::from_secs(2),
            "5k-team build took {elapsed:?}"
        );
    }
}

//! Pure score planning from a narrow, immutable game snapshot.

use std::collections::HashMap;

use anyhow::{Result, bail};
use cds_db::{
    game_challenge::{ScoreInput as ChallengeScoreInput, ScoreUpdate as ChallengeScoreUpdate},
    submission::{ScoreInput as SubmissionScoreInput, ScoreUpdate as SubmissionScoreUpdate},
    team::{ScoreInput as TeamScoreInput, ScoreUpdate as TeamScoreUpdate},
};

use super::math;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct ScorePlan {
    pub submissions: Vec<SubmissionScoreUpdate>,
    pub challenges: Vec<ChallengeScoreUpdate>,
    pub teams: Vec<TeamScoreUpdate>,
}

/// Computes every persisted score from one database snapshot.
pub(super) fn build(
    mut submissions: Vec<SubmissionScoreInput>,
    mut challenges: Vec<ChallengeScoreInput>,
    teams: Vec<TeamScoreInput>,
) -> Result<ScorePlan> {
    submissions.sort_by_key(|submission| {
        (
            submission.challenge_id,
            submission.created_at,
            submission.id,
        )
    });
    challenges.sort_by_key(|challenge| challenge.challenge_id);

    let mut by_challenge: HashMap<i64, Vec<SubmissionScoreInput>> = HashMap::new();
    for submission in submissions {
        by_challenge
            .entry(submission.challenge_id)
            .or_default()
            .push(submission);
    }

    let mut plan = ScorePlan::default();
    let mut team_totals: HashMap<i64, (i64, Option<i64>)> = HashMap::new();

    for challenge in challenges {
        let challenge_submissions = by_challenge
            .remove(&challenge.challenge_id)
            .unwrap_or_default();
        let solve_count = challenge_submissions.len();
        let base_pts = math::curve(
            challenge.max_pts,
            challenge.min_pts,
            challenge.difficulty,
            solve_count as i64,
        );

        for (index, submission) in challenge_submissions.into_iter().enumerate() {
            let bonus = challenge.bonus_ratios.get(index).copied().unwrap_or(0);
            let pts = base_pts * (100 + bonus) / 100;
            let rank = index as i64 + 1;

            if submission.pts != pts || submission.rank != rank {
                plan.submissions.push(SubmissionScoreUpdate {
                    id: submission.id,
                    pts,
                    rank,
                });
            }

            if let Some(team_id) = submission.team_id {
                let total = team_totals.entry(team_id).or_insert((0, None));
                total.0 += pts;
                total.1 = Some(total.1.map_or(submission.created_at, |last| {
                    last.max(submission.created_at)
                }));
            }
        }

        let challenge_pts = base_pts
            * (100
                + challenge
                    .bonus_ratios
                    .get(solve_count)
                    .copied()
                    .unwrap_or(0))
            / 100;
        if challenge.pts != challenge_pts {
            plan.challenges.push(ChallengeScoreUpdate {
                challenge_id: challenge.challenge_id,
                pts: challenge_pts,
            });
        }
    }

    if let Some(challenge_id) = by_challenge.keys().min().copied() {
        bail!("missing scoring configuration for challenge {challenge_id}");
    }

    let mut ranked_teams = teams
        .into_iter()
        .map(|team| {
            let (pts, last_solve_at) = team_totals.get(&team.id).copied().unwrap_or((0, None));
            (team, pts, last_solve_at)
        })
        .collect::<Vec<_>>();
    ranked_teams.sort_by(|(a, a_pts, a_time), (b, b_pts, b_time)| {
        b_pts
            .cmp(a_pts)
            .then_with(|| a_time.cmp(b_time))
            .then_with(|| a.id.cmp(&b.id))
    });

    for (index, (team, pts, _)) in ranked_teams.into_iter().enumerate() {
        let rank = index as i64 + 1;
        if team.pts != pts || team.rank != rank {
            plan.teams.push(TeamScoreUpdate {
                id: team.id,
                pts,
                rank,
            });
        }
    }

    Ok(plan)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn challenge(challenge_id: i64, pts: i64) -> ChallengeScoreInput {
        ChallengeScoreInput {
            challenge_id,
            difficulty: 10,
            max_pts: 1_000,
            min_pts: 100,
            bonus_ratios: vec![10, 5, 0],
            pts,
        }
    }

    fn submission(
        id: i64,
        challenge_id: i64,
        team_id: i64,
        created_at: i64,
    ) -> SubmissionScoreInput {
        SubmissionScoreInput {
            id,
            challenge_id,
            team_id: Some(team_id),
            created_at,
            pts: 0,
            rank: 0,
        }
    }

    #[test]
    fn builds_submission_challenge_and_team_scores_in_one_pass() {
        let plan = build(
            vec![submission(2, 10, 2, 200), submission(1, 10, 1, 100)],
            vec![challenge(10, 0)],
            vec![
                TeamScoreInput {
                    id: 1,
                    pts: 0,
                    rank: 0,
                },
                TeamScoreInput {
                    id: 2,
                    pts: 0,
                    rank: 0,
                },
            ],
        )
        .unwrap();

        let base = math::curve(1_000, 100, 10, 2);
        assert_eq!(
            plan.submissions,
            vec![
                SubmissionScoreUpdate {
                    id: 1,
                    pts: base * 110 / 100,
                    rank: 1,
                },
                SubmissionScoreUpdate {
                    id: 2,
                    pts: base * 105 / 100,
                    rank: 2,
                },
            ]
        );
        assert_eq!(
            plan.challenges,
            vec![ChallengeScoreUpdate {
                challenge_id: 10,
                pts: base,
            }]
        );
        assert_eq!(plan.teams[0].id, 1);
        assert_eq!(plan.teams[0].pts, base * 110 / 100);
        assert_eq!(plan.teams[0].rank, 1);
        assert_eq!(plan.teams[1].id, 2);
        assert_eq!(plan.teams[1].pts, base * 105 / 100);
        assert_eq!(plan.teams[1].rank, 2);
    }

    #[test]
    fn equal_timestamps_are_ranked_deterministically_by_submission_id() {
        let plan = build(
            vec![submission(20, 10, 2, 100), submission(10, 10, 1, 100)],
            vec![challenge(10, 0)],
            Vec::new(),
        )
        .unwrap();

        assert_eq!(plan.submissions[0].id, 10);
        assert_eq!(plan.submissions[0].rank, 1);
        assert_eq!(plan.submissions[1].id, 20);
        assert_eq!(plan.submissions[1].rank, 2);
    }

    #[test]
    fn unchanged_values_are_not_written_again() {
        let base = math::curve(1_000, 100, 10, 1);
        let plan = build(
            vec![SubmissionScoreInput {
                id: 1,
                challenge_id: 10,
                team_id: Some(1),
                created_at: 100,
                pts: base * 110 / 100,
                rank: 1,
            }],
            vec![ChallengeScoreInput {
                pts: base * 105 / 100,
                ..challenge(10, 0)
            }],
            vec![TeamScoreInput {
                id: 1,
                pts: base * 110 / 100,
                rank: 1,
            }],
        )
        .unwrap();

        assert_eq!(plan, ScorePlan::default());
    }

    #[test]
    fn teams_without_solves_receive_stable_id_tiebreak_ranks() {
        let plan = build(
            Vec::new(),
            vec![challenge(10, 0)],
            vec![
                TeamScoreInput {
                    id: 2,
                    pts: 1,
                    rank: 1,
                },
                TeamScoreInput {
                    id: 1,
                    pts: 1,
                    rank: 2,
                },
            ],
        )
        .unwrap();

        assert_eq!(plan.teams[0].id, 1);
        assert_eq!(plan.teams[0].pts, 0);
        assert_eq!(plan.teams[0].rank, 1);
        assert_eq!(plan.teams[1].id, 2);
        assert_eq!(plan.teams[1].rank, 2);
    }

    #[test]
    fn rejects_submission_without_game_challenge_configuration() {
        let error = build(vec![submission(1, 99, 1, 100)], Vec::new(), Vec::new()).unwrap_err();

        assert!(error.to_string().contains("challenge 99"));
    }

    #[test]
    fn large_multi_challenge_scenario_is_complete_and_deterministic() {
        let challenges = (1..=20)
            .map(|challenge_id| challenge(challenge_id, 0))
            .collect::<Vec<_>>();
        let submissions = (1..=2_000)
            .map(|id| submission(id, (id - 1) % 20 + 1, (id - 1) % 100 + 1, 1_000 + id / 20))
            .collect::<Vec<_>>();
        let teams = (1..=100)
            .map(|id| TeamScoreInput {
                id,
                pts: 0,
                rank: 0,
            })
            .collect::<Vec<_>>();

        let first = build(submissions.clone(), challenges.clone(), teams.clone()).unwrap();
        let second = build(submissions, challenges, teams).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.submissions.len(), 2_000);
        assert_eq!(first.challenges.len(), 20);
        assert_eq!(first.teams.len(), 100);
        assert_eq!(
            first.teams.iter().map(|team| team.rank).collect::<Vec<_>>(),
            (1..=100).collect::<Vec<_>>()
        );
    }
}

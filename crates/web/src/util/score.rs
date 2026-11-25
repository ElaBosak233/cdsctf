//! Score hydration and rank-sorted team pagination helpers backed by
//! [`cds_score::Score`].

use std::collections::{HashMap, HashSet};

use cds_db::{
    AdminTeamView, DB, GameChallengeSummary, GameChallengeView, SubmissionView, TeamView,
    submission::Status,
    team::{FindTeamOptions, State as TeamState},
};
use cds_score::{GameScoreSnapshot, Score};
use futures_util::future::try_join_all;

use crate::traits::WebError;

/// Hydrates derived `(pts, rank)` on a page of submissions from per-game CQRS
/// snapshots. Only `Correct` in-game submissions receive non-zero scores.
pub async fn hydrate_submissions(
    score: &Score,
    submissions: &mut [SubmissionView],
) -> Result<(), WebError> {
    let game_ids: Vec<i64> = submissions
        .iter()
        .filter(|s| s.status == Status::Correct)
        .filter_map(|s| s.game_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    if game_ids.is_empty() {
        return Ok(());
    }

    let fetched = try_join_all(
        game_ids
            .into_iter()
            .map(|game_id| async move { Ok::<_, anyhow::Error>((game_id, score.snapshot(game_id).await?)) }),
    )
    .await?;
    let snapshots: HashMap<i64, GameScoreSnapshot> = fetched.into_iter().collect();

    for submission in submissions {
        if submission.status == Status::Correct
            && let Some(game_id) = submission.game_id
            && let Some(snap) = snapshots.get(&game_id)
        {
            let (pts, rank) = snap.submission_score(submission.id);
            submission.pts = pts;
            submission.rank = rank;
        }
    }
    Ok(())
}

/// Hydrates derived `(pts, rank)` on a single team view.
pub async fn hydrate_team(score: &Score, team: &mut TeamView) -> Result<(), WebError> {
    if team.state != TeamState::Passed {
        team.pts = 0;
        team.rank = 0;
        return Ok(());
    }
    let snap = score.snapshot(team.game_id).await?;
    let (pts, rank) = snap.team_score(team.id);
    team.pts = pts;
    team.rank = rank;
    Ok(())
}

/// Hydrates derived `pts` on a list of [`GameChallengeView`] rows for one game.
pub async fn hydrate_game_challenges(
    score: &Score,
    game_id: i64,
    challenges: &mut [GameChallengeView],
) -> Result<(), WebError> {
    if challenges.is_empty() {
        return Ok(());
    }
    let snap = score.snapshot(game_id).await?;
    for challenge in challenges {
        challenge.pts = snap.challenge_pts(challenge.challenge_id);
    }
    Ok(())
}

/// Hydrates derived `pts` on a list of [`GameChallengeSummary`] rows for one
/// game.
pub async fn hydrate_game_challenge_summaries(
    score: &Score,
    game_id: i64,
    challenges: &mut [GameChallengeSummary],
) -> Result<(), WebError> {
    if challenges.is_empty() {
        return Ok(());
    }
    let snap = score.snapshot(game_id).await?;
    for challenge in challenges {
        challenge.pts = snap.challenge_pts(challenge.challenge_id);
    }
    Ok(())
}

/// Recognizes whether a `sorts` query parameter requests ordering by derived
/// `rank` or `pts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DerivedTeamSort {
    RankAsc,
    RankDesc,
    PtsAsc,
    PtsDesc,
}

pub fn parse_derived_team_sort(sorts: Option<&str>) -> Option<DerivedTeamSort> {
    let first = sorts?.split(',').next()?.trim();
    match first {
        "rank" => Some(DerivedTeamSort::RankAsc),
        "-rank" => Some(DerivedTeamSort::RankDesc),
        "pts" => Some(DerivedTeamSort::PtsAsc),
        "-pts" => Some(DerivedTeamSort::PtsDesc),
        _ => None,
    }
}

/// Lists admin teams with full filtering, pagination, and member loading,
/// supporting derived `rank` / `pts` sorting seamlessly even for thousands of
/// teams.
pub async fn list_admin_teams(
    db: &DB,
    score: &Score,
    game_id: i64,
    options: FindTeamOptions,
) -> Result<(Vec<AdminTeamView>, u64), WebError> {
    let snap = score.snapshot(game_id).await?;

    let Some(sort_mode) = parse_derived_team_sort(options.sorts.as_deref()) else {
        let (mut teams, total) = cds_db::team::find_with_users(&db.conn, options).await?;
        hydrate_admin_teams(&mut teams, &snap);
        return Ok((teams, total));
    };

    // Unified filter-then-rank path: fetch matching IDs via PostgreSQL indexes,
    // order them in memory against the score snapshot (< 1ms for 5k teams),
    // slice the requested page, then load full profiles only for that page.
    let mut matched_ids = cds_db::team::find_ids(&db.conn, &options).await?;
    let total = matched_ids.len() as u64;
    sort_team_ids_by_snapshot(&mut matched_ids, &snap, sort_mode);

    let page_ids = slice_page(&matched_ids, options.page, options.size);
    let mut teams = cds_db::team::find_with_users_by_ids(&db.conn, game_id, page_ids).await?;
    hydrate_admin_teams(&mut teams, &snap);
    Ok((teams, total))
}

fn hydrate_admin_teams(teams: &mut [AdminTeamView], snap: &GameScoreSnapshot) {
    for entry in teams {
        if entry.team.state == TeamState::Passed {
            let (pts, rank) = snap.team_score(entry.team.id);
            entry.team.pts = pts;
            entry.team.rank = rank;
        }
    }
}

fn slice_page<T>(items: &[T], page: Option<u64>, size: Option<u64>) -> &[T] {
    match (page, size) {
        (Some(_), Some(0)) => &[],
        (Some(page), Some(size)) => {
            let start = page.saturating_sub(1).saturating_mul(size) as usize;
            if start >= items.len() {
                &[]
            } else {
                let end = start.saturating_add(size as usize).min(items.len());
                &items[start..end]
            }
        }
        _ => items,
    }
}

fn sort_team_ids_by_snapshot(
    ids: &mut [i64],
    snap: &GameScoreSnapshot,
    sort_mode: DerivedTeamSort,
) {
    ids.sort_unstable_by(|&a, &b| {
        let (a_pts, a_rank) = snap.team_score(a);
        let (b_pts, b_rank) = snap.team_score(b);
        let a_unranked = a_rank == 0;
        let b_unranked = b_rank == 0;

        match sort_mode {
            DerivedTeamSort::RankAsc | DerivedTeamSort::PtsDesc => a_unranked
                .cmp(&b_unranked)
                .then_with(|| a_rank.cmp(&b_rank))
                .then_with(|| b_pts.cmp(&a_pts))
                .then_with(|| a.cmp(&b)),
            DerivedTeamSort::RankDesc | DerivedTeamSort::PtsAsc => b_unranked
                .cmp(&a_unranked)
                .then_with(|| b_rank.cmp(&a_rank))
                .then_with(|| a_pts.cmp(&b_pts))
                .then_with(|| a.cmp(&b)),
        }
    });
}

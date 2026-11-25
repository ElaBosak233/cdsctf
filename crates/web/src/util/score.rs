//! Score hydration and rank-sorted team pagination helpers backed by
//! [`cds_score::ScoreService`].

use std::collections::{HashMap, HashSet};

use cds_db::{
    AdminTeamView, DB, GameChallengeSummary, GameChallengeView, SubmissionView, TeamView,
    submission::Status,
    team::{FindTeamOptions, State as TeamState},
};
use cds_score::{GameScoreSnapshot, ScoreService};

use crate::traits::WebError;

/// Hydrates derived `(pts, rank)` on a page of submissions from per-game CQRS
/// snapshots. Only `Correct` in-game submissions receive non-zero scores.
pub async fn hydrate_submissions(
    score: &ScoreService,
    submissions: &mut [SubmissionView],
) -> Result<(), WebError> {
    let game_ids: HashSet<i64> = submissions
        .iter()
        .filter(|s| s.status == Status::Correct)
        .filter_map(|s| s.game_id)
        .collect();
    if game_ids.is_empty() {
        return Ok(());
    }

    let mut snapshots: HashMap<i64, std::sync::Arc<GameScoreSnapshot>> =
        HashMap::with_capacity(game_ids.len());
    for game_id in game_ids {
        snapshots.insert(game_id, score.snapshot(game_id).await?);
    }

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
pub async fn hydrate_team(score: &ScoreService, team: &mut TeamView) -> Result<(), WebError> {
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
    score: &ScoreService,
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
    score: &ScoreService,
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
    score: &ScoreService,
    game_id: i64,
    options: FindTeamOptions,
) -> Result<(Vec<AdminTeamView>, u64), WebError> {
    let snap = score.snapshot(game_id).await?;

    let Some(sort_mode) = parse_derived_team_sort(options.sorts.as_deref()) else {
        let (mut teams, total) = cds_db::team::find_with_users(&db.conn, options).await?;
        for entry in &mut teams {
            if entry.team.state == TeamState::Passed {
                let (pts, rank) = snap.team_score(entry.team.id);
                entry.team.pts = pts;
                entry.team.rank = rank;
            }
        }
        return Ok((teams, total));
    };

    let has_narrow_filter = options.id.is_some()
        || options.name.is_some()
        || options.has_writeup.is_some()
        || options.user_id.is_some();

    let (page_ids, total) = if !has_narrow_filter
        && matches!(options.state, None | Some(TeamState::Passed))
    {
        // Fast O(1) path: slice pre-ranked IDs directly from the snapshot,
        // combining with unranked teams only when `state` is unfiltered.
        let ranked_len = snap.ranked_teams.len() as u64;
        let unranked_len = if options.state.is_none() {
            cds_db::team::count_unranked(&db.conn, game_id).await?
        } else {
            0
        };
        let total = ranked_len + unranked_len;
        let (offset, limit) = match (options.page, options.size) {
            (Some(_), Some(0)) => (0, 0),
            (Some(page), Some(size)) => (page.saturating_sub(1).saturating_mul(size), size),
            _ => (0, total),
        };

        let page_ids = slice_ranked_and_unranked_ids(
            db,
            game_id,
            &snap,
            unranked_len,
            sort_mode,
            offset,
            limit,
        )
        .await?;
        (page_ids, total)
    } else {
        // Filter-then-rank path: fetch only matching team IDs from PostgreSQL,
        // sort `(rank/pts, id)` in memory (< 1ms for 5k teams), and slice.
        let mut matched_ids = cds_db::team::find_ids(&db.conn, &options).await?;
        let total = matched_ids.len() as u64;
        sort_team_ids_by_snapshot(&mut matched_ids, &snap, sort_mode);

        let page_ids = match (options.page, options.size) {
            (Some(_), Some(0)) => Vec::new(),
            (Some(page), Some(size)) => {
                let start = page.saturating_sub(1).saturating_mul(size) as usize;
                if start >= matched_ids.len() {
                    Vec::new()
                } else {
                    let end = start.saturating_add(size as usize).min(matched_ids.len());
                    matched_ids[start..end].to_vec()
                }
            }
            _ => matched_ids,
        };
        (page_ids, total)
    };

    let mut teams = cds_db::team::find_with_users_by_ids(&db.conn, game_id, &page_ids).await?;
    for entry in &mut teams {
        if entry.team.state == TeamState::Passed {
            let (pts, rank) = snap.team_score(entry.team.id);
            entry.team.pts = pts;
            entry.team.rank = rank;
        }
    }
    Ok((teams, total))
}

async fn slice_ranked_and_unranked_ids(
    db: &DB,
    game_id: i64,
    snap: &GameScoreSnapshot,
    unranked_len: u64,
    sort_mode: DerivedTeamSort,
    offset: u64,
    limit: u64,
) -> Result<Vec<i64>, WebError> {
    if limit == 0 {
        return Ok(Vec::new());
    }
    let ranked_len = snap.ranked_teams.len() as u64;

    // Place ranked Passed teams (`rank 1..=N`) ahead of unranked teams for
    // `rank` / `-pts` so #1 always appears first; reverse for `-rank` / `pts`.
    let ranked_first = matches!(
        sort_mode,
        DerivedTeamSort::RankAsc | DerivedTeamSort::PtsDesc
    );
    let reverse_ranked = matches!(
        sort_mode,
        DerivedTeamSort::RankDesc | DerivedTeamSort::PtsAsc
    );

    let slice_ranked = |start: u64, take: u64| -> Vec<i64> {
        if take == 0 || start >= ranked_len {
            return Vec::new();
        }
        let s = start as usize;
        let e = (start.saturating_add(take)).min(ranked_len) as usize;
        if reverse_ranked {
            let total = snap.ranked_teams.len();
            (s..e)
                .map(|i| snap.ranked_teams[total - 1 - i].team_id)
                .collect()
        } else {
            snap.ranked_teams[s..e].iter().map(|t| t.team_id).collect()
        }
    };

    if ranked_first {
        let mut ids = Vec::with_capacity(limit as usize);
        if offset < ranked_len {
            let take = limit.min(ranked_len - offset);
            ids.extend(slice_ranked(offset, take));
        }
        let remaining = limit.saturating_sub(ids.len() as u64);
        if remaining > 0 && unranked_len > 0 {
            let unranked_offset = offset.saturating_sub(ranked_len);
            ids.extend(
                cds_db::team::find_unranked_ids_page(&db.conn, game_id, unranked_offset, remaining)
                    .await?,
            );
        }
        Ok(ids)
    } else {
        let mut ids = Vec::with_capacity(limit as usize);
        if offset < unranked_len {
            let take = limit.min(unranked_len - offset);
            ids.extend(
                cds_db::team::find_unranked_ids_page(&db.conn, game_id, offset, take).await?,
            );
        }
        let remaining = limit.saturating_sub(ids.len() as u64);
        if remaining > 0 {
            let ranked_offset = offset.saturating_sub(unranked_len);
            ids.extend(slice_ranked(ranked_offset, remaining));
        }
        Ok(ids)
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

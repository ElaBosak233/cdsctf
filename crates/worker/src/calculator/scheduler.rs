//! Per-game single-flight state used by the calculator queue consumer.

use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ScheduleAction {
    Start,
    Coalesced,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Completion<M> {
    pub acknowledged: Vec<M>,
    pub discarded: Vec<M>,
    pub rerun: bool,
}

#[derive(Debug)]
struct Job<M> {
    active: Vec<M>,
    queued: Vec<M>,
}

#[derive(Debug)]
pub(super) struct Scheduler<K, M> {
    jobs: HashMap<K, Job<M>>,
}

impl<K, M> Default for Scheduler<K, M> {
    fn default() -> Self {
        Self {
            jobs: HashMap::new(),
        }
    }
}

impl<K, M> Scheduler<K, M>
where
    K: Clone + Eq + Hash,
{
    /// Enqueues a message and reports whether a worker should start now.
    pub fn schedule(&mut self, key: K, message: M) -> ScheduleAction {
        if let Some(job) = self.jobs.get_mut(&key) {
            job.queued.push(message);
            ScheduleAction::Coalesced
        } else {
            self.jobs.insert(
                key,
                Job {
                    active: vec![message],
                    queued: Vec::new(),
                },
            );
            ScheduleAction::Start
        }
    }

    /// Finishes one run. Messages received during it become one rerun batch.
    pub fn complete(&mut self, key: &K, succeeded: bool) -> Option<Completion<M>> {
        let mut job = self.jobs.remove(key)?;

        if !succeeded {
            job.active.append(&mut job.queued);
            return Some(Completion {
                acknowledged: Vec::new(),
                discarded: job.active,
                rerun: false,
            });
        }

        let acknowledged = job.active;
        let rerun = !job.queued.is_empty();
        if rerun {
            self.jobs.insert(
                key.clone(),
                Job {
                    active: job.queued,
                    queued: Vec::new(),
                },
            );
        }

        Some(Completion {
            acknowledged,
            discarded: Vec::new(),
            rerun,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_game_messages_become_exactly_one_rerun() {
        let mut scheduler = Scheduler::default();
        assert_eq!(scheduler.schedule(Some(1), "first"), ScheduleAction::Start);
        assert_eq!(
            scheduler.schedule(Some(1), "second"),
            ScheduleAction::Coalesced
        );
        assert_eq!(
            scheduler.schedule(Some(1), "third"),
            ScheduleAction::Coalesced
        );

        let first = scheduler.complete(&Some(1), true).unwrap();
        assert_eq!(first.acknowledged, vec!["first"]);
        assert!(first.rerun);

        let second = scheduler.complete(&Some(1), true).unwrap();
        assert_eq!(second.acknowledged, vec!["second", "third"]);
        assert!(!second.rerun);
    }

    #[test]
    fn different_games_start_independently() {
        let mut scheduler = Scheduler::default();
        assert_eq!(scheduler.schedule(Some(1), 1), ScheduleAction::Start);
        assert_eq!(scheduler.schedule(Some(2), 2), ScheduleAction::Start);
    }

    #[test]
    fn failed_run_acknowledges_nothing_and_releases_the_key() {
        let mut scheduler = Scheduler::default();
        scheduler.schedule(Some(1), "first");
        scheduler.schedule(Some(1), "second");

        let completion = scheduler.complete(&Some(1), false).unwrap();
        assert!(completion.acknowledged.is_empty());
        assert_eq!(completion.discarded, vec!["first", "second"]);
        assert!(!completion.rerun);
        assert_eq!(scheduler.schedule(Some(1), "retry"), ScheduleAction::Start);
    }

    #[test]
    fn full_rebuild_messages_are_coalesced_separately() {
        let mut scheduler = Scheduler::default();
        assert_eq!(scheduler.schedule(None, 1), ScheduleAction::Start);
        assert_eq!(scheduler.schedule(Some(1), 2), ScheduleAction::Start);
        assert_eq!(scheduler.schedule(None, 3), ScheduleAction::Coalesced);
    }
}

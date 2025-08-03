#![allow(dead_code)]

use rand::{Rng, rng};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub term: u64,
    pub command: Command,
}

#[derive(Clone, Debug)]
pub enum Command {
    NoOp,
    Set(String, String),
    Delete(String),
}

#[derive(Debug)]
pub struct ConsensusState {
    pub current_term: u64,
    pub voted_for: Option<String>,
    pub last_applied: u64,
    pub log: Vec<LogEntry>,
    pub commit_index: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug)]
pub enum ConsensusEvent {
    ElectionTimeout,
    HeartbeatTimeout,
    ReceiveAppendEntries,
    ReceiveVoteGranted,
    ReceiveVoteRejected,
}

pub fn random_election_timeout() -> Duration {
    let millis = rng().random_range(150..=300);
    Duration::from_millis(millis)
}

pub const HEARTBEAT_INTERVAL: Duration = Duration::from_millis(50);

impl ConsensusState {
    pub fn new() -> Self {
        Self {
            current_term: 0,
            voted_for: None,
            log: Vec::new(),
            commit_index: 0,
            last_applied: 0,
        }
    }

    pub fn append_entry(&mut self, term: u64, command: Command) {
        self.log.push(LogEntry { term, command });
    }
}

impl Default for ConsensusState {
    fn default() -> Self {
        Self::new()
    }
}

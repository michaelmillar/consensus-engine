#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub term: u64,
    pub command: Command,
}

#[derive(Clone, Debug)]
pub enum Command {
    NoOp,
    Set(String, String),
    Delete(String)
}

#[derive(Debug)]
pub struct ConsensusState {
    pub current_term: u64,
    pub voted_for: Option<String>,
    pub last_applied: u64,
    pub log: Vec<LogEntry>,
    pub commit_index: u64
}

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

    pub fn append_entry(&mut self, term:u64, command: Command) {
        self.log.push(LogEntry {term , command});
    }
}

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
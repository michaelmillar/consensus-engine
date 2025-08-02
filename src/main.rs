fn main() {
    println!("Hello, world!");
}

struct LogPair {
    term: String,
    command: String,
}

struct ConsensusState {
    current_term: i64,
    voted_for: String,
    last_applied: i64,
    log: Vec<LogPair>,
    commit_index: i64
}
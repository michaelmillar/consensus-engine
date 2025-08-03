mod consensus_engine {

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
}

fn main() {

}

/*fn main() {
    let mut state = ConsensusState::new();

    state.append_entry(1, Command::Set("ky 1".to_string(), "val 1".to_string()));
    state.append_entry(1, Command::NoOp);
    state.append_entry(2, Command::Delete("ky 1".to_string()));

    println!("Current term: {}", state.current_term);
    println!("Voted for: {:?}", state.voted_for);
    println!("Log: {:#?}", state.log);
}*/

use consensus_engine::*;


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_log_append() {
        let mut state = ConsensusState::new();
        state.append_entry(1, Command::Set("k".to_string(), "v".to_string()));
        assert_eq!(state.log.len(), 1);
        assert!(matches!(state.log[0].command, Command::Set(_, _)));
    }
}

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry {
            term: 1,
            command: Command::Set("key 1".to_string(), "val 1".to_string()),
        };

        assert_eq!(entry.term, 1);
        match entry.command {
            Command::Set(key, value) => {
                assert_eq!(key, "key 1");
                assert_eq!(value, "val 1");
            }
            _ => panic!("Expected Set command"), 
        }
    }
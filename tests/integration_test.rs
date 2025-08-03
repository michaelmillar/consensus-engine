use consensus_engine::{ConsensusState, Command, LogEntry};


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
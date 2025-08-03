use consensus_engine::{Command, ConsensusState, LogEntry};

fn main() {
    let mut state = ConsensusState::new();

    state.append_entry(1, Command::Set("ky 1".to_string(), "val 1".to_string()));
    state.append_entry(1, Command::NoOp);
    state.append_entry(2, Command::Delete("ky 1".to_string()));

    println!("Current term: {}", state.current_term);
    println!("Voted for: {:?}", state.voted_for);
    println!("Log: {:#?}", state.log);
}

use tokio::time::{sleep, Instant}

pub struct ConsensusNode {
    pub id: String,
    pub role: Role,
    pub state: ConsensusState,
    pub peers: Vec<String>,
    pub event_rx: tokio::sync::mpsc::UnboundedReceiver<ConsensusEvent>,
    pub event_tx: tokio::sync::mpsc::UnboundedSender<ConsensusEvent>,
}

impl ConsensusNode {
    let timeout = random_election_timeout();
    let deadline = Instant::now() + timeout;

    loop {
        tokio::select! {
            Some(event) = self.event_rx.recv() => {
                match event {
                    ConsensusEvent::ReceiveAppendEntries => {
                        return;
                    }
                    ConsensusEvent::ElectionTimeout => {
                        self.role = Role::Candidate;
                        return;
                    }
                    _ => {}
                }
            }
            _ = sleep_until(deadline) => {
                self.role = Role::Candidate;
                return;
            }
        }
    }
}
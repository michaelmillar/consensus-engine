# Consensus Engine

## Logic WIP
- [ ] Event Loop 
    - [ ] Roles: Follower, Candidate, Leader
  - [ ] Timers
    - [ ] Election timeout
    - [ ] Heartbeat interval
  - [ ] Event-driven transitions
    - [ ] timeout: follower -> candidate
    - [ ] election win: candidate -> leader
    - [ ] receive heartbeat: candidate/follower -> follower
  
## Testing

```bash

cargo test

cargo llvm-cov --html --open
```
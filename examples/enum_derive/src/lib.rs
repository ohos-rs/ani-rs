//! Derived ANI enum example.

use ani_derive::{ani, AniEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, AniEnum)]
#[ani(name = "Status")]
pub enum Status {
    Idle = 0,
    Running = 2,
    Stopped,
}

#[ani]
pub fn next_status(status: Status) -> Status {
    match status {
        Status::Idle => Status::Running,
        Status::Running => Status::Stopped,
        Status::Stopped => Status::Idle,
    }
}

#[ani]
pub fn status_name(status: Status) -> String {
    match status {
        Status::Idle => "idle".to_string(),
        Status::Running => "running".to_string(),
        Status::Stopped => "stopped".to_string(),
    }
}

#[ani]
pub fn is_terminal(status: Status) -> bool {
    matches!(status, Status::Stopped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_flow_works() {
        assert_eq!(next_status(Status::Idle), Status::Running);
        assert_eq!(next_status(Status::Running), Status::Stopped);
        assert_eq!(status_name(Status::Stopped), "stopped");
        assert!(is_terminal(Status::Stopped));
        assert!(!is_terminal(Status::Idle));
    }
}

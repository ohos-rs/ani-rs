//! Derived ANI enum example.

use ani_derive::{ani, AniEnum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, AniEnum)]
#[ani(name = "Status")]
pub enum Status {
    Idle = 0,
    Running = 2,
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AniEnum)]
pub enum Message {
    Text(String),
    Point { x: i32, y: i32 },
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AniEnum)]
#[ani(discriminator = "kind", case = "camelCase")]
pub enum DirectionalMessage {
    #[ani(rename = "legacyText", input_only)]
    Legacy(String),
    RenamedField {
        #[ani(rename = "payloadText")]
        payload: String,
        #[ani(skip)]
        #[serde(default)]
        local_only: bool,
    },
    #[ani(output_only)]
    Generated(i32),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AniEnum)]
pub enum Envelope<T> {
    Value(T),
    Empty,
}

#[ani]
pub fn message_identity(message: Message) -> Message {
    message
}

#[ani]
pub fn message_point_sum(message: Message) -> i32 {
    match message {
        Message::Point { x, y } => x + y,
        Message::Text(text) => text.len() as i32,
        Message::Empty => 0,
    }
}

#[ani]
pub fn directional_input(message: DirectionalMessage) -> String {
    match message {
        DirectionalMessage::Legacy(value) => value,
        DirectionalMessage::RenamedField { payload, .. } => payload,
        DirectionalMessage::Generated(value) => value.to_string(),
    }
}

#[ani]
pub fn directional_output(value: i32) -> DirectionalMessage {
    DirectionalMessage::Generated(value)
}

#[ani]
pub fn generic_envelope_identity(value: Envelope<i32>) -> Envelope<i32> {
    value
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
        assert_eq!(
            message_identity(Message::Point { x: 3, y: 4 }),
            Message::Point { x: 3, y: 4 }
        );
    }
}

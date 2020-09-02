use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub enum MessageType {
    METADATA,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Message {
    pub typ: MessageType,
    pub content: [u8; 32],
}

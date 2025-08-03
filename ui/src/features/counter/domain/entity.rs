use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Counter {
    pub value: i32,
}

impl Counter {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

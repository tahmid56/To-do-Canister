use candid::CandidType;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, CandidType)]
pub struct TodoItem{
    pub id: i32,
    pub description: String,
    pub completed: bool,
}
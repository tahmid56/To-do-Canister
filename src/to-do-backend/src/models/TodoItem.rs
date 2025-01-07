use candid::{CandidType, Int};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, CandidType)]
pub struct TodoItem{
    pub id: Int,
    pub description: String,
    pub completed: bool,
}
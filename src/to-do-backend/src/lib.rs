mod models;


use std::collections::HashMap;

use candid::CandidType;
use ic_cdk::{query, storage, update};
use models::TodoItem::TodoItem;
use serde::Deserialize;
use candid::Int;

#[derive(CandidType, Default, Deserialize)]
struct TodoList{
    items: HashMap<Int, TodoItem>,
    next_id: Int,
}

#[update]
fn add_todo(description: String) -> Int {
    let (mut state,): (TodoList,) = storage::stable_restore().unwrap_or_default();
    let id = state.next_id.clone();
    let todo = TodoItem {
        id: id.clone(),
        description,
        completed: false,
    };
    state.items.insert(id.clone(), todo);
    state.next_id += 1;

    // Save the updated state back to stable storage
    storage::stable_save((state,)).unwrap();

    id
}


#[query]
fn get_all_todo_list() -> Vec<TodoItem> {
    let (state,): (TodoList,) = storage::stable_restore().unwrap_or_default();
    state.items.values().cloned().collect()
}


#[update]
fn update_todo(id: Int, description: String, completed: bool) {
    let (mut state,): (TodoList,) = storage::stable_restore().unwrap();
    if let Some(todo) = state.items.get_mut(&id) {
        todo.description = description;
        todo.completed = completed;
    }
    storage::stable_save((state,)).unwrap();
}

#[update]
fn delete_todo(id: Int) {
    let (mut state,): (TodoList,) = storage::stable_restore().unwrap();
    state.items.remove(&id);
    storage::stable_save((state,)).unwrap();
}
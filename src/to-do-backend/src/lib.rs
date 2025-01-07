mod models;


use std::collections::HashMap;

use candid::CandidType;
use ic_cdk::{query, storage, update};
use models::TodoItem::TodoItem;
use serde::Deserialize;


#[derive(CandidType, Default, Deserialize)]
struct TodoList{
    items: HashMap<i32, TodoItem>,
    next_id: i32,
}

#[update]
fn add_todo(description: String) -> i32 {
    let (mut state,): (TodoList,) = storage::stable_restore().unwrap();
    let id = state.next_id;
    let todo = TodoItem {
        id,
        description,
        completed: false,
    };
    state.items.insert(id, todo);
    state.next_id += 1;

    // Save the updated state back to stable storage
    storage::stable_save((state,)).unwrap();

    id
}


#[query]
fn get_all_todo_list() -> Vec<TodoItem> {
    let (state,): (TodoList,) = storage::stable_restore().unwrap();
    state.items.values().cloned().collect()
}


#[update]
fn update_todo(id: i32, description: String, completed: bool) {
    let (mut state,): (TodoList,) = storage::stable_restore().unwrap();
    if let Some(todo) = state.items.get_mut(&id) {
        todo.description = description;
        todo.completed = completed;
    }
    storage::stable_save((state,)).unwrap();
}

#[update]
fn delete_todo(id: i32) {
    let (mut state,): (TodoList,) = storage::stable_restore().unwrap();
    state.items.remove(&id);
    storage::stable_save((state,)).unwrap();
}
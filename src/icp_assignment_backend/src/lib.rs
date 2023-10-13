use serde::Serialize;

use ic_cdk::{
    export::candid::{CandidType, Deserialize},
    query, update,
};
use std::cell::RefCell;
use std::collections::BTreeMap;

type TodoStore = BTreeMap<String, Todo>;

#[derive(Clone, Debug, Default, CandidType, Deserialize, Serialize)]
pub struct Todo {
    pub title: String,
    pub content: String,
    pub completed: String
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoSchema {
    pub title: Option<String>,
    pub content: Option<String>,
}

thread_local! {
    static TODO_STORE: RefCell<TodoStore> = RefCell::default();
}

#[ic_cdk_macros::query]
fn get(title: String) -> Result<Todo, String> {

        let todo_data = TODO_STORE.with(|todo_store| todo_store.borrow().get(&title).cloned());
        if todo_data.is_none() {
            return Err("Invalid title id for Todo".to_string());
        }
        return Ok(todo_data.unwrap());

}

#[update]
fn insert(todo: Todo) -> Result<String, String> {

    TODO_STORE.with(|todo_store| {

        let mut todo_map =  todo_store.borrow_mut();
        let td = todo_map.get(&todo.title).clone();

        if td.is_none() {
            let new_todo = Todo{
                title:todo.title, 
                content:todo.content,
                completed: todo.completed 
            };
        
            todo_map.insert(new_todo.title.to_string(), new_todo);
            return Ok("Todo task created successfully".to_string());
        }

        return Err("Todo list already exist for following title".to_string());  
    })
}

#[update]
fn update(title: String, todo: Todo) -> Result<String, String> {

    TODO_STORE.with(|todo_store| {
        let mut todo_map = todo_store.borrow_mut();
        let td = todo_map.get(&title).clone();
        if td.is_some() {
            if !title.eq(&todo.title) {
                return Err("Title of Todo cannot be updated".to_string());
            }

            let new_todo = Todo{
                title: todo.title, 
                content: todo.content,
                completed: todo.completed
                };

            todo_map.insert(title, new_todo);
            return Ok("Todo item updated successfully".to_string());
        }
        return Err("No Todo item exist for this title".to_string());

    })
}

#[update]
fn delete_entry(title: String) -> Result<String, String> {

    TODO_STORE.with(|todo_store| {
        let mut todo_map = todo_store.borrow_mut();
        let td = todo_map.get(&title).clone();
        if td.is_some() {
            let _removed_todo = todo_map.remove(&title);
            return Ok("Todo item removed successfully".to_string());
        }
        return Err("No Todo item exist for this title".to_string());

    })
}


#[query]
fn get_all_todos() -> Result<Vec<Todo>, String> {

    TODO_STORE.with(|todo_store| {
        let todo_map = todo_store.borrow_mut();
        if todo_map.to_owned().is_empty() {
            return Err("Empty Todo List".to_string());
        }
        let mut todo_vec: Vec<Todo> = Vec::new();
        for (_key, value) in todo_map.to_owned().iter() {
            todo_vec.push(value.to_owned());
        }
        return Ok(todo_vec);  
    })
}

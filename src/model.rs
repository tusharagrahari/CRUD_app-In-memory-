use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}


pub struct AppState {
    pub todo_db: Arc<Mutex<Vec<Todo>>>, // This is a vector of Todo items
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            todo_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}


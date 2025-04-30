use actix_web::{delete, get, patch, post, web::{self, Data}, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;

use crate::{model::{QueryOptions, Todo, UpdateTodo}, response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse}, AppState};

#[get("/health_check")]
pub async fn health_check_handler() -> impl Responder {
    let response = GenericResponse {
        status: "success".to_string(),
        message: "Server is running".to_string(),
    };
    HttpResponse::Ok().json(response)
}


#[get("/todos")]
pub async fn get_todos_handler(opts: web::Query<QueryOptions>, data: web::Data<AppState>) -> impl Responder {
    let todos = data.todo_db.lock().unwrap();
    let limit = opts.limit.unwrap_or(10);
    let page = (opts.page.unwrap_or(1)-1) * limit;

    let todos: Vec<Todo> = todos.clone().into_iter().skip(page).take(limit).collect();

    let response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };

    HttpResponse::Ok().json(response)
}

#[get("/todos/{id}")]
pub async fn get_todo_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let todos = data.todo_db.lock().unwrap();
    let todo = todos.iter().find(|todo| todo.id.as_ref() == Some(&id));

    match todo {
        Some(todo) => {
            let response = SingleTodoResponse {
                status: "success".to_string(),
                data: TodoData{ todo: todo.clone() },
            };
            HttpResponse::Ok().json(response)
       }
        None => {
            let response = GenericResponse {
                status: "error".to_string(),
                message: "Todo not found".to_string(),
            };
            HttpResponse::NotFound().json(response)
        }
    }
}


#[post("/todos")]
async fn create_todo_handler(
    mut body: web::Json<Todo>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut vec = data.todo_db.lock().unwrap();

    let todo = vec.iter().find(|todo| todo.title == body.title);

    if todo.is_some() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with title: '{}' already exists", body.title),
        };
        return HttpResponse::Conflict().json(error_response);
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.created_at = Some(datetime);
    body.updated_at = Some(datetime);

    let todo = body.to_owned();

    vec.push(body.into_inner());

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo },
    };

    HttpResponse::Ok().json(json_response)
}


#[patch("/todos/{id}")]
async fn update_todo_handler(path: web::Path<String>, body: web::Json<UpdateTodo>, data: web::Data<AppState>) -> 
    impl Responder {
        let mut vec = data.todo_db.lock().unwrap();

        let id = path.into_inner();
        let todo = vec.iter_mut().find(|todo| todo.id == Some(id.to_owned()));
    
        if todo.is_none() {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Todo with ID: {} not found", id),
            };
            return HttpResponse::NotFound().json(error_response);
        }
    
        let todo = todo.unwrap();
        let datetime = Utc::now();
        let payload = Todo {
            id: todo.id.to_owned(),
            title: body.title.to_owned().unwrap_or(todo.title.to_owned()),
            content: body.content.to_owned().unwrap_or(todo.content.to_owned()),
            completed: if body.completed.is_some() {
                body.completed
            } else {
                todo.completed
            },
            created_at: todo.created_at.to_owned(),
            updated_at: Some(datetime),
        };
        *todo = payload;
    
        let json_response = SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo: todo.clone() },
        };
    
        HttpResponse::Ok().json(json_response)
}


#[delete("/todos/{id}")]
async fn delete_todo_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut vec = data.todo_db.lock().unwrap();

    let id = path.into_inner();
    let todo = vec.iter_mut().find(|todo| todo.id == Some(id.to_owned()));

    if todo.is_none() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with ID: {} not found", id),
        };
        return HttpResponse::NotFound().json(error_response);
    }

    vec.retain(|todo| todo.id != Some(id.to_owned()));
    HttpResponse::NoContent().finish()
}


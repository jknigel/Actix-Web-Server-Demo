use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client as HttpClient;
use async_trait::async_trait;
use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    fn insert_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn get_task(&self, task_id: u64) -> Option<&Task> {
        return self.tasks.get(&task_id)
    }

    fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn delete_task(&mut self, task_id: u64) {
        self.tasks.remove(&task_id);
    }

    fn update_task(&mut self, updated_task: Task) {
        self.tasks.insert(updated_task.id, updated_task);
    }

    //USER DATA RELATED FUNCTIONS

    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|user| user.username == username)
    }

    fn get_user_by_id(&self, user_id: u64) -> Option<&User> {
        self.users.get(&user_id)
    }

    //DATABASE SAVING
    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let data:String = serde_json::to_string(&self);
        let mut file:fs::File = fs::File::create(filename)?;
        file.write_all(data.as_bytes())?;
        return Ok(())
    }

    fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let data:String = fs::read_to_string(filename)?;
        let db:Database = serde_json::from_str(&data)?;
        return Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>,
    http_client: HttpClient,
}

async fn create_task(app_state: web::Data<AppState>, new_task: web::Json<Task>) -> impl Responder {
    let mut db: Mutex<Database> = app_state.db.lock().unwrap();
    db.insert_task(new_task.into_inner());
    let _ = db.save_to_file("database.json");
    return HttpResponse::Ok().finish()
}

fn main() {
    println!("Hello, world!");
}

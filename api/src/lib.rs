use actix_example_service::{
    sea_orm::{Database, DatabaseConnection},
    types::UpdateTaskByIdRequest,
    types::UpdateTaskRequest,
    Mutation, Query,
};
// use actix_files::Files as Fs;
use actix_web::{
    delete, error, get, middleware, post, put, web, App, Error, HttpRequest, HttpResponse,
    HttpServer, Result,
};

use chrono::{Datelike, NaiveDate};
use chrono::{Local, Utc};
use chrono_tz::Canada::Mountain;
use cron::Schedule;
use entity::task;
use listenfd::ListenFd;
use migration::{sea_orm::prelude::Date, Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, env, str::FromStr, thread, time::Duration};
use tokio_cron_scheduler::{Job, JobScheduler};

// const DEFAULT_POSTS_PER_PAGE: u64 = 5;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct FlashData {
    kind: String,
    message: String,
}

// #[derive(Deserialize)]
// pub struct UpdateTaskRequest {
//     pub id: i32,
//     pub title: String,
//     pub date: Date,
//     pub recurring_option: Option<task::RecurringOption>,
//     pub is_completed: bool,
//     pub position: i32,
// }

#[derive(Deserialize)]
pub struct BulkUpdateTaskRequest {
    pub tasks: Vec<UpdateTaskRequest>, // A list of tasks to update
}

#[get("/tasks")]
async fn all(
    req: HttpRequest,
    data: web::Data<AppState>,
    query: web::Query<HashMap<String, String>>, // Accept query parameters
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    // Check if the "date" parameter is provided
    let tasks = if let Some(date_str) = query.get("date") {
        match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(date) => Query::find_tasks_by_date(conn, date).await, // Fetch tasks by date
            Err(_) => {
                return Err(error::ErrorBadRequest(
                    "Invalid date format. Use YYYY-MM-DD",
                ))
            }
        }
    } else {
        Query::find_all_tasks(conn).await // Fetch all tasks if no date is provided
    };

    let tasks = tasks.map_err(|_| error::ErrorInternalServerError("Failed to fetch posts"))?;

    Ok(HttpResponse::Ok().json(tasks))
}

#[get("/tasks/{id}")]
async fn get_task_by_id(
    data: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let task = Query::find_task_by_id(conn, id)
        .await
        .map_err(|_| error::ErrorInternalServerError("Failed to fetch posts"))?;

    match task {
        Some(task) => Ok(HttpResponse::Ok().json(task)), // Return the task if found
        None => Ok(HttpResponse::NotFound().body("Task not found")), // Return 404 if not found
    }
}

#[post("/tasks")]
async fn create_task(
    _: HttpRequest,
    data: web::Data<AppState>,
    json: web::Json<task::Model>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let new_task = json.into_inner();

    let inserted_task = Mutation::add_task(conn, new_task).await.map_err(|e| {
        println!("Error inserting task: {:?}", e);
        error::ErrorInternalServerError("Failed to insert task")
    })?;

    Ok(HttpResponse::Created().json(inserted_task))
}

#[put("/tasks/{id}")]
async fn update_task(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    json: web::Json<UpdateTaskByIdRequest>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let update_data = json.into_inner();

    let result = Mutation::update_task_by_id(
        conn,
        id,
        update_data.title,
        update_data.date,
        update_data.time,
        update_data.recurring_option,
        update_data.is_completed,
        update_data.position,
    )
    .await;

    match result {
        Ok(updated_post) => Ok(HttpResponse::Ok().json(updated_post)),
        Err(err) => Err(error::ErrorInternalServerError("Failed to update post")),
    }
}

#[put("/tasks")]
async fn update_tasks(
    data: web::Data<AppState>,
    json: web::Json<Vec<UpdateTaskRequest>>, // Expect a Vec of UpdateTaskRequest
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let updates = json.into_inner();

    let result = Mutation::update_tasks_bulk(conn, updates).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Tasks updated successfully"
        }))),
        Err(err) => {
            println!("Error updating tasks in bulk: {:?}", err);
            Err(error::ErrorInternalServerError("Failed to update tasks"))
        }
    }
}

#[put("/reset_tasks_due_today")]
async fn reset_due_tasks_handler(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    match Mutation::reset_due_tasks(conn).await {
        Ok(rows) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": format!("Updated {} tasks due today", rows)
        }))),
        Err(err) => {
            eprintln!("Error updating tasks: {:?}", err);
            Err(error::ErrorInternalServerError("Failed to update tasks"))
        }
    }
}

#[delete("/tasks/{id}")]
async fn delete_task(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let result = Mutation::delete_task_by_id(conn, id)
        .await
        .expect("could not delete post");

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Post deleted successfully"
    })))
}

// async fn not_found(data: web::Data<AppState>, request: HttpRequest) -> Result<HttpResponse, Error> {
//     let mut ctx = tera::Context::new();
//     ctx.insert("uri", request.uri().path());

//     let template = &data.templates;
//     let body = template
//         .render("error/404.html.tera", &ctx)
//         .map_err(|_| error::ErrorInternalServerError("Template error"))?;

//     Ok(HttpResponse::Ok().content_type("text/html").body(body))
// }

async fn scheduled_task() {
    println!("Running scheduled task at {}", Local::now());

    // Your task logic here (e.g., fetch and process tasks)
    println!("Task executed successfully.");
}

async fn start_scheduler() -> Result<(), Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;

    sched
        .add(Job::new_async_tz(
            "0 45 12 * * *",
            Mountain,
            |uuid, mut l| {
                Box::pin(async move {
                    println!("Running scheduled task at {}", chrono::Local::now());
                    // Query and print the next scheduled run time for this job.
                    match l.next_tick_for_job(uuid).await {
                        Ok(Some(ts)) => println!("Next scheduled run for job {}: {:?}", uuid, ts),
                        Ok(None) => println!("Job {} has no next scheduled run", uuid),
                        Err(e) => {
                            println!("Error getting next scheduled run for job {}: {:?}", uuid, e)
                        }
                    }
                    scheduled_task().await;
                })
            },
        )?)
        .await?;

    sched.start().await?;
    Ok(())
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // establish connection to database and apply migrations
    // -> create post table if not exists
    let conn = Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    tokio::spawn(async {
        if let Err(e) = start_scheduler().await {
            eprintln!("Scheduler failed: {:?}", e);
        }
    });

    // load tera templates and build app state
    // let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    // let state = AppState { templates, conn };
    let state = AppState { conn };

    // create server and try to serve over socket if possible
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .default_service(web::route().to(|| async { HttpResponse::NotFound().finish() }))
            .configure(init)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {server_url}");
    server.run().await?;

    Ok(())
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(all);
    cfg.service(get_task_by_id);
    cfg.service(create_task);
    cfg.service(update_task);
    cfg.service(update_tasks);
    cfg.service(reset_due_tasks_handler);
    cfg.service(delete_task);
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}

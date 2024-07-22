use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpServer, Responder};
use anyhow::Context;
use common::User;
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use tracing::info;
use tracing_actix_web::TracingLogger;

struct AppState {
    pool: PgPool,
}

#[derive(Serialize, FromRow)]
struct GetUser {
    id: i32,
    username: String,
}

impl Into<User> for GetUser {
    fn into(self) -> User {
        User {
            id: self.id as usize,
            name: self.username,
        }
    }
}

#[get("/test")]
async fn test() -> &'static str {
    "Hello world!"
}

#[post("/user")]
async fn create_user(state: web::Data<AppState>) -> &'static str {
    let pool = &state.pool;
    sqlx::query("INSERT INTO users (username) VALUES ($1)")
        .bind("Alice")
        .execute(pool)
        .await
        .expect("Failed to insert user");

    "User created!"
}

#[get("/users")]
async fn get_users(state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let pool = &state.pool;
    let users = sqlx::query_as::<_, GetUser>("SELECT * FROM users")
        .fetch_all(pool)
        .await
        .map_err(actix_web::error::ErrorBadRequest)?
        .into_iter()
        .map(Into::<User>::into)
        .collect::<Vec<_>>();

    Ok(web::Json(users))
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let pool = connect_db().await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { pool: pool.clone() }))
            .wrap(Cors::permissive())
            .wrap(TracingLogger::default())
            .service(test)
            .service(create_user)
            .service(get_users)
    })
    .bind(("0.0.0.0", 25565))?
    .run()
    .await
    .context("Failed to start server")
}

async fn connect_db() -> anyhow::Result<PgPool> {
    let res = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:1234@db:5432/test")
        .await
        .context("Failed to connect to Postgres");

    info!("Connected to Postgres");

    res
}

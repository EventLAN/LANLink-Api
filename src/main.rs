use axum::Router;
use deadpool_diesel::postgres::Pool;
use listenfd::ListenFd;
use tokio::net::TcpListener;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

mod features;
mod schema;
mod core;

use features::auth::controllers::auth_controller;
use features::protected::protected_controller;
use features::sign_up::controllers::sign_up_controller;
use features::user::controllers::user_controller;
use features::equipment::controllers::equipment_controller;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() {
    let db_pool = create_db_pool();

    run_migrations(&db_pool).await;

    let app = Router::new()
        .merge(auth_controller::router())
        .merge(protected_controller::router())
        .merge(sign_up_controller::router())
        .merge(equipment_controller::router())
        .merge(user_controller::router())
        .with_state(db_pool);


    // create listener
    let listener = create_listener().await;

    // run it
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn create_db_pool() -> Pool {
    // create db pools
    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    Pool::builder(manager)
        .build()
        .unwrap()
}

async fn run_migrations(pool: &Pool) {
    let conn = pool.get().await.unwrap();
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();
}

async fn create_listener() -> TcpListener {
    let mut listenfd = ListenFd::from_env();
    let port = dotenvy::var("PORT").unwrap();
    match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => TcpListener::bind(format!("127.0.0.1:{port}")).await.unwrap(),
    }
}
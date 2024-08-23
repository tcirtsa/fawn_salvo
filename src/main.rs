mod handler;
mod db;
mod model;

use salvo::http::Method;
use salvo::prelude::*;
use salvo::cors::Cors;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use std::{env, vec};
use dotenv::dotenv;
use once_cell::sync::OnceCell;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

static DB_POOL: OnceCell<DbPool> = OnceCell::new();

pub fn connect() -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, PoolError> {
    DB_POOL.get().unwrap().get()
}
fn build_pool(database_url: &str, size: u32) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    diesel::r2d2::Pool::builder()
        .max_size(size)
        .min_idle(Some(size))
        .test_on_check_out(false)
        .idle_timeout(None)
        .max_lifetime(None)
        .build(manager)
}
#[handler]
async fn hello() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    // 载入.env文件中的环境变量
    dotenv().ok();

    // 从环境变量获取数据库的连接字符串
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    DB_POOL
        .set(build_pool(&database_url, 10).expect(&format!("Error connecting to {}", &database_url)))
        .ok();
    
    tracing_subscriber::fmt::init();
    
    let cors = Cors::new()
        .allow_origin("*")
        .allow_credentials(false)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .into_handler();

    let router = Router::with_path("/test").hoop(cors).get(handler::test::test);
    let acceptor = TcpListener::new("127.0.0.1:7878").bind().await;
    Server::new(acceptor).serve(router).await;
}

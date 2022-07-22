mod api;
mod types;

use axum::{
    async_trait,
    body::Body,
    extract::{Extension, FromRequest, RequestParts},
    extract::{MatchedPath, Path},
    http::{Request, Response, StatusCode},
    response::Html,
    routing::get,
    Router,
};
// use sqlx::postgres::{PgPool, PgPoolOptions};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{convert::Infallible, net::SocketAddr, time::Duration};

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    // let db_connection_str = std::env::var("DATABASE_URL")
    //     .unwrap_or("postgres://username:password@host/database".to_string());

    // setup connection
    // let mut opt = ConnectOptions::new(db_connection_str);
    // opt.max_connections(5)
    //     .connect_timeout(Duration::from_secs(3));
    // let db: DatabaseConnection = Database::connect(opt)
    //     .await
    //     .expect("cannot connect to database");

    let counter = Arc::new(Mutex::new(0));

    // build our application with some routes
    let app = Router::new()
        .route("/", get(serve))
        .route(
            "/count",
            get(|req: Request<Body>| async move {
                let mut counter_ref = counter.lock().await;
                *counter_ref += 1;
                Html(format!("<h1>I've been loaded {} times!</h1>", *counter_ref))
            }),
        )
        .route("/api/:action", get(action));
    //     .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn serve(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("req: {:?}", req);
    let res = Response::new(Body::from("Hi from `GET /`"));
    Ok::<_, Infallible>(res)
}

async fn action(req: Request<Body>) {
    let path_opt = req.extensions().get::<MatchedPath>();
    println!("path_opt: {:?}", path_opt);
    if let Some(path) = path_opt {
        println!("path: {:?}", path.as_str());
    }
}

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower_service::Service;

struct CounterOp {
    count: Arc<Mutex<i32>>,
    op: i32,
}

impl Future for CounterOp {
    type Output = Result<i32, ()>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Ok(count_ref) = self.count.try_lock() {
            let result = *count_ref + self.op;
            Poll::Ready(Ok(result))
        } else {
            Poll::Pending
        }
    }
}

struct AtomicCounter {
    count: Arc<Mutex<i32>>,
}

impl Service<i32> for AtomicCounter {
    type Response = i32;
    type Error = ();
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    /// this should allow time for the DB to spin up. if not spun up, return Poll::Pending, otherwise Poll::Ready(Ok())
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: i32) -> Self::Future {
        let count_clone = self.count.clone();
        Box::pin(async move {
            let mut counter_ref = count_clone.lock().await;
            *counter_ref += req;
            Ok(*counter_ref)
        })
    }
}

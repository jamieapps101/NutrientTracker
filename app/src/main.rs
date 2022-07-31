mod api;
mod util;

use axum::{
    async_trait,
    body::Body,
    extract::{Extension, FromRequest, MatchedPath, Path, Query, RequestParts},
    http::{Request, Response, StatusCode},
    response::Html,
    routing::{get, get_service, post},
    Router,
};
use chrono::{Datelike, Timelike, Utc};
// use sqlx::postgres::{PgPool, PgPoolOptions};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{convert::Infallible, net::SocketAddr, time::Duration};

use std::sync::Arc;
use tokio::sync::Mutex;

use tower::ServiceBuilder;
use tower_http::services::ServeDir;

use itertools::Itertools;

use entity::prelude::*;
use sea_orm::{entity::Set, ActiveModelTrait, EntityTrait};

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let db_connection_str = std::env::var("DATABASE_URL").unwrap();
    //     .unwrap_or("postgres://username:password@host/database".to_string());

    // setup connection
    let mut opt = ConnectOptions::new(db_connection_str);
    opt.max_connections(5)
        .connect_timeout(Duration::from_secs(3));
    let db: DatabaseConnection = Database::connect(opt)
        .await
        .expect("cannot connect to database");

    let counter = Arc::new(Mutex::new(0));

    //
    // let user_routes = Router::new().route("/:id", get(|| async {}));
    // let team_routes = Router::new().route("/", post(|| async {}));
    // let api_routes = Router::new()
    //     .nest("/users", user_routes)
    //     .nest("/teams", team_routes);
    let api_routes = Router::new().route("/", get(serve)).layer(Extension(db));

    // build our application with some routes
    let app = Router::new()
        .route(
            "/",
            get_service(ServeDir::new("../frontend").append_index_html_on_directories(true))
                .handle_error(|_error: std::io::Error| async move { println!("oh dear") }),
        )
        .nest("/api", api_routes);

    // .route(
    //     "/count",
    //     get(|req: Request<Body>| async move {
    //         let mut counter_ref = counter.lock().await;
    //         *counter_ref += 1;
    //         Html(format!("<h1>I've been loaded {} times!</h1>", *counter_ref))
    //     }),
    // )
    // .route("/api/:action", get(action))
    // .layer(ServiceBuilder::new().layer(Extension(db)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct Params {
    page: Option<usize>,
    posts_per_page: Option<usize>,
}

async fn serve(
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Response<Body>, Infallible> {
    // println!("req: {:?}", req);
    // let now = Utc::now();

    // let time = entity::time::ActiveModel {
    //     hour: Set(now.hour() as i16),
    //     minute: Set(now.minute() as i16),
    //     second: Set(now.second() as i16),
    //     ..Default::default()
    // };
    // let time_res: Result<entity::time::Model, sea_orm::DbErr> = time.insert(conn).await;
    // let res = match time_res {
    //     Ok(_time) => {
    //         // retreive all
    //         let dates_res = Time::find()
    //             .into_model::<entity::time::Model>()
    //             .all(conn)
    //             .await;
    //         match dates_res {
    //             Ok(dates) => {
    //                 let mut response_string: String = "Hi from `GET /`".to_owned();

    //                 if !dates.is_empty() {
    //                     response_string += "\nI previously said hi on: \n";
    //                     response_string += &dates.iter().map(|d| format!("- {d:?}")).join("\n")
    //                 }
    //                 Response::new(Body::from(response_string))
    //             }
    //             Err(_db_err) => Response::new(Body::from("Hi from `GET /` (db retrieve err)")),
    //         }
    //     }
    //     Err(_db_err) => Response::new(Body::from("Hi from `GET /` (db insert err)")),
    // };
    // Ok::<_, Infallible>(res)
    todo!()
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

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_db_connection() {
        let db_connection_str = std::env::var("DATABASE_URL").unwrap();
        //     .unwrap_or("postgres://username:password@host/database".to_string());

        // setup connection
        let mut opt = ConnectOptions::new(db_connection_str);
        opt.max_connections(5)
            .connect_timeout(Duration::from_secs(3));
        let db: DatabaseConnection = Database::connect(opt)
            .await
            .expect("cannot connect to database");

        let date: entity::date::ActiveModel = entity::date::ActiveModel {
            day: Set(0),
            month: Set(0),
            year: Set(0),
            ..Default::default()
        };
        let date: entity::date::Model = date.insert(&db).await.unwrap();
        println!("added date: {date:?}");

        // retreive all
        let dates = Date::find()
            .into_model::<entity::date::Model>()
            .all(&db)
            .await
            .unwrap();
        dates.iter().for_each(|d| {
            println!("{d:?}");
        });
    }
}

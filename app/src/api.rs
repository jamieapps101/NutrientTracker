use axum::{
    body::Body,
    extract::{Extension, FromRequest, Json, MatchedPath, Path, Query, RequestParts},
    http::{Request, Response, StatusCode},
};
use chrono::{Datelike, Duration, Timelike, Utc};
use itertools::Itertools;
use sea_orm::{
    entity::Set, ActiveModelTrait, ColumnTrait, ConnectOptions, Database, DatabaseConnection,
    EntityTrait, IntoActiveModel, QueryFilter, Value,
};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, net::SocketAddr};

use entity::{prelude::Sessions, *};

use crate::util;

// TODO add in login managment API functions here

/// todo: make generic on any type that can be deserialised and transmitted into the db
async fn add<
    'a,
    AM: ActiveModelTrait<Entity = EN>,
    T: Deserialize<'a> + IntoActiveModel<AM>,
    EN: EntityTrait<Model = T>,
>(
    Extension(ref conn): Extension<DatabaseConnection>,
    Json(payload): Json<util::Transmission<T>>,
) -> Result<Response<Body>, Infallible> {
    // todo!();

    // extract id/data from transmission
    let (session_id, data) = payload.unwrap();
    // validate session id
    match is_valid_session(session_id, conn).await {
        SessionValidation::Valid => {
            // do nothing, continue flow
        }
        SessionValidation::Invalid => {
            todo!()
        }
        SessionValidation::Timeout => {
            todo!()
        }
        SessionValidation::DBErr(err) => {}
    }
    // convert to active model
    let data: AM = data.into_active_model();
    // create in db
    if let Err(err) = EN::insert(data).exec(conn).await {
        todo!()
        // do something?
    }
    // add to log??
    let res = Response::new(Body::from("Done".to_owned()));
    Ok::<_, Infallible>(res)
}

enum SessionValidation {
    Valid,
    Invalid,
    Timeout,
    DBErr(sea_orm::DbErr),
}

async fn is_valid_session(session_id: u32, conn: &DatabaseConnection) -> SessionValidation {
    if cfg!(test) {
        // running in test mode
        // todo: determine test to determine if true validation required
        // todo: add true validation
        SessionValidation::Valid
    } else {
        // query db based on session id
        let user_session_res: Result<Option<sessions::Model>, sea_orm::DbErr> = Sessions::find()
            .filter(sessions::Column::UserId.eq(session_id))
            .one(conn)
            .await;
        let user_session: Option<sessions::Model> = match user_session_res {
            Ok(s) => s,
            Err(err) => return SessionValidation::DBErr(err),
        };

        if let Some(user_session) = user_session {
            let current_time = chrono::Utc::now();
            // if session id, but current_time - active_time > 5mins -> TimeOut (also delete session)
            if (current_time - user_session.last_active) > Duration::minutes(5) {
                // delete session
                let user_session_am = user_session.into_active_model();
                if let Err(err) = user_session_am.delete(conn).await {
                    return SessionValidation::DBErr(err);
                } else {
                    return SessionValidation::Timeout;
                }
            } else {
                let mut user_session_am = user_session.into_active_model();
                let orm_val = Value::ChronoDateTimeUtc(Some(Box::new(current_time)));
                user_session_am.set(sessions::Column::LastActive, orm_val);
                if let Err(err) = user_session_am.update(conn).await {
                    return SessionValidation::DBErr(err);
                } else {
                    // update last active time
                    return SessionValidation::Valid;
                }
            }
            // else -> session valid
        } else {
            // if no session id -> Invalid
            return SessionValidation::Invalid;
        }
    }
}

/*
    async fn serve(
        Extension(ref conn): Extension<DatabaseConnection>,
    ) -> Result<Response<Body>, Infallible> {
        // println!("req: {:?}", req);
        let now = Utc::now();

        let time = entity::time::ActiveModel {
            hour: Set(now.hour() as i16),
            minute: Set(now.minute() as i16),
            second: Set(now.second() as i16),
            ..Default::default()
        };
        let time_res: Result<entity::time::Model, sea_orm::DbErr> = time.insert(conn).await;
        let res = match time_res {
            Ok(_time) => {
                // retreive all
                let dates_res = Time::find()
                    .into_model::<entity::time::Model>()
                    .all(conn)
                    .await;
                match dates_res {
                    Ok(dates) => {
                        let mut response_string: String = "Hi from `GET /`".to_owned();

                        if dates.len() > 0 {
                            response_string += "\nI previously said hi on: \n";
                            response_string += &dates.iter().map(|d| format!("- {d:?}")).join("\n")
                        }
                        Response::new(Body::from(response_string))
                    }
                    Err(_db_err) => Response::new(Body::from("Hi from `GET /` (db retrieve err)")),
                }
            }
            Err(_db_err) => Response::new(Body::from("Hi from `GET /` (db insert err)")),
        };
        Ok::<_, Infallible>(res)
    }
*/

#[cfg(test)]
mod test {
    #[test]
    fn test_is_valid_session() {
        // TODO
        // test no db
        // test valid, invalid and timeout
        unimplemented!();
    }
}

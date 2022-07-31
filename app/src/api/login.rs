use axum::{
    body::Body,
    extract::{Extension, Json},
    http::Response,
};
use chrono::Utc;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::convert::Infallible;

use crate::util;

use entity::{
    prelude::{Sessions, UserAccounts},
    *,
};

use rand::prelude::*;
use rand_hc::Hc128Rng;
use sha3::{Digest, Sha3_512};

// TODO add in login managment API functions here
pub async fn create_user(
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(ref mut rng): Extension<Hc128Rng>,
    Json(payload): Json<util::LoginRequest>,
) -> Result<Response<Body>, Infallible> {
    // No need for ID here
    let data = payload;
    // check email is not in use.
    // todo, above

    // generate salt
    let mut salt = [0u8; 64];
    rng.fill_bytes(&mut salt);

    // hash incoming password hash with salt
    let mut hasher = Sha3_512::new();
    hasher.update(data.password_hash);
    hasher.update(salt);
    let hash = hasher.finalize();
    let hash_vec = hash.to_vec();
    let salt_vec = salt.to_vec();

    // create active model
    let user_details = user_accounts::ActiveModel {
        name: ActiveValue::Set(data.username),
        salt: ActiveValue::Set(salt_vec),
        password_hash: ActiveValue::Set(hash_vec),
        ..Default::default()
    };
    // insert record
    Ok(match UserAccounts::insert(user_details).exec(conn).await {
        Ok(_user_details) => Response::new(Body::from("done")),
        Err(_err) => Response::new(Body::from("not done")),
    })
}

pub async fn login(
    Extension(ref conn): Extension<DatabaseConnection>,
    Json(payload): Json<util::LoginRequest>,
) -> Result<Response<Body>, Infallible> {
    // take username/password_hash
    // determine if username exists, return err if not
    let user_account_res: Result<Option<user_accounts::Model>, sea_orm::DbErr> =
        UserAccounts::find()
            .filter(user_accounts::Column::Name.eq(payload.username))
            .one(conn)
            .await;
    let user_account: Option<user_accounts::Model> = match user_account_res {
        Ok(s) => s,
        Err(_err) => todo!(), // io err, not a non-existant error
    };
    if user_account.is_none() {
        // return some error about incorrect username/password
        todo!()
    }
    let user_account = user_account.unwrap();
    // get salt for username
    let salt = user_account.salt;
    // create hash with password_hash and salt
    let mut hasher = Sha3_512::new();
    hasher.update(payload.password_hash);
    hasher.update(salt);
    let hash = hasher.finalize();

    // compare against db hash
    // if not same, return err
    if hash.to_vec() != user_account.password_hash {
        // return some error about incorrect username/password
        todo!()
    }
    // otherwise create session and return session id
    let new_session = sessions::ActiveModel {
        user_id: ActiveValue::Set(user_account.id),
        start_time: ActiveValue::Set(Utc::now()),
        last_active: ActiveValue::Set(Utc::now()),
        ..Default::default()
    };
    let _res = Sessions::insert(new_session).exec(conn).await;

    todo!();
}

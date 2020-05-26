use actix::prelude::*;
use actix::sync::SyncContext;
use actix_web::{web, HttpResponse, Responder};
use diesel::sqlite::SqliteConnection;
use std::io::Error;
use uuid;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(index));
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("user!")
}

struct DbExecutor(SqliteConnection);

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let uuid = format!("{}", uuid::Uuid::new_v4());

        let new_user = User {
            id: uuid,
            name: msg.name,
        };
        diesel::insert_into(users)
            .values(&new_user)
            .execute(&self.0)
            .expect("Error inserting user");

        let mut items = users
            .filter(id.eq(&uuid))
            .load::<User>(&self.0)
            .expect("Error loading user");

        Ok(new_user)
    }
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

struct CreateUser {
    name: String,
}

impl Message for CreateUser {
    type Result = Result<User, Error>;
}

struct User {
    id: String,
    name: String,
}

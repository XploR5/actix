use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::PgPool;
use crate::models::User;

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

pub async fn create_user(
    pool: web::Data<PgPool>,
    form: web::Form<CreateUser>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = User::new(form.name.clone(), form.email.clone());

    match diesel::insert_into(crate::schema::users::table)
        .values(&user)
        .execute(&conn)
    {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse { success: true }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    match crate::schema::users::table.load::<User>(&conn) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
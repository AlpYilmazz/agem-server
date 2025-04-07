use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;

use crate::{
    db::{DatabaseConnection, types::DB_Account},
    password::hash_password,
    response::{HttpErrorBody, HttpJsonMessageBody},
    service,
};

#[derive(Deserialize)]
struct CreateAccountRequest {
    pub name: String,
    pub lastname: String,
    pub phone_number: String,
    pub email: String,
    pub password: String,
}

#[post("/account")]
pub async fn create_account(
    db: web::Data<DatabaseConnection>,
    account: web::Json<CreateAccountRequest>,
) -> impl Responder {
    let db = (*db.into_inner()).clone();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let db_account = DB_Account {
        id: 0,
        name: account.name.clone(),
        lastname: account.lastname.clone(),
        email: account.email.clone(),
        phone_number: account.phone_number.clone(),
        hashed_password: hash_password(&account.password),
        password_set_ts: now as i64,
    };

    match service::account::create_account(db, db_account).await {
        Ok(true) => HttpResponse::Created().json_message_body("Success"),
        Ok(false) => HttpResponse::InternalServerError()
            .error_body("Account cannot be created because given phone number is already used."),
        Err(err) => HttpResponse::InternalServerError().error_body(err),
    }
}

#[derive(Deserialize)]
struct AddRolesToAccountRequest {
    account_id: i32,
    role_ids: Vec<i32>,
}

#[post("/account/roles")]
pub async fn add_roles_to_account(
    db: web::Data<DatabaseConnection>,
    request: web::Json<AddRolesToAccountRequest>,
) -> impl Responder {
    let db = (*db.into_inner()).clone();
    let request = request.into_inner();

    match service::account::add_roles_to_account(db, request.account_id, request.role_ids).await {
        Ok(true) => HttpResponse::Created().json_message_body("Success"),
        Ok(false) => HttpResponse::InternalServerError().error_body("Duplicate user role"),
        Err(err) => HttpResponse::InternalServerError().error_body(err),
    }
}

#[post("/roles")]
pub async fn create_roles(
    db: web::Data<DatabaseConnection>,
    roles: web::Json<Vec<String>>,
) -> impl Responder {
    let db = (*db.into_inner()).clone();
    let roles = roles.into_inner();

    match service::account::create_roles(db, roles).await {
        Ok(true) => HttpResponse::Created().json_message_body("Success"),
        Ok(false) => HttpResponse::InternalServerError().error_body("Duplicate role"),
        Err(err) => HttpResponse::InternalServerError().error_body(err),
    }
}

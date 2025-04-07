#![allow(non_camel_case_types)]

use sqlx::prelude::FromRow;

pub type ID = i32;

#[derive(FromRow)]
pub struct DB_Account {
    pub id: ID,
    pub phone_number: String,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub hashed_password: String,
    pub password_set_ts: i64,
}

#[derive(FromRow)]
pub struct DB_Role {
    pub id: ID,
    pub role: String,
}

#[derive(FromRow)]
pub struct DB_MonthlyFee {
    pub id: ID,
    pub year: i32,
    pub month: i32,
}

#[derive(FromRow)]
pub struct DB_UserRole {
    pub user_id: ID,
    pub role_id: ID,
}

#[derive(FromRow)]
pub struct DB_Payment {
    pub user_id: ID,
    pub fee_id: ID,
}
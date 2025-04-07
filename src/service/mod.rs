use crate::db::types::ID;

pub mod account;

pub struct Account {
    pub id: ID,
    pub phone_number: String,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub hashed_password: String,
    pub password_set_ts: u64,
    // pub roles: Vec<String>,
    // pub payments: Vec<MonthlyFee>
}

pub struct Role {
    // pub id: ID,
    pub role: String,
}

pub struct MonthlyFee {
    // pub id: ID,
    pub year: u32,
    pub month: u32,
}

pub struct Payments {
    pub made: Vec<MonthlyFee>,
    pub precovered: Vec<MonthlyFee>,
}

// pub struct UserRole {
//     pub user_id: u32,
//     pub role_id: u32,
// }

// pub struct Payment {
//     pub user_id: u32,
//     pub fee_id: u32,
// }
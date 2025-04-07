// use actix_web::{HttpResponse, Responder, get, web};
// use serde::Serialize;

// use crate::{
//     api::SortPagination,
//     db::memorydb::{DB_MonthlyFee, MemoryDB},
// };

// #[derive(Serialize)]
// struct MonthlyFee {
//     month: u32,
//     year: u32,
// }

// #[get("/monthly_fees")]
// async fn get_all_fees(
//     db: web::Data<MemoryDB>,
//     query: web::Query<SortPagination>,
// ) -> impl Responder {
//     let mut db = db.write().unwrap();

//     // let fees = Vec::with_capacity(query.count);
//     // if query.sort >= 0 {
//     // } else {
//     //     // DESC

//     // }

//     HttpResponse::Ok()
// }

use actix_web::{HttpRequest, HttpResponse, Responder, get};
use serde::Deserialize;

pub mod auth;
pub mod v1;

#[derive(Deserialize)]
struct Sort {
    sort: i32,
}

#[derive(Deserialize)]
struct Pagination {
    offset: usize,
    count: usize,
}

#[derive(Deserialize)]
struct SortPagination {
    sort: i32,
    offset: usize,
    count: usize,
}

#[get("/whoami")]
async fn whoami(req: HttpRequest) -> impl Responder {
    let info = format!(
        r#"
        peer_addr: {:?}
        connection_info: {:?}
        request: {:?}
        "#,
        req.peer_addr(),
        req.connection_info(),
        req,
    );
    HttpResponse::Ok().body(info)
}

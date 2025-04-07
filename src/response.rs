use actix_web::{HttpResponse, HttpResponseBuilder};
use serde::Serialize;

#[derive(Serialize)]
struct JsonMessage {
    msg: String,
}

pub trait HttpJsonMessageBody {
    fn json_message_body(self, msg: impl ToString) -> HttpResponse;
}

impl HttpJsonMessageBody for HttpResponseBuilder {
    fn json_message_body(mut self, msg: impl ToString) -> HttpResponse {
        self.json(JsonMessage { msg: msg.to_string() })
    }
}

pub trait HttpErrorBody {
    fn error_body(self, msg: impl ToString) -> HttpResponse;
}

impl HttpErrorBody for HttpResponseBuilder {
    fn error_body(mut self, msg: impl ToString) -> HttpResponse {
        self.json(JsonMessage { msg: msg.to_string() })
    }
}
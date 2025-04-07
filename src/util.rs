use actix_web::{
    HttpMessage,
    http::header::{self, Header, HeaderName, HeaderValue, InvalidHeaderValue, TryIntoHeaderValue},
};

pub struct BearerToken {
    pub token: String,
}

impl TryIntoHeaderValue for BearerToken {
    type Error = InvalidHeaderValue;

    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        let header = format!("Bearer {}", &self.token);
        HeaderValue::from_str(&header)
    }
}

impl Header for BearerToken {
    fn name() -> HeaderName {
        header::AUTHORIZATION
    }

    fn parse<M: HttpMessage>(msg: &M) -> Result<Self, actix_web::error::ParseError> {
        match msg.headers().get(Self::name()).map(|h| h.to_str()) {
            Some(Ok(b)) => Ok(Self {
                token: b
                    .split_once("Bearer ")
                    .ok_or(actix_web::error::ParseError::Header)?
                    .1
                    .to_string(),
            }),
            _ => Err(actix_web::error::ParseError::Header),
        }
    }
}

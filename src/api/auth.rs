use actix_web::{HttpResponse, Responder, get, http::header::ContentType, post, web};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{ACCESS_TOKEN_EXPIRES_IN, Claims},
    db::DatabaseConnection,
    password::verify_password,
    response::HttpErrorBody,
    service::{self, Account, Role},
    util::BearerToken,
};

const JWT_ENCODING_KEY_SECRET: &'static str = "c284f7ac-b1cd-4f14-bdeb-cd4736974e3b";

#[derive(Deserialize)]
struct AccessTokenRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct RefreshRequest {
    refresh_token: String,
}

#[derive(Deserialize, Serialize)]
struct TokenResponse {
    access_token: String,
    expires_in: usize,
    refresh_token: String,
}

impl TokenResponse {
    fn create(account: Account, roles: Vec<Role>) -> TokenResponse {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let access_token_claims = Claims::for_access_token(
            now,
            account.email.clone(),
            roles.into_iter().map(|r| r.role).collect(),
        );
        let refresh_token_claims = Claims::for_refresh_token(now, account.email.clone());

        let access_token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &access_token_claims,
            &jsonwebtoken::EncodingKey::from_secret(JWT_ENCODING_KEY_SECRET.as_ref()),
        )
        .unwrap();

        let refresh_token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &refresh_token_claims,
            &jsonwebtoken::EncodingKey::from_secret(JWT_ENCODING_KEY_SECRET.as_ref()),
        )
        .unwrap();

        TokenResponse {
            access_token,
            expires_in: ACCESS_TOKEN_EXPIRES_IN,
            refresh_token,
        }
    }
}

#[get("/me")]
pub async fn get_me_from_access_token(bearer_token: web::Header<BearerToken>) -> impl Responder {
    let bearer_token = bearer_token.into_inner();

    let claims = match jsonwebtoken::decode::<Claims>(
        &bearer_token.token,
        &jsonwebtoken::DecodingKey::from_secret(JWT_ENCODING_KEY_SECRET.as_ref()),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    ) {
        Ok(c) => c.claims,
        Err(err) => {
            return HttpResponse::BadRequest().error_body(err);
        }
    };

    HttpResponse::Ok().json(claims)
}

#[post("/connect")]
pub async fn create_access_token(
    db: web::Data<DatabaseConnection>,
    request: web::Form<AccessTokenRequest>,
) -> impl Responder {
    let db = (*db.into_inner()).clone();
    let request = request.into_inner();

    let account = match service::account::fetch_account_by_email(db.clone(), request.email).await {
        Ok(Some(a)) => a,
        Ok(None) => {
            return HttpResponse::NotFound().error_body("Account not found.");
        }
        Err(err) => {
            return HttpResponse::InternalServerError().error_body(err);
        }
    };

    if !verify_password(&request.password, &account.hashed_password) {
        return HttpResponse::NotFound().error_body("Incorrect password.");
    }

    let roles = match service::account::fetch_account_roles(db, account.id).await {
        Ok(a) => a,
        Err(err) => {
            return HttpResponse::InternalServerError().error_body(err);
        }
    };

    let token_response = TokenResponse::create(account, roles);

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(token_response)
}

#[post("/refresh")]
pub async fn refresh_access_token(
    db: web::Data<DatabaseConnection>,
    request: web::Form<RefreshRequest>,
) -> impl Responder {
    let db = (*db.into_inner()).clone();
    let request = request.into_inner();

    // Validate refresh token
    let (issued_at, account_email) = {
        let refresh_token_claims = match jsonwebtoken::decode::<Claims>(
            &request.refresh_token,
            &jsonwebtoken::DecodingKey::from_secret(JWT_ENCODING_KEY_SECRET.as_ref()),
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
        ) {
            Ok(c) => c.claims,
            Err(err) => {
                return HttpResponse::BadRequest().error_body(err);
            }
        };

        (refresh_token_claims.iat, refresh_token_claims.sub)
    };

    // Make access token
    let account = match service::account::fetch_account_by_email(db.clone(), account_email).await {
        Ok(Some(a)) => a,
        Ok(None) => {
            return HttpResponse::NotFound().error_body("Account not found.");
        }
        Err(err) => {
            return HttpResponse::InternalServerError().error_body(err);
        }
    };

    if issued_at <= account.password_set_ts as usize {
        return HttpResponse::BadRequest().error_body("Invalid token.");
    }

    let roles = match service::account::fetch_account_roles(db, account.id).await {
        Ok(a) => a,
        Err(err) => {
            return HttpResponse::InternalServerError().error_body(err);
        }
    };

    let token_response = TokenResponse::create(account, roles);

    HttpResponse::Ok().json(token_response)
}

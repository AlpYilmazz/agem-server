use serde::{Deserialize, Serialize};

pub const TOKEN_TYPE_ACCESS: &'static str = "access";
pub const TOKEN_TYPE_REFRESH: &'static str = "refresh";
pub const ACCESS_TOKEN_EXPIRES_IN: usize = 1 * 60 * 20; // 20 minutes
pub const REFRESH_TOKEN_EXPIRES_IN: usize = 1 * 60 * 60 * 24 * 7; // 1 week

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    // aud: String,         // Optional. Audience
    pub exp: usize,             // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize,             // Optional. Issued at (as UTC timestamp)
    // iss: String,         // Optional. Issuer
    // nbf: usize,          // Optional. Not Before (as UTC timestamp)
    pub sub: String,            // Optional. Subject (whom token refers to)
    // -- custom --
    pub token_type: String,
    pub role: Vec<String>,
}

impl Claims {
    pub fn for_access_token(now: usize, sub: String, role: Vec<String>) -> Claims {
        Claims {
            exp: now + ACCESS_TOKEN_EXPIRES_IN,
            iat: now,
            sub,
            token_type: TOKEN_TYPE_ACCESS.to_string(),
            role,
        }
    }

    pub fn for_refresh_token(now: usize, sub: String) -> Claims {
        Claims {
            exp: now + REFRESH_TOKEN_EXPIRES_IN,
            iat: now,
            sub,
            token_type: TOKEN_TYPE_REFRESH.to_string(),
            role: Vec::new(),
        }
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.role.iter().any(|r| r.eq(role))
    }

    pub fn has_role_starts_with(&self, role: &str) -> bool {
        self.role.iter().any(|r| r.starts_with(role))
    }

    pub fn is_admin(&self) -> bool {
        self.has_role("Admin")
    }
}
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthBodyDto {
    access_token: String,
    token_type: String,
}

impl AuthBodyDto {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}
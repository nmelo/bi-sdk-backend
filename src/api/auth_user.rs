use crate::utils::{error::GenericError, result::Result};
use serde::{Deserialize, Serialize};

// -- Auth User

#[derive(Deserialize, Serialize, Debug)]
pub struct Request {
    email: String,
    password: String,
    returnSecureToken: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Response {
    idToken: String,
    email: String,
    refreshToken: String,
    expiresIn: String,
    localId: String,
    registered: bool,
}

pub async fn handle(request: Request) -> Result<Response> {
    let api_key = std::env::var("FIREBASE_API_KEY")?;
    let serialized_request = serde_json::to_string(&request)?;
    let response = reqwest::Client::new()
        .post(format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
            api_key
        ))
        .body(serialized_request)
        .send()
        .await?;
    let status = response.status();
    let body = response.text().await?;
    return match status {
        reqwest::StatusCode::OK => {
            let deserialized_response: Response = serde_json::from_str(&body)?;
            return Ok(deserialized_response);
        }
        _ => Err(Box::new(GenericError(body.into()))),
    };
}

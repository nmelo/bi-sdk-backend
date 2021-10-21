use crate::models::user::User;
use crate::utils::{error::GenericError, result::Result};
use serde::{Deserialize, Serialize};

// -- Create User

#[derive(Deserialize, Serialize, Debug)]
pub struct Request {
    binding_token_delivery_method: String,
    external_id: String,
    email: String,
    user_name: String,
    display_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Response {
    user: User,
}

pub async fn handle(request: Request) -> Result<Response> {
    let api_token = std::env::var("API_TOKEN")?;
    let serialized_request = serde_json::to_string(&request)?;

    let response = reqwest::Client::new()
        .post("https://api.byndid.com/v1/manage/users")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", api_token),
        )
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

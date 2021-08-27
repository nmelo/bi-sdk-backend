use serde::{Deserialize, Serialize};
use std::env;
use std::error;
use std::fmt;

// -- Recover User

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Request {
	ExternalId(String),
	InternalId(String),
}

#[derive(Deserialize, Serialize)]
pub struct Response {
	internal_id: String,
	external_id: String,
	email: String,
	user_name: String,
	display_name: String,
	date_created: String,
	date_modified: String,
	status: String,
}

#[derive(Debug)]
struct GenericError(String);

impl error::Error for GenericError {}

impl fmt::Display for GenericError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

pub async fn handle(request: Request) -> Result<Response> {
	let api_token = env::var("API_TOKEN")?;
	let serialized_request = serde_json::to_string(&request)?;

	let response = reqwest::Client::new()
		.post("https://api.byndid.com/v0/recover-user")
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

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    internal_id: String,
    external_id: String,
    email: String,
    user_name: String,
    display_name: String,
    date_created: String,
    date_modified: String,
    status: String,
}

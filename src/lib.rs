#[macro_use]
extern crate serde_derive;
use failure::Error;

use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub token: String,
    pub security_code: String,
    pub error: Option<ApiError>,
}

pub fn request_login(email: &str) -> Result<LoginRequest, Error> {
    let client = reqwest::Client::new();
    let mut json = HashMap::new();
    json.insert("email", email);
    json.insert("tokenName", "Unofficial Now client written in Rust");

    let mut response = client
        .post("https://api.zeit.co/now/registration")
        .json(&json)
        .send()?;

    let json: LoginRequest = response.json()?;
    Ok(json)
}

#[derive(Deserialize, Debug)]
pub struct Auth {
    pub token: Option<String>,
    pub error: Option<ApiError>,
}

pub fn verify_login(email: &str, token: &str) -> Result<Auth, Error> {
    let client = reqwest::Client::new();
    let auth_url = format!(
        "https://api.zeit.co/now/registration/verify?email={}&token={}",
        email, token
    );

    let mut response = client.get(&auth_url).send()?;

    let json: Auth = response.json()?;
    Ok(json)
}

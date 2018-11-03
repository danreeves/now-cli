use super::handlers;
use failure::Error;
use reqwest::header;
use std::collections::HashMap;
use std::fmt;

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

#[derive(Deserialize, Debug)]
pub enum DeploymentType {
    NPM,
    DOCKER,
    STATIC,
}

impl fmt::Display for DeploymentType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let printable = match &self {
            DeploymentType::NPM => "NPM",
            DeploymentType::DOCKER => "DOCKER",
            DeploymentType::STATIC => "STATIC",
        };
        write!(formatter, "{}", printable)
    }
}

#[derive(Deserialize, Debug)]
pub enum DeploymentState {
    DEPLOYING,
    DEPLOYMENT_ERROR,
    BOOTED,
    BUILDING,
    READY,
    BUILD_ERROR,
    FROZEN,
}

impl fmt::Display for DeploymentState {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let printable = match &self {
            DeploymentState::DEPLOYING => "DEPLOYING",
            DeploymentState::DEPLOYMENT_ERROR => "DEPLOYMENT_ERROR",
            DeploymentState::BOOTED => "BOOTED",
            DeploymentState::BUILDING => "BUILDING",
            DeploymentState::READY => "READY",
            DeploymentState::BUILD_ERROR => "BUILD_ERROR",
            DeploymentState::FROZEN => "FROZEN",
        };
        write!(formatter, "{}", printable)
    }
}

#[derive(Deserialize, Debug)]
pub struct Scale {
    current: u8,
    min: u8,
    max: u8,
}

#[derive(Deserialize, Debug)]
pub struct Deployment {
    pub uid: String,
    pub name: String,
    pub url: String,
    pub created: i64,
    #[serde(rename = "type")]
    pub deployment_type: DeploymentType,
    pub state: Option<DeploymentState>,
    pub scale: Option<Scale>,
}

#[derive(Deserialize, Debug)]
struct Deployments {
    deployments: Option<Vec<Deployment>>,
    error: Option<ApiError>,
}

pub fn get_list(auth: handlers::Auth) -> Result<Vec<Deployment>, Error> {
    let client = reqwest::Client::new();

    let mut response = client
        .get("https://api.zeit.co/v2/now/deployments")
        .header(header::AUTHORIZATION, format!("Bearer {}", auth.token))
        .send()?;

    let json: Deployments = response.json()?;

    if let Some(deployments) = json.deployments {
        return Ok(deployments);
    } else {
        bail!(json.error.unwrap().message)
    }
}

use console::style;
use dialoguer::{Confirmation, Input};
use directories::ProjectDirs;
use failure::Error;
use mkdirp::mkdirp;
use serde_json::{from_str, to_string};
use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthFile {
    pub email: String,
    pub token: String,
}

#[derive(Debug)]
pub struct UserData {
    pub data: Option<AuthFile>,
    project_dir: ProjectDirs,
}

impl UserData {
    fn new(email: String, token: String) -> UserData {
        let project_dir = UserData::get_project_dir();
        let data = AuthFile { email, token };
        return UserData {
            data: Some(data),
            project_dir,
        };
    }

    fn from_fs() -> UserData {
        let project_dir = UserData::get_project_dir();
        let mut user_data = UserData {
            data: None,
            project_dir,
        };
        // TODO: Handle this Result?
        user_data.load();
        user_data
    }

    fn get_project_dir() -> ProjectDirs {
        if let Some(project_dir) = ProjectDirs::from("com", "Unofficial Now CLI", "now-cli") {
            return project_dir;
        }
        // ProjectDirs couldn't determine the home directory or something
        // I have no idea why this would fail but apparently it can. We
        // can't do anything if it does fail though...
        panic!()
    }

    fn get_config_dir(&self) -> &Path {
        self.project_dir.data_dir()
    }

    fn get_config_filepath(&self) -> PathBuf {
        self.get_config_dir().join("auth.json")
    }

    fn load(&mut self) -> Result<(), Error> {
        let filepath = self.get_config_filepath();
        if let Ok(mut file) = File::open(&filepath) {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let json: AuthFile = from_str(&contents)?;
            self.data = Some(json);
        }
        Ok(())
    }

    fn save(&self) -> Result<(), Error> {
        let config_dir = self.get_config_dir();
        let filepath = self.get_config_filepath();
        let json_string = to_string(&self.data)?;

        mkdirp(&config_dir)?;
        let mut file = File::create(&filepath)?;
        file.write_all(&json_string.as_bytes())?;
        Ok(())
    }

    fn delete(&self) -> Result<(), Error> {
        let filepath = self.get_config_filepath();
        remove_file(&filepath)?;
        Ok(())
    }
}

pub fn login() -> Result<(), Error> {
    println!("▲ Authenticate with {}", style("Now").bold());
    let email = Input::new(&format!("{}", style("Email").bold())).interact()?;

    let login_request = now_cli::request_login(&email)?;

    println!(
        "An email has been sent to {} with the security code {}.",
        style(&email).bold(),
        style(login_request.security_code).bold()
    );

    if Confirmation::new(&format!("Please confirm once you've authenticated..."))
        .interact()
        .unwrap_or(false)
    {
        let auth_token = now_cli::verify_login(&email, &login_request.token)?;

        if auth_token.error.is_some() {
            // TODO return error
        } else if auth_token.token.is_some() {
            let user_data = UserData::new(email.clone(), auth_token.token.unwrap());
            user_data.save()?;
            println!("Logged in as {}", style(&email).bold());
        }
    } else {
        println!("You've not been signed in");
        return Ok(());
    }

    Ok(())
}

pub fn logout() -> Result<(), Error> {
    let user_data = UserData::from_fs();
    if let Some(data) = &user_data.data {
        if let Ok(_success) = user_data.delete() {
            println!("▲ Logged out of {}", style(&data.email).bold());
        } else {
            // TODO: Surface this error so that it can be debugged by users
            println!("▲ Failed to log out of {}", style(&data.email).bold());
        }
    } else {
        println!("▲ You're not logged in");
    }
    Ok(())
}

pub fn whoami() -> Result<(), Error> {
    let user_data = UserData::from_fs();
    if let Some(data) = &user_data.data {
        println!("▲ Logged in as {}", style(&data.email).bold());
    } else {
        println!("▲ You're not logged in");
    }
    Ok(())
}

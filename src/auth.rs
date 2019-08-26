use crate::HTTP_ORIGIN;
use dirs::home_dir;
use reqwest::Client;
use rpassword::read_password_from_tty;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

fn password_prompt() -> Result<String, io::Error> {
	loop {
		let input = read_password_from_tty(Some("Password: "))?;
		if input.chars().count() >= 6 {
			return Ok(input);
		}
		println!("Invalid password.\n");
	}
}

#[derive(Serialize, Debug)]
pub struct LoginRequest {
	email: String,
	password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
	pub token: String,
	#[serde(rename = "userId")]
	pub user_id: String,
	#[serde(rename = "expiresAt")]
	pub expires_at: String,
	#[serde(rename = "createdAt")]
	pub created_at: String,
}

impl LoginRequest {
	pub fn new(email: &str, password: &str) -> Self {
		Self {
			email: email.to_owned(),
			password: password.to_owned(),
		}
	}
}

pub fn login_request(email: &str) -> Result<LoginResponse, Box<dyn Error>> {
	let password = password_prompt()?;
	Ok(Client::new()
		.get(&format!("{}/user/token", HTTP_ORIGIN))
		.json(&LoginRequest::new(email, &password))
		.send()?
		.json()?)
}

pub struct RcFile {
	pub path: PathBuf,
}

impl RcFile {
	pub fn new() -> Result<Self, io::Error> {
		if let Some(home_path) = &home_dir() {
			let path = Path::new(&home_path).join(".yapbrc");
			if !path.exists() {
				File::create(&path)?;
			}
			Ok(Self { path })
		} else {
			Err(io::Error::new(
				io::ErrorKind::NotFound,
				"Could not access home directory.",
			))
		}
	}

	pub fn save_token(&self, token: &str) -> Result<(), io::Error> {
		OpenOptions::new()
			.write(true)
			.open(&self.path)?
			.write_all(token.as_bytes())
	}

	pub fn get_token(&self) -> Result<String, io::Error> {
		let file = File::open(&self.path)?;
		let mut reader = BufReader::new(file);
		let mut contents = String::new();
		reader.read_to_string(&mut contents)?;
		Ok(contents)
	}
}

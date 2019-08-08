use crate::HTTP_ORIGIN;
use dirs::home_dir;
use reqwest::Client;
use rpassword::read_password_from_tty;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
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

fn rc_file_path() -> PathBuf {
	Path::new(&home_dir().unwrap()).join(".yapbrc")
}

#[derive(Serialize, Debug)]
struct AuthenticateRequest {
	email: String,
	password: String,
}

impl AuthenticateRequest {
	pub fn new(email: &str, password: &str) -> Self {
		Self {
			email: email.to_owned(),
			password: password.to_owned(),
		}
	}
}

#[derive(Deserialize, Debug)]
struct AuthenticationResponse {
	token: String,
	#[serde(rename = "userId")]
	user_id: String,
	#[serde(rename = "expiresAt")]
	expires_at: String,
	#[serde(rename = "createdAt")]
	created_at: String,
}

pub fn login(email: &str) -> Result<(), Box<dyn Error>> {
	let password = password_prompt()?;
	let res: AuthenticationResponse = Client::new()
		.post(&format!("{}/api/user/token", HTTP_ORIGIN))
		.json(&AuthenticateRequest::new(email, &password))
		.send()?
		.json()?;
	println!("{:?}", res);
	let path = rc_file_path();
	let mut file = if path.exists() {
		File::open(path)?
	} else {
		File::create(path)?
	};
	file.write_all(res.token.as_bytes())?;
	Ok(())
}

pub fn get_token() -> Result<String, Box<dyn Error>> {
	let path = rc_file_path();
	if path.exists() {
		let file = File::open(path)?;
		let mut reader = BufReader::new(file);
		let mut contents = String::new();
		reader.read_to_string(&mut contents)?;
		Ok(contents)
	} else {
		Err(Box::from("Path does not exist"))
	}
}

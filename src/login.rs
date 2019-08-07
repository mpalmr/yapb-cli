use crate::HTTP_ORIGIN;
use dirs::home_dir;
use reqwest::Client;
use rpassword::read_password_from_tty;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;

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
}

pub fn login(email: &str) -> Result<(), Box<dyn Error>> {
	let password = password_prompt()?;
	let _res: AuthenticationResponse = Client::new()
		.post(&format!("{}/api/authenticate", HTTP_ORIGIN))
		.json(&AuthenticateRequest::new(email, &password))
		.send()?
		.json()?;
	let rc_file_path = Path::new(&home_dir().unwrap()).join(".yapbrc");
	let _rc_file = File::open(rc_file_path)?;
	Ok(())
}

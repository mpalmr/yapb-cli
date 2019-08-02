use crate::HTTP_ORIGIN;
use dirs::home_dir;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::path::Path;

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

pub fn login(email: &str, password: &str) -> Result<(), Box<dyn Error>> {
	let res: AuthenticationResponse = Client::new()
		.post(&format!("{}/api/authenticate", HTTP_ORIGIN))
		.json(&AuthenticateRequest::new(email, password))
		.send()?
		.json()?;
	let rc_file = File::open(Path::new(&home_dir().unwrap()).join(".yapbrc"))?;
	// rc_file.write("\n").map_err(|e| Box::from(e));
	Ok(())
}

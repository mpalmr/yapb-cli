use crate::auth::{login_request, RcFile};
use crate::paste::{self, Paste};
use crate::HTTP_ORIGIN;
use clap::ArgMatches;
use std::error::Error;
use std::path::Path;

pub fn get(args: &ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
	println!("Fetching paste.");
	let paste_id = args.value_of("id").unwrap();
	let out_dir = Path::new(args.value_of("target").unwrap_or(&paste_id));
	match out_dir.to_str() {
		Some(dir_text) => {
			println!("Fetching paste...");
			let paste = Paste::fetch(paste_id)?;
			println!("Writing files...");
			paste.write_to(out_dir)?;
			println!("Paste saved to: {}", dir_text);
			Ok(())
		}
		_ => Err(Box::from("Could not resolve path.")),
	}
}

pub fn create(args: &ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
	println!("Creating new paste.");
	let file_names = args.values_of("files").unwrap().collect::<Vec<&str>>();
	println!("Reading files...");
	let files = paste::read_files(file_names)?;
	println!("Uploading paste...");
	let paste = Paste::create(files)?;
	println!("Paste created: {}/paste/{}", HTTP_ORIGIN, paste.id);
	Ok(())
}

pub fn login(args: &ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
	let email = args.value_of("email").unwrap();
	println!("Logging in as '{}'", email);
	println!("Authenticating...");
	let res = login_request(email);
	println!("Credentials valid, opening rc file...");
	let rc_file = RcFile::new()?;
	println!("Using '{}'", rc_file.path.to_str().unwrap());
	Ok(())
}

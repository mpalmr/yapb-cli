use crate::auth::{login_request, RcFile};
use crate::paste::{self, Paste};
use crate::{Cli, HTTP_ORIGIN};
use clap::ArgMatches;
use std::error::Error;
use std::path::Path;

pub fn get(cli: Cli, args: &ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
	let paste_id = args.value_of("id").unwrap();
	let out_dir = Path::new(args.value_of("target").unwrap_or(&paste_id));
	if let Some(dir_str) = out_dir.to_str() {
		cli.log("Fetching paste...");
		let paste = Paste::fetch(paste_id)?;
		cli.log("Writing files...");
		paste.write(out_dir)?;
		println!("Paste saved to: {}", dir_str);
		Ok(())
	} else {
		Err(Box::from("Could not resolve path."))
	}
}

pub fn create(cli: Cli, args: &ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
	cli.log("Creating new paste...");
	let file_names = args.values_of("files").unwrap().collect::<Vec<&str>>();
	cli.log("Reading files...");
	let files = paste::read_files(file_names)?;
	cli.log("Uploading paste...");
	let paste = Paste::create(files)?;
	println!("Paste created: {}/paste/{}", HTTP_ORIGIN, paste.id);
	Ok(())
}

pub fn login(cli: Cli, args: &ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
	let email = args.value_of("email").unwrap();
	println!("Logging in as '{}'", email);
	cli.log("Authenticating...");
	let res = login_request(email);
	cli.log("Credentials valid, opening rc file...");
	let rc_file = RcFile::new()?;
	cli.log(&format!("Using '{}'", rc_file.path.to_str().unwrap()));
	Ok(())
}

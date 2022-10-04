use clap::{command, Command};
use color_eyre::eyre::WrapErr;
use eyre::Result;

mod log;

fn main() -> Result<()> {
	log::setup()?;

	let matches = command!()
		.propagate_version(true)
		.subcommand_required(true)
		.subcommand(
			Command::new("scan").about("Fetch and index sanction lists"),
		)
		.subcommand(Command::new("server").about("Start the compliance server"))
		.arg_required_else_help(true)
		.get_matches();

	match matches.subcommand() {
		Some(("scan", _)) => {
			barreleye_scan::start().wrap_err("Could not start server")?;
		}
		Some(("server", _)) => {
			barreleye_server::start().wrap_err("Could not start server")?;
		}
		_ => unreachable!("No command found"),
	}

	Ok(())
}

use bip39::{Mnemonic as Bip39Mnemonic, Language};
use failure::{format_err, Fallible};
use structopt::StructOpt;
use regex::Regex;

mod master_secret;
mod slip39;

pub use master_secret::MasterSecret;
pub use slip39::{Slip39, ShareInspector};
use sssmc39::Share;

#[derive(Debug, StructOpt)]
#[structopt(rename_all="kebab")]
struct Options {
	#[structopt(short, long, env = "SLIP39_PASSWORD", hide_env_values = true)]
	/// Password that is required in addition to the mnemonics to restore the master secret. Preferably
	/// provide it through an environment variable to avoid leaking it to other processes.
	password: String,
	#[structopt(subcommand)]
	sub_command: SubCommand,
}

fn parse_group_spec(src: &str) -> Fallible<(u8,u8)> {
	let pattern = Regex::new(r"^(?P<group_threshold>\d+)(-?of-?|:|/)(?P<group_members>\d+)$")?;
	let captures = pattern.captures(src).ok_or_else(|| format_err!("Group specification '{}' is invalid. Write something like '5of8'", src))?;
	let group_threshold: u8 = captures["group_threshold"].parse()?;
	let group_members: u8 = captures["group_members"].parse()?;
	Ok((group_threshold, group_members))
}

#[derive(Debug, StructOpt)]
enum SubCommand {
	/// Generate master secret and split it to parts
	/// 
	/// SLIP-0039 defines a 2-level split: The master secret is split into group secrets and then
	/// those are split further into member secrets. You can define the required and total number of
	/// members in each group, and also define how many groups are required to restore the master secret.
	Generate {
		#[structopt(short, long, default_value = "256")]
		/// Length of the master secret in bits
		entropy_bits: u16,
		#[structopt(flatten)]
		split_options: SplitOptions,
	},
	Split {
		#[structopt(short, long, env = "SLIP39_BIP39", hide_env_values = true)]
		/// BIP-0039 mnemonic to split. Use double quotes around it, but preferably provide it
		/// through an environment variable to avoid leaking it to other processes on this machine
		mnemonic: String,
		#[structopt(flatten)]
		split_options: SplitOptions,
	},
	Combine {},
	Inspect {
		mnemonic: String,
	}
}

#[derive(Debug, StructOpt)]
struct SplitOptions {
	#[structopt(short, long)]
	/// Number of groups required for restoring the master secret (by default all groups are required)
	required_groups: Option<u8>,
	#[structopt(short, long, default_value = "4")]
	/// The higher this number, the safer and slower the splitting and combining is
	iterations: u8,
	#[structopt(short, long("group"), parse(try_from_str = parse_group_spec), required=true, number_of_values=1)]
	/// Specify required and total number of members for each group (e.g. 8-of-15).
	/// Multiple groups need multiple occurences of this option.
	groups: Vec<(u8,u8)>
}

impl SplitOptions {
	fn split(&self, master_secret: &MasterSecret, password: &str) -> Fallible<Slip39> {
		let required_groups = self.required_groups
			.unwrap_or_else(|| self.groups.len() as u8);
		let slip39 = Slip39::new(
			required_groups,
			&self.groups,
			&master_secret,
			&password,
			self.iterations
		)?;
		Ok(slip39)
	}
}

fn main() -> Fallible<()> {
	let options = Options::from_args();
	match options.sub_command {
		SubCommand::Generate { entropy_bits, split_options } => {
			let master_secret = MasterSecret::new(entropy_bits)?;
			let slip39 = split_options.split(&master_secret, &options.password)?;
			println!("{}", serde_json::to_string_pretty(&slip39)?);
		}
		SubCommand::Split { mnemonic, split_options } => {
			let bip39 = Bip39Mnemonic::from_phrase(mnemonic, Language::English)?;
			let master_secret = MasterSecret::from(&bip39);
			let slip39 = split_options.split(&master_secret, &options.password)?;
			println!("{}", serde_json::to_string_pretty(&slip39)?);
		}
		SubCommand::Combine {} => {
			println!("Not so fast, young padawan!");
		}
		SubCommand::Inspect { mnemonic } => {
			let words = mnemonic.split_ascii_whitespace().map(str::to_owned).collect();
			let share = Share::from_mnemonic(&words)?;
			println!("{}", serde_json::to_string_pretty(&ShareInspector::from(&share))?);
		}
	}
	Ok(())
}
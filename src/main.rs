use failure::{format_err, Fallible};
use structopt::StructOpt;
use regex::Regex;

mod master_secret;
mod slip39;

pub use master_secret::MasterSecret;
pub use slip39::Slip39;

#[derive(Debug, StructOpt)]
#[structopt(rename_all="kebab")]
struct Options {
	#[structopt(short, long, env = "SLIP39_PASSWORD", hide_env_values = true)]
	/// Password that is required in addition to the mnemonics to restore the master secret
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
	},
	Split {},
	Combine {},
}

fn main() -> Fallible<()> {
	let options = Options::from_args();
	match options.sub_command {
		SubCommand::Generate { entropy_bits, required_groups, iterations, groups } => {
			let master_secret = MasterSecret::new(entropy_bits)?;
			let required_groups = required_groups.unwrap_or_else(|| groups.len() as u8);
			let slip39 = Slip39::new(required_groups, &groups, &master_secret, &options.password, iterations)?;
			println!("{}", slip39);
			Ok(())
		}
		SubCommand::Combine {} | SubCommand::Split {} => {
			println!("Not so fast, young padawan!");
			Ok(())
		}
	}
}
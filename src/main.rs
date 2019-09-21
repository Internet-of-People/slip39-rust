use failure::Fallible;

mod master_secret;
mod slip39;

pub use master_secret::MasterSecret;
pub use slip39::Slip39;

fn main() -> Fallible<()> {
	let master_secret = MasterSecret::new(256)?;
	let slip39 = Slip39::new(2, &vec![(1, 1), (1, 1), (3, 5), (2, 6)], &master_secret, "morpheus", 4)?;
	println!("{}", slip39);
	Ok(())
}
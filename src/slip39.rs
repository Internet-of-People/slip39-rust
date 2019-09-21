use sssmc39::*;
use std::fmt;

use crate::MasterSecret;

pub struct Slip39(Vec<GroupShare>);

impl Slip39 {
	#[allow(clippy::ptr_arg)] // until PR is merged
	pub fn new(
		group_threshold: u8,
		groups: &Vec<(u8, u8)>,
		master_secret: &MasterSecret,
		passphrase: &str,
		iteration_exponent: u8,
	) -> Result<Self, Error> {
		let group_shares = generate_mnemonics(group_threshold, groups, master_secret.as_ref(), passphrase, iteration_exponent)?;
		Ok(Self(group_shares))
	}
}

impl fmt::Display for Slip39 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let share_1_1 = &self.0[0].member_shares[0];
		writeln!(f, "Secret ({}-of-{})", share_1_1.group_threshold, share_1_1.group_count)?;
		for (g, group) in self.0.iter().enumerate() {
			writeln!(f, "  Group-{:02} ({}-of-{})", g + 1, group.member_threshold, group.member_shares.len())?;
			let mnemonics = group.mnemonic_list().expect("formatting a valid mnemonic should not get an error");
			for (m, mnemonic) in mnemonics.iter().enumerate() {
				writeln!(f, "    Mnemonic-{:02}-{:02}", g + 1, m + 1)?;
				let mut i = 0;
				for row in mnemonic.as_slice().chunks(3) {
					write!(f, "    ")?;
					for word in row {
						i += 1;
						let mut left = word.clone();
						let right = left.split_off(4);
						write!(f, "  {:02}:{}·{:·<4}", i, left, right)?;
					}
					writeln!(f)?;
				}
			}
		}
		Ok(())
	}
}

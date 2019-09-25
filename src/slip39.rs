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
		let group_shares = generate_mnemonics(
			group_threshold,
			groups,
			master_secret.as_ref(),
			passphrase,
			iteration_exponent
		)?;
		Ok(Self(group_shares))
	}

	pub fn iter(&self) -> std::slice::Iter<'_, GroupShare> {
		self.0.iter()
	}
}

struct ShareFormatter<'a>(&'a Share);

impl<'a> fmt::Display for ShareFormatter<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let share = self.0;
		let m = share.member_index + 1;
		let g = share.group_index + 1;
		writeln!(f, "Mnemonic-{:02}-{:02}", g, m)?;
		let mnemonic = share.to_mnemonic().expect("formatting a valid mnemonic should not get an error");
		let mut i = 0;
		for row in mnemonic.as_slice().chunks(3) {
			for word in row {
				i += 1;
				let mut left = word.clone();
				let right = left.split_off(4);
				write!(f, "  {:02}:{}·{:·<4}", i, left, right)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

impl<'a> fmt::Debug for ShareFormatter<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mnemonic = self.0.to_mnemonic().expect("formatting a valid mnemonic should not get an error");
		let words = mnemonic.iter()
			.map(|w| format!("{}{}", w.chars().next().unwrap().to_ascii_uppercase(), &w[1..4]))
			.collect::<Vec<_>>();
		writeln!(f, "{}", words.join(""))?;
		Ok(())
	}
}

struct GroupShareFormatter<'a>(&'a GroupShare);

impl<'a> fmt::Display for GroupShareFormatter<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let group = &self.0;
		let g = group.group_index + 1;
		writeln!(f, "Group-{:02} ({}-of-{})", g, group.member_threshold, group.member_shares.len())?;
		for share in group.member_shares.iter() {
			writeln!(f, "{}", ShareFormatter(share))?;
		}
		Ok(())
	}
}

impl<'a> fmt::Debug for GroupShareFormatter<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let group = &self.0;
		for share in group.member_shares.iter() {
			write!(f, "{:?}", ShareFormatter(share))?;
		}
		Ok(())
	}
}

impl fmt::Display for Slip39 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let share_1_1 = &self.0[0].member_shares[0];
		writeln!(f, "Secret ({}-of-{})", share_1_1.group_threshold, share_1_1.group_count)?;
		for group in self.iter() {
			writeln!(f, "{}", GroupShareFormatter(group))?;
		}
		Ok(())
	}
}


impl fmt::Debug for Slip39 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for group in self.iter() {
			write!(f, "{:?}", GroupShareFormatter(group))?;
		}
		Ok(())
	}
}

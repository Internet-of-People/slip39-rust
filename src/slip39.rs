use sssmc39::*;
use serde::{Serialize, Serializer};

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

#[derive(Serialize)]
struct ShareFormatter {
	group_index: u8,
	member_index: u8,
	mnemonic: String,
}

impl From<&Share> for ShareFormatter {
	fn from(share: &Share) -> Self {
		let mnemonic = share.to_mnemonic().expect("formatting a valid mnemonic should not get an error");
		Self {
			member_index: share.member_index + 1,
			group_index: share.group_index + 1,
			mnemonic: mnemonic.join(" "),
		}
	}
}

#[derive(Serialize)]
struct GroupShareFormatter {
	member_threshold: u8,
	member_count: u8,
	shares: Vec<ShareFormatter>,
}

impl From<&GroupShare> for GroupShareFormatter {
	fn from(group: &GroupShare) -> Self {
		Self {
			member_threshold: group.member_threshold,
			member_count: group.member_shares.len() as u8,
			shares: group.member_shares.iter().map(ShareFormatter::from).collect(),
		}
	}
}

#[derive(Serialize)]
struct Slip39Formatter {
	group_count: u8,
	group_threshold: u8,
	groups: Vec<GroupShareFormatter>,
}

impl<T: AsRef<[GroupShare]>> From<T> for Slip39Formatter {
    fn from(value: T) -> Self {
        let group_shares = value.as_ref();
        let share_1_1 = &group_shares[0].member_shares[0];
        Self {
            group_count: share_1_1.group_count,
            group_threshold: share_1_1.group_threshold,
            groups: group_shares.iter().map(GroupShareFormatter::from).collect(),
        }
    }
}

impl Serialize for Slip39 {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
		where S: Serializer
	{
		Slip39Formatter::from(&self.0).serialize(serializer)
	}
}

#[derive(Serialize)]
pub struct ShareInspector {
    identifier: u16,
    iterations: u8,
    group_threshold: u8,
    group_index: u8,
    member_threshold: u8,
    member_index: u8,
}

impl From<&Share> for ShareInspector {
    fn from(share: &Share) -> Self {
        Self {
            identifier: share.identifier,
            iterations: share.iteration_exponent,
            group_threshold: share.group_threshold,
            group_index: share.group_index + 1,
            member_threshold: share.member_threshold,
            member_index: share.member_index + 1,
        }
    }
}

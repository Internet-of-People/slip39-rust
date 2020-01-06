use sssmc39::*;

#[derive(Clone)]
pub struct MasterSecret(Vec<u8>);

impl MasterSecret {
    pub fn new(strength_bits: u16) -> Result<Self, Error> {
        use rand::{thread_rng, Rng};
        let proto_share = Share::new()?; // shamir::share::ShareConfig is not exported
        if strength_bits < proto_share.config.min_strength_bits {
            return Err(ErrorKind::Value(format!(
                "The requested strength of the master secret({} bits) must be at least {} bits.",
                strength_bits, proto_share.config.min_strength_bits,
            )))?;
        }
        if strength_bits % 16 != 0 {
            return Err(ErrorKind::Value(format!(
				"The requested strength of the master secret({} bits) must be a multiple of 16 bits.",
				strength_bits,
			)))?;
        }
        let mut v = vec![];
        for _ in 0..strength_bits as usize / 8 {
            v.push(thread_rng().gen());
        }
        Ok(Self(v))
    }
}

impl<T: AsRef<[u8]>> From<T> for MasterSecret {
    fn from(value: T) -> Self {
        Self(value.as_ref().to_owned())
    }
}

impl AsRef<Vec<u8>> for MasterSecret {
    fn as_ref(&self) -> &Vec<u8> {
        &self.0
    }
}

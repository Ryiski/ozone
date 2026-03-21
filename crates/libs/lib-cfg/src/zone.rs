use crate::records::{Records, SoaRecord};

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct Zone {
	#[knus(property)]
	pub domain: String,
	#[knus(property, default)]
	pub ttl: String,
	#[knus(child)]
	pub records: Records,
}

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct Adblock {
	#[knus(child)]
	pub soa: SoaRecord,
}

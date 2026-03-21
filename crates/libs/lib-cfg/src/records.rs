#[derive(Default, Debug, Clone, knus::Decode)]
pub struct NsRecord {
	#[knus(property)]
	pub name: String,
	#[knus(property)]
	pub value: String,
	#[knus(property, default)]
	pub ttl: String,
}

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct ARecord {
	#[knus(property)]
	pub name: String,
	#[knus(property)]
	pub value: String,
	#[knus(property, default)]
	pub ttl: String,
	#[knus(property, default)]
	pub ptr: bool,
}

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct AaaaRecord {
	#[knus(property)]
	pub name: String,
	#[knus(property)]
	pub value: String,
	#[knus(property, default)]
	pub ttl: String,
	#[knus(property, default = true)]
	pub ptr: bool,
}

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct CnameRecord {
	#[knus(property)]
	pub name: String,
	#[knus(property)]
	pub value: String,
	#[knus(property, default)]
	pub ttl: String,
}

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct MxRecord {
	#[knus(property)]
	pub name: String,
	#[knus(property)]
	pub value: String,
	#[knus(property)]
	pub priority: u32,
	#[knus(property, default)]
	pub ttl: String,
}

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct SrvRecord {
	#[knus(property)]
	pub name: String,
	#[knus(property)]
	pub value: String,
	#[knus(property)]
	pub priority: u32,
	#[knus(property)]
	pub port: u32,
	#[knus(property)]
	pub weight: u32,
	#[knus(property, default)]
	pub ttl: String,
}

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct TxtRecord {
	#[knus(property)]
	pub name: String,
	#[knus(property)]
	pub value: String,
	#[knus(property, default)]
	pub ttl: String,
}

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct PtrRecord {
	#[knus(property)]
	pub name: String,
	#[knus(property)]
	pub value: String,
	#[knus(property, default)]
	pub ttl: String,
	pub target_reverse_zone: String,
}

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct SoaRecord {
	#[knus(property)]
	pub ns: String,
	#[knus(property)]
	pub email: String,
	pub serial: String,
	#[knus(property)]
	pub refresh: String,
	#[knus(property)]
	pub retry: String,
	#[knus(property)]
	pub expire: String,
	#[knus(property)]
	pub min_ttl: String,
}

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct Records {
	#[knus(child)]
	pub soa: SoaRecord,
	#[knus(children(name = "ns"), default)]
	pub ns: Vec<NsRecord>,
	#[knus(children(name = "a"), default)]
	pub a: Vec<ARecord>,
	#[knus(children(name = "mx"), default)]
	pub mx: Vec<MxRecord>,
	#[knus(children(name = "aaaa"), default)]
	pub aaaa: Vec<AaaaRecord>,
	#[knus(children(name = "cname"), default)]
	pub cname: Vec<CnameRecord>,
	#[knus(children(name = "txt"), default)]
	pub txt: Vec<TxtRecord>,
	#[knus(children(name = "srv"), default)]
	pub srv: Vec<SrvRecord>,
	#[knus(children(name = "ptr"), default)]
	pub ptr: Vec<PtrRecord>,
}

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct RecordsRoot {
	#[knus(child, default)]
	pub records: Records,
}

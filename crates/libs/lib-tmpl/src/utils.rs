use chrono::Timelike;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct MaxLengths {
	pub ns: HashMap<String, usize>,
	pub a: HashMap<String, usize>,
	pub aaaa: HashMap<String, usize>,
	pub mx: HashMap<String, usize>,
	pub cname: HashMap<String, usize>,
	pub txt: HashMap<String, usize>,
	pub srv: HashMap<String, usize>,
	pub ptr: HashMap<String, usize>,
	pub soa: HashMap<String, usize>,
}

impl MaxLengths {
	pub fn get_padded(
		&self,
		record_type: &str,
		record_key: &str,
		record_value: &str,
		record_ttl: impl Into<String>,
	) -> String {
		let val = if record_value.is_empty() {
			record_ttl.into()
		} else {
			record_value.to_string()
		};

		let direction = if record_key == "Value" {
			lib_utils::net::PadType::Left
		} else {
			lib_utils::net::PadType::Right
		};

		let length = match record_type {
			"A" => *self.a.get(record_key).unwrap_or(&val.len()),
			"AAAA" => *self.aaaa.get(record_key).unwrap_or(&val.len()),
			"CNAME" => *self.cname.get(record_key).unwrap_or(&val.len()),
			"MX" => *self.mx.get(record_key).unwrap_or(&val.len()),
			"NS" => *self.ns.get(record_key).unwrap_or(&val.len()),
			"TXT" => *self.txt.get(record_key).unwrap_or(&val.len()),
			"SRV" => *self.srv.get(record_key).unwrap_or(&val.len()),
			"PTR" => *self.ptr.get(record_key).unwrap_or(&val.len()),
			"SOA" => *self.soa.get("Value").unwrap_or(&val.len()),
			_ => val.len(),
		};

		lib_utils::net::str_pad(&val, length, " ", direction)
	}
}
pub fn string_in_slice(needle: &str, haystack: &[String]) -> bool {
	haystack.contains(&needle.to_string())
}

pub fn generate_serial() -> String {
	let now = Utc::now();

	let date: u32 = now.format("%Y%m%d").to_string().parse().unwrap();

	let seconds = now.num_seconds_from_midnight();

	let serial = date * 100000 + seconds;

	serial.to_string()
}

pub fn calculate_max_record_component_length(zone: &lib_cfg::zone::Zone) -> MaxLengths {
	let mut max_lengths = MaxLengths::default();

	let mut ns_map = HashMap::new();
	ns_map.insert("Name".to_string(), 0);
	ns_map.insert("Value".to_string(), 0);
	ns_map.insert("TTL".to_string(), 0);

	let mut a_map = HashMap::new();
	a_map.insert("Name".to_string(), 0);
	a_map.insert("Value".to_string(), 0);
	a_map.insert("TTL".to_string(), 0);

	let mut aaaa_map = HashMap::new();
	aaaa_map.insert("Name".to_string(), 0);
	aaaa_map.insert("Value".to_string(), 0);
	aaaa_map.insert("TTL".to_string(), 0);

	let mut cname_map = HashMap::new();
	cname_map.insert("Name".to_string(), 0);
	cname_map.insert("Value".to_string(), 0);
	cname_map.insert("TTL".to_string(), 0);

	let mut txt_map = HashMap::new();
	txt_map.insert("Name".to_string(), 0);
	txt_map.insert("Value".to_string(), 0);
	txt_map.insert("TTL".to_string(), 0);

	let mut ptr_map = HashMap::new();
	ptr_map.insert("Name".to_string(), 0);
	ptr_map.insert("Value".to_string(), 0);
	ptr_map.insert("TTL".to_string(), 0);

	let mut mx_map = HashMap::new();
	mx_map.insert("Name".to_string(), 0);
	mx_map.insert("Value".to_string(), 0);
	mx_map.insert("TTL".to_string(), 0);
	mx_map.insert("Priority".to_string(), 0);

	let mut srv_map = HashMap::new();
	srv_map.insert("Name".to_string(), 0);
	srv_map.insert("Value".to_string(), 0);
	srv_map.insert("TTL".to_string(), 0);
	srv_map.insert("Priority".to_string(), 0);
	srv_map.insert("Weight".to_string(), 0);
	srv_map.insert("Port".to_string(), 0);

	for record in &zone.records.ns {
		ns_map.insert("Name".to_string(), ns_map["Name"].max(record.name.len()));
		ns_map.insert("Value".to_string(), ns_map["Value"].max(record.value.len()));
	}

	for record in &zone.records.a {
		a_map.insert("Name".to_string(), a_map["Name"].max(record.name.len()));
		a_map.insert("Value".to_string(), a_map["Value"].max(record.value.len()));
	}

	for record in &zone.records.aaaa {
		aaaa_map.insert("Name".to_string(), aaaa_map["Name"].max(record.name.len()));
		aaaa_map.insert(
			"Value".to_string(),
			aaaa_map["Value"].max(record.value.len()),
		);
	}

	for record in &zone.records.cname {
		cname_map.insert("Name".to_string(), cname_map["Name"].max(record.name.len()));
		cname_map.insert(
			"Value".to_string(),
			cname_map["Value"].max(record.value.len()),
		);
	}

	for record in &zone.records.mx {
		mx_map.insert("Name".to_string(), mx_map["Name"].max(record.name.len()));
		mx_map.insert("Value".to_string(), mx_map["Value"].max(record.value.len()));

		mx_map.insert(
			"Priority".to_string(),
			mx_map["Priority"].max(record.priority.to_string().len()),
		);
	}

	for record in &zone.records.srv {
		srv_map.insert("Name".to_string(), srv_map["Name"].max(record.name.len()));
		srv_map.insert(
			"Value".to_string(),
			srv_map["Value"].max(record.value.len()),
		);

		srv_map.insert(
			"Priority".to_string(),
			srv_map["Priority"].max(record.priority.to_string().len()),
		);
		srv_map.insert(
			"Weight".to_string(),
			srv_map["Weight"].max(record.weight.to_string().len()),
		);
		srv_map.insert(
			"Port".to_string(),
			srv_map["Port"].max(record.port.to_string().len()),
		);
	}

	for record in &zone.records.txt {
		txt_map.insert("Name".to_string(), txt_map["Name"].max(record.name.len()));
		txt_map.insert(
			"Value".to_string(),
			txt_map["Value"].max(record.value.len()),
		);
	}

	for record in &zone.records.ptr {
		ptr_map.insert("Name".to_string(), ptr_map["Name"].max(record.name.len()));
		ptr_map.insert(
			"Value".to_string(),
			ptr_map["Value"].max(record.value.len()),
		);
	}

	let soa = &zone.records.soa;
	let mut soa_map = HashMap::new();
	soa_map.insert("Value".to_string(), 0);
	soa_map.insert("Value".to_string(), soa_map["Value"].max(soa.refresh.len()));
	soa_map.insert("Value".to_string(), soa_map["Value"].max(soa.retry.len()));
	soa_map.insert("Value".to_string(), soa_map["Value"].max(soa.expire.len()));
	soa_map.insert("Value".to_string(), soa_map["Value"].max(soa.min_ttl.len()));

	max_lengths.soa = soa_map;
	max_lengths.ns = ns_map;
	max_lengths.a = a_map;
	max_lengths.aaaa = aaaa_map;
	max_lengths.cname = cname_map;
	max_lengths.mx = mx_map;
	max_lengths.srv = srv_map;
	max_lengths.txt = txt_map;
	max_lengths.ptr = ptr_map;

	max_lengths
}

pub fn build_reverse_zones(cfg: &lib_cfg::Config) -> Vec<String> {
	let mut reverse_zones = HashSet::new();
	for zone in &cfg.zones {
		for a_record in &zone.records.a {
			if a_record.value.contains("/") {
				let (_, _, rev_net, _) =
					lib_utils::net::split_v4_address_into_parts(&a_record.value);
				if !rev_net.is_empty() {
					reverse_zones.insert(rev_net + ".in-addr.arpa");
				}
			}
		}
		for aaaa_record in &zone.records.aaaa {
			if aaaa_record.value.contains("/") {
				let (_, _, rev_net, _) =
					lib_utils::net::split_v6_address_into_parts(&aaaa_record.value);
				if !rev_net.is_empty() {
					reverse_zones.insert(rev_net);
				}
			}
		}
	}

	let mut result: Vec<String> = reverse_zones.into_iter().collect();
	result.sort();
	result
}

pub fn build_reverse_zones_per_zone(cfg: &lib_cfg::Config) -> HashMap<String, Vec<String>> {
	let mut map: HashMap<String, Vec<String>> = HashMap::new();

	for zone in &cfg.zones {
		let mut revs = Vec::new();

		for a_record in &zone.records.a {
			let (_, _, rev_net, _) = lib_utils::net::split_v4_address_into_parts(&a_record.value);
			if !rev_net.is_empty() {
				revs.push(rev_net);
			}
		}

		for aaaa_record in &zone.records.aaaa {
			let (_, _, rev_net, _) =
				lib_utils::net::split_v6_address_into_parts(&aaaa_record.value);
			if !rev_net.is_empty() {
				revs.push(rev_net);
			}
		}

		revs.sort();
		map.insert(zone.domain.clone(), revs);
	}

	map
}

pub fn build_reverse_zones_by_domain(cfg: &lib_cfg::Config) -> HashMap<String, HashSet<String>> {
	let mut reverse_zones_map = HashMap::new();

	for zone in &cfg.zones {
		if reverse_zones_map.contains_key(&zone.domain) {
			continue;
		}
		let mut reverse_zones = HashSet::new();

		for a_record in &zone.records.a {
			let (_, _, rev_net, _) = lib_utils::net::split_v4_address_into_parts(&a_record.value);
			if !rev_net.is_empty() {
				reverse_zones.insert(rev_net);
			}
		}
		for aaaa_record in &zone.records.aaaa {
			let (_, _, rev_net, _) =
				lib_utils::net::split_v6_address_into_parts(&aaaa_record.value);
			if !rev_net.is_empty() {
				reverse_zones.insert(rev_net);
			}
		}

		reverse_zones_map.insert(zone.domain.clone(), reverse_zones);
	}

	reverse_zones_map
}

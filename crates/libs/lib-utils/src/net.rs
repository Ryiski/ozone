pub fn split_v4_address_into_parts(address: &str) -> (String, String, String, String) {
	let parts: Vec<&str> = address.split('/').collect();
	let address_part = parts[0].to_string();

	if parts.len() > 1 {
		let cidr: u8 = parts[1].parse().unwrap_or(0);

		let octets: Vec<i32> = address_part
			.split('.')
			.map(|x| x.parse().unwrap_or(0))
			.collect();

		let (network_array, host_array) = match cidr {
			0..=8 => (&octets[0..1], &octets[1..]),
			9..=16 => (&octets[0..2], &octets[2..]),
			17..=24 => (&octets[0..3], &octets[3..]),
			_ => (&octets[0..4], &octets[4..]),
		};

		let rev_net: Vec<String> = network_array.iter().rev().map(|v| v.to_string()).collect();

		let rev_host: Vec<String> = host_array.iter().rev().map(|v| v.to_string()).collect();

		return (
			address_part,
			parts[1].to_string(),
			rev_net.join("."),
			rev_host.join("."),
		);
	}

	(address_part, "".into(), "".into(), "".into())
}

pub fn split_v6_address_into_parts(address: &str) -> (String, String, String, String) {
	let a = new_ipv6_address(address);

	(a.ip, a.cidr, a.reverse_zone, a.reverse_record)
}

pub struct IPv6Address {
	pub ip: String,
	pub cidr: String,
	pub address_parts: Vec<String>,
	pub address_string: String,
	pub network_parts: Vec<String>,
	pub network_prefix: String,
	pub host_parts: Vec<String>,
	pub host_address: String,
	pub padded_host_address: String,
	pub reverse_zone: String,
	pub reverse_record: String,
}

pub fn new_ipv6_address(address: &str) -> IPv6Address {
	let mut address_part = String::new();
	let mut cidr_part = String::new();

	if address.contains('/') {
		let parts: Vec<&str> = address.split('/').collect();
		address_part = parts[0].to_string();
		cidr_part = parts.get(1).unwrap_or(&"").to_string();
	}

	let mut address_parts = vec!["0000".to_string(); 8];
	let mut network_parts = vec!["".to_string(); 8];
	let mut host_parts = vec!["".to_string(); 8];

	let mut network_prefix = String::new();
	let mut host_address = String::new();
	let mut padded_host_address = String::new();

	let split: Vec<&str> = address_part.split("::").collect();

	if split.len() == 1 {
		for (i, part) in split[0].split(':').enumerate() {
			address_parts[i] = pad_ipv6_octet(part);
		}
	} else if split.len() == 2 {
		let net = split[0];
		let host = split[1];

		let net_parts: Vec<&str> = net.split(':').collect();
		for (i, part) in net_parts.iter().enumerate() {
			let p = pad_ipv6_octet(part);
			network_parts[i] = p.clone();
			address_parts[i] = p;
		}

		network_prefix = network_parts.join(":").trim_end_matches(':').to_string();

		let host_split: Vec<&str> = host.split(':').collect();
		let offset = 8 - (host_split.len() + net_parts.len());

		for (i, part) in host_split.iter().enumerate() {
			let idx = i + net_parts.len() + offset;
			let p = pad_ipv6_octet(part);
			host_parts[idx] = p.clone();
			address_parts[idx] = p;
		}

		host_address = host_parts.join(":").trim_start_matches(':').to_string();

		padded_host_address = address_parts
			.join(":")
			.replacen(&network_prefix, "", 1)
			.trim_start_matches(':')
			.to_string();
	}

	let network_r = reverse_string(&network_prefix.replace(':', ""));
	let host_r = reverse_string(&padded_host_address.replace(':', ""));

	let rev_network = network_r
		.chars()
		.map(|c| c.to_string())
		.collect::<Vec<_>>()
		.join(".");
	let rev_host = host_r
		.chars()
		.map(|c| c.to_string())
		.collect::<Vec<_>>()
		.join(".");

	IPv6Address {
		ip: address_part,
		cidr: cidr_part,
		address_parts: address_parts.clone(),
		address_string: address_parts.join(":"),
		network_parts,
		network_prefix,
		host_parts,
		host_address,
		padded_host_address,
		reverse_zone: format!("{}.ip6.arpa", rev_network),
		reverse_record: rev_host,
	}
}

pub fn pad_ipv6_octet(octet: &str) -> String {
	str_pad(octet, 4, "0", PadType::Left)
}

pub fn reverse_string(s: &str) -> String {
	s.chars().rev().collect()
}

pub fn string_in_slice(target: &str, list: &[String]) -> bool {
	list.iter().any(|x| x == target)
}

pub fn reverse_int_slice(mut v: Vec<i32>) -> Vec<i32> {
	v.reverse();
	v
}

#[derive(Clone, Copy)]
pub enum PadType {
	Left,
	Right,
	Both,
}

pub fn str_pad(input: &str, pad_length: usize, pad_str: &str, pad_type: PadType) -> String {
	if input.len() >= pad_length {
		return input.to_string();
	}

	let pad_needed = pad_length - input.len();

	match pad_type {
		PadType::Right => {
			let mut out = input.to_string();
			while out.len() < pad_length {
				out.push_str(pad_str);
			}
			out.truncate(pad_length);
			out
		}
		PadType::Left => {
			let mut out = String::new();
			while out.len() + input.len() < pad_length {
				out.push_str(pad_str);
			}
			out.push_str(input);
			out[out.len() - pad_length..].to_string()
		}
		PadType::Both => {
			let left = pad_needed / 2;
			let _right = pad_needed - left;

			let mut out = String::new();

			while out.len() < left {
				out.push_str(pad_str);
			}

			out.truncate(left);
			out.push_str(input);

			while out.len() < pad_length {
				out.push_str(pad_str);
			}

			out.truncate(pad_length);
			out
		}
	}
}

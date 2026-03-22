use std::path::Path;

use crate::{
	templates::{self, GeneratorTemplate},
	utils,
};

pub fn generate_forward_zones(cfg: &lib_cfg::Config, out_dir: &Path) -> Result<(), String> {
	let serial = utils::generate_serial();

	for zone in &cfg.zones {
		let zone_ttl = if !zone.ttl.is_empty() {
			zone.ttl.clone()
		} else {
			cfg.defaults.ttl.clone()
		};

		// -----------------------------
		// A RECORDS
		// -----------------------------
		let mut a_records = Vec::new();

		for record in &zone.records.a {
			let ttl = if !record.ttl.is_empty() {
				record.ttl.clone()
			} else {
				zone_ttl.clone()
			};

			let mut value = record.value.clone();

			if value.contains('/') {
				let (ip, _, _, _) = lib_utils::net::split_v4_address_into_parts(&value);
				value = ip;
			}

			a_records.push(lib_cfg::records::ARecord {
				name: record.name.clone(),
				value,
				ttl,
				ptr: true,
			});
		}

		// -----------------------------
		// AAAA RECORDS
		// -----------------------------
		let mut aaaa_records = Vec::new();

		for record in &zone.records.aaaa {
			let ttl = if !record.ttl.is_empty() {
				record.ttl.clone()
			} else {
				zone_ttl.clone()
			};

			let mut value = record.value.clone();

			if value.contains('/') {
				let (ip, _, _, _) = lib_utils::net::split_v6_address_into_parts(&value);
				value = ip;
			}

			aaaa_records.push(lib_cfg::records::AaaaRecord {
				name: record.name.clone(),
				value,
				ttl,
				ptr: true,
			});
		}

		// -----------------------------
		// BUILD ZONE
		// -----------------------------
		let mut new_zone = lib_cfg::zone::Zone {
			domain: zone.domain.clone(),
			ttl: zone_ttl.clone(),
			records: lib_cfg::records::Records {
				soa: zone.records.soa.clone(),
				a: a_records,
				aaaa: aaaa_records,
				cname: zone.records.cname.clone(),
				mx: zone.records.mx.clone(),
				ns: zone.records.ns.clone(),
				txt: zone.records.txt.clone(),
				srv: zone.records.srv.clone(),
				ptr: vec![],
			},
		};

		new_zone.records.soa.serial = serial.clone();

		let max_lengths = utils::calculate_max_record_component_length(&new_zone);

		let path = out_dir.join(format!("{}.zone", zone.domain));

		let template_data = templates::zones::ZoneTmplData {
			zone: &new_zone,
			ttl: zone_ttl.clone(),
			max_lengths,
			ttl_swap_filter: templates::zones::TtlSwapFilter {
				default_ttl: zone_ttl,
			},
		};

		GeneratorTemplate::Zone(template_data).render_to_file(&path, cfg.dry_run)?;
	}

	Ok(())
}

use std::collections::HashMap;

pub fn generate_reverse_zones(cfg: &lib_cfg::Config, out_dir: &Path) -> Result<(), String> {
	let mut reverse_zones: HashMap<String, lib_cfg::zone::Zone> = HashMap::new();
	let mut used: HashMap<String, HashMap<String, bool>> = HashMap::new();

	let serial = utils::generate_serial();

	for zone in &cfg.zones {
		let zone_ttl = if !zone.ttl.is_empty() {
			zone.ttl.clone()
		} else {
			cfg.defaults.ttl.clone()
		};

		// ------------------------------
		// IPv4
		// ------------------------------
		for record in &zone.records.a {
			if record.ptr || record.name.contains('*') || !record.value.contains('/') {
				continue;
			}

			let (_, _, network, host) = lib_utils::net::split_v4_address_into_parts(&record.value);
			let rev_zone = format!("{}.in-addr.arpa", network);

			let zone_entry = reverse_zones.entry(rev_zone.clone()).or_insert_with(|| {
				used.insert(rev_zone.clone(), HashMap::new());

				lib_cfg::zone::Zone {
					domain: rev_zone.clone(),
					ttl: zone_ttl.clone(),
					records: lib_cfg::records::Records {
						soa: zone.records.soa.clone(),
						ptr: vec![],
						ns: zone.records.ns.clone(),
						..Default::default()
					},
				}
			});

			let used_hosts = used.get_mut(&rev_zone).unwrap();
			if used_hosts.contains_key(&host) {
				continue;
			}

			let name = if record.name != "@" {
				format!("{}.", record.name)
			} else {
				"".into()
			};

			zone_entry.records.ptr.push(lib_cfg::records::PtrRecord {
				name: host.clone(),
				value: format!("{}{}.", name, zone.domain),
				ttl: zone_ttl.clone(),
				..Default::default()
			});

			used_hosts.insert(host, true);
		}

		// ------------------------------
		// IPv6
		// ------------------------------
		for record in &zone.records.aaaa {
			if record.ptr || record.name.contains('*') || !record.value.contains('/') {
				continue;
			}

			let addr = lib_utils::net::new_ipv6_address(&record.value);
			let rev_zone = addr.reverse_zone;
			let host = addr.reverse_record;

			let zone_entry = reverse_zones.entry(rev_zone.clone()).or_insert_with(|| {
				used.insert(rev_zone.clone(), HashMap::new());

				lib_cfg::zone::Zone {
					domain: rev_zone.clone(),
					ttl: zone_ttl.clone(),
					records: lib_cfg::records::Records {
						soa: zone.records.soa.clone(),
						ptr: vec![],
						ns: zone.records.ns.clone(),
						..Default::default()
					},
				}
			});

			let used_hosts = used.get_mut(&rev_zone).unwrap();
			if used_hosts.contains_key(&host) {
				continue;
			}

			let name = if record.name != "@" {
				format!("{}.", record.name)
			} else {
				"".into()
			};

			zone_entry.records.ptr.push(lib_cfg::records::PtrRecord {
				name: host.clone(),
				value: format!("{}{}.", name, zone.domain),
				ttl: zone_ttl.clone(),
				..Default::default()
			});

			used_hosts.insert(host, true);
		}
	}

	// ------------------------------
	// Write files
	// ------------------------------
	for (name, zone) in reverse_zones {
		// let path = format!("{}/zones/{}.zone", base_path, name);

		let mut zone = zone;
		zone.records.soa.serial = serial.clone();

		let max_lengths = utils::calculate_max_record_component_length(&zone);
		let path = out_dir.join(format!("rev.{}.zone", name));

		let template_data = templates::zones::ZoneTmplData {
			zone: &zone,
			ttl: cfg.defaults.ttl.clone(),
			max_lengths,
			ttl_swap_filter: templates::zones::TtlSwapFilter {
				default_ttl: cfg.defaults.ttl.clone(),
			},
		};

		GeneratorTemplate::Zone(template_data).render_to_file(&path, cfg.dry_run)?;
	}

	Ok(())
}

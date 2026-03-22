use crate::{
	templates::{self, GeneratorTemplate},
	utils,
};
use rayon::prelude::*;
use std::{collections::HashSet, path::Path};

const HOSTS_BYTES: &[u8] = include_bytes!(concat!("../assets/adblock/hosts"));

pub fn generate_adblock_zone_template(cfg: &lib_cfg::Config, out_dir: &Path) -> Result<(), String> {
	let domains = parse_hosts(HOSTS_BYTES);

	let mut cname_records = Vec::with_capacity(domains.len());

	for d in domains {
		cname_records.push(lib_cfg::records::CnameRecord {
			name: d,
			value: ".".to_string(),
			..Default::default()
		});
	}

	let zone = lib_cfg::zone::Zone {
		domain: "rpz.local".into(),
		records: lib_cfg::records::Records {
			cname: cname_records,
			soa: lib_cfg::records::SoaRecord {
				ns: cfg.adblock.soa.ns.clone(),
				email: cfg.adblock.soa.email.clone(),
				serial: utils::generate_serial(),
				refresh: cfg.adblock.soa.refresh.clone(),
				retry: cfg.adblock.soa.retry.clone(),
				expire: cfg.adblock.soa.expire.clone(),
				min_ttl: cfg.adblock.soa.min_ttl.clone(),
			},
			..Default::default()
		},
		..Default::default()
	};

	let zone_ttl = if !zone.ttl.is_empty() {
		zone.ttl.clone()
	} else {
		cfg.defaults.ttl.clone()
	};

	let max_lengths = utils::calculate_max_record_component_length(&zone);

	let path = out_dir.join(format!("{}.zone", zone.domain));

	let template_data = templates::zones::ZoneTmplData {
		zone: &zone,
		ttl: zone_ttl.clone(),
		max_lengths,
		ttl_swap_filter: templates::zones::TtlSwapFilter {
			default_ttl: zone_ttl,
		},
	};

	GeneratorTemplate::Zone(template_data).render_to_file(&path, cfg.dry_run)?;

	Ok(())
}

pub fn parse_hosts(content: &[u8]) -> Vec<String> {
	let lines: Vec<&[u8]> = content.split(|&b| b == b'\n').collect();

	let domains: Vec<String> = lines
		.par_iter()
		.filter_map(|line| {
			let line = trim_ascii(line);

			if line.is_empty() || line[0] == b'#' {
				return None;
			}

			let mut parts = split_ascii_whitespace(line);

			parts.next()?;
			let domain = parts.next()?;

			let mut d = String::with_capacity(domain.len() + 1);
			d.push_str(std::str::from_utf8(domain).ok()?);

			if !d.ends_with('.') {
				d.push('.');
			}

			Some(d)
		})
		.collect();

	let mut seen = HashSet::new();
	domains
		.into_iter()
		.filter(|d| seen.insert(d.clone()))
		.collect()
}

fn trim_ascii(mut s: &[u8]) -> &[u8] {
	while let Some((&first, rest)) = s.split_first() {
		if first.is_ascii_whitespace() {
			s = rest;
		} else {
			break;
		}
	}

	while let Some((&last, rest)) = s.split_last() {
		if last.is_ascii_whitespace() {
			s = rest;
		} else {
			break;
		}
	}

	s
}

fn split_ascii_whitespace(s: &[u8]) -> impl Iterator<Item = &[u8]> {
	s.split(|b| b.is_ascii_whitespace())
		.filter(|p| !p.is_empty())
}

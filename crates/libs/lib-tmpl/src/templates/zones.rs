use crate::utils;
use askama::Template;

#[derive(Template)]
#[template(path = "zone.tmpl", escape = "none")]
pub struct ZoneTmplData<'a> {
	pub zone: &'a lib_cfg::zone::Zone,
	pub ttl: String,
	pub max_lengths: utils::MaxLengths,
	pub ttl_swap_filter: TtlSwapFilter,
}

pub struct TtlSwapFilter {
	pub default_ttl: String,
}

impl TtlSwapFilter {
	pub fn filter(&self, ttl: &str) -> String {
		if ttl.is_empty() {
			return self.default_ttl.clone();
		}

		ttl.to_owned()
	}
}

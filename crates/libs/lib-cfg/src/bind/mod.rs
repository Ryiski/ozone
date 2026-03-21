pub mod acl;
pub mod view;
use std::path::Path;

use crate::{
	bind::{acl::Acl, view::View},
	error::Result,
	include::Include,
};

#[derive(Default, Debug, Clone, knus::Decode)]
pub struct Bind {
	#[knus(children(name="acl"), default = vec![])]
	pub acls: Vec<Acl>,
	#[knus(children(name="view"), default = vec![])]
	pub views: Vec<View>,
	#[knus(children(name="include"), unwrap(argument), default = vec![])]
	pub includes: Vec<String>,
}

impl Bind {
	pub fn include(&mut self, config_path: &Path) -> Result<()> {
		Include::parse(config_path, &self.includes, |include| {
			#[allow(irrefutable_let_patterns)]
			match include {
				Include::Acl(acl) => {
					self.acls.push(acl);
				}
				Include::View(view) => {
					self.views.push(view);
				}
				Include::Zone(_zone) => todo!(),
			}

			Ok(())
		})?;

		Ok(())
	}
}

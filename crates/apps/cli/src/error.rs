#![allow(unused)]
use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
	Io(#[from] std::io::Error),
	Config(#[from] lib_cfg::error::CfgError),
	Other(String),
}

impl Error {
	pub fn other(msg: impl Into<String>) -> Self {
		Self::Other(msg.into())
	}
}

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

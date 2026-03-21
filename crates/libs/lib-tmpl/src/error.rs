use derive_more::From;

pub type Result<T> = core::result::Result<T, CfgError>;

#[derive(Debug, From)]
pub enum CfgError {
	Io(#[from] std::io::Error),
	Kdl(#[from] kdl::KdlError),
	Knus(#[from] knus::Error),
	Glob(#[from] glob::GlobError),
	GlobPatternError(#[from] glob::PatternError),
	Other(String),
}

impl CfgError {
	pub fn other(msg: impl Into<String>) -> Self {
		Self::Other(msg.into())
	}
}

impl core::fmt::Display for CfgError {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for CfgError {}

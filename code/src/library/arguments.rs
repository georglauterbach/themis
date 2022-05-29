// SPDX-License-Identifier: GPL-3.0-or-later

// see https://github.com/clap-rs/clap/blob/v3.1.2/examples/derive_ref/README.md
// for the clap derive reference

#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, name = "yourprojectname")]
pub struct Arguments
{
	#[clap(flatten)]
	verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,
}

impl Arguments
{
	/// #### Parse the Global Log Level from User Provided Arguments
	///
	/// With [`clap_verbosity_flag`], we can just provide a default log level
	/// ([`clap_verbosity_flag::InfoLevel`]), and depending on the user choice (`-q`,
	/// `-qq`, `-v` or `-vv`), the correct log level is returned.
	#[must_use]
	pub fn parse_log_level(&self) -> log::Level
	{
		self.verbosity.log_level().unwrap_or(log::Level::Error)
	}
}

// SPDX-License-Identifier: GPL-3.0-or-later

/// ### The Global Logger
///
/// This static variable is used by the [`log`] crate for logging all messages.
static LOGGER: Logger = Logger;

lazy_static::lazy_static! {
	/// ### Check if Output is a TTY
	///
	/// This variable is used by the logger to check whether the output
	/// is a TTY. If not, we do not enable colors.
	static ref IS_TTY: bool = atty::is(atty::Stream::Stdout);
}

/// ### The Main Test Runner Logger
///
/// This structure holds associated function that provide logging. The
/// [`log::Log`] trait is implemented for this structure.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
struct Logger;

impl Logger
{
	/// ### Wrapper for [`log::Log::log`]
	///
	/// Because the [`termcolor`] streams as well as the [`write!`] macro return a
	/// [`Result`], using the `?` operator is cleverly used here. This is basically
	/// just a wrapper for [`log::Log::log`].
	fn log(&self, record: &log::Record) -> Result<(), Box<dyn std::error::Error>>
	{
		use std::io::Write;
		use termcolor::{
			Color,
			ColorChoice,
			ColorSpec,
			StandardStream,
			WriteColor,
		};
		use log::Level;

		if !log::Log::enabled(&self, record.metadata()) {
			return Ok(());
		}

		// https://coolors.co/fb4934-fabd2f-458588-83a598-8f8f8f
		let (log_level, color) = match record.level() {
			Level::Error => ("ERROR", Color::Red),
			Level::Warn => ("WARNING", Color::Yellow),
			Level::Info => ("INFO", Color::Blue),
			Level::Debug => ("DEBUG", Color::Green),
			Level::Trace => ("TRACE", Color::Cyan),
		};

		let color_choice = if *IS_TTY {
			ColorChoice::Always
		} else {
			ColorChoice::Never
		};

		let mut stdout = StandardStream::stdout(color_choice);
		let date_time = time::OffsetDateTime::now_local()?;

		stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
		write!(&mut stdout, "{:>9}  ", log_level)?;
		stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
		write!(
			&mut stdout,
			"{:02}/{:02}/{:02} {:02}:{:02}:{:02}",
			date_time.day(),
			date_time.month() as u8,
			date_time.year(),
			date_time.hour(),
			date_time.minute(),
			date_time.second(),
		)?;
		stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
		write!(&mut stdout, " | ")?;
		stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
		writeln!(&mut stdout, "{}", record.args())?;

		Ok(())
	}
}

impl log::Log for Logger
{
	fn enabled(&self, metadata: &log::Metadata) -> bool { metadata.level() <= log::max_level() }

	fn log(&self, record: &log::Record)
	{
		if let Err(error) = self.log(record) {
			println!("ERROR                      | {}", error);
		}
	}

	fn flush(&self) {}
}

/// ### Show Initial Information
///
/// This function sets the log level and displays version and
/// bootloader information. The default log level chosen if [`None`] is provided
/// is "Info".
pub fn initialize(log_level: log::Level)
{
	log::set_max_level(log_level.to_level_filter());
	log::set_logger(&LOGGER).expect("Log should not have already been set");
}

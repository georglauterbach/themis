// SPDX-License-Identifier: GPL-3.0-or-later

/// ## Command Line Arguments
///
/// This module handles command lines arguments, utilizing [`clap`].
pub mod arguments;

/// ## Foreign Function Interface
///
/// This module contains and makes code available, which was created in another
/// programming language.
pub mod ffi;

/// ### Global Log
///
/// Provides logging with the help of [`log`].
pub mod logger;

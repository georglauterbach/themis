// SPDX-License-Identifier: GPL-3.0-or-later

// Clippy lint target one. Enables all lints that are on by
// default (correctness, suspicious, style, complexity, perf) .
#![deny(clippy::all)]
// Clippy lint target two. Enables lints which are rather strict
// or have occasional false positives.
#![deny(clippy::nursery)]
// Clippy lint target three. Enables new lints that are still
// under development
#![deny(clippy::pedantic)]
// Clippy lint target four. Enable lints for the cargo manifest
// file, a.k.a. Cargo.toml.
#![deny(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
// Lint target for code documentation. This lint enforces code
// documentation on every code item.
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
// Lint target for code documentation. When running `rustdoc`,
// show an error when using broken links.
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
// All other, generic lint targets that were not
// covered previously
#![deny(missing_debug_implementations)]

//! # _yourprojectname_

/// ## The Application Library
///
/// While `lib.rs` is automatically identified by `cargo` to be a library, we crate the
/// directory `library` to organize our actual library. It is important that `library` is
/// owned by `lib.rs` (`mod library;`), and not by `main.rs`. This way, `main.rs` can call
/// re-exported modules with `use yourprojectname::<RE-EXPORTED MODULE>`.
mod library;

pub use library::{
	arguments,
	ffi,
	logger,
};

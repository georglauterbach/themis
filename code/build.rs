// SPDX-License-Identifier: GPL-3.0-or-later

/// ### Library Name
///
/// This variable holds the name of the library that is created for calls on the FFI. It
/// is a strict subset of the filename that is being created, i.e. it is missing the
/// suffix (the file extension) and the `lib` prefix.
const FFI_LIBRARY_NAME: &str = "ffi";

/// ### Base Directory
///
/// This is the directory that one can use to create absolute paths. It is the exact
/// directory the `Cargo.toml` file is located in.
const CARGO_MANIFEST_DIRECTORY: &str = env!("CARGO_MANIFEST_DIR");

/// ### The `bindgen` Output
///
/// These are the Rust bindings that we use for calls on the FFI. It is used with the
/// `include!` macro. When changing this name, make sure to also change it in the source
/// code.
const BINDINGS_FILE_NAME: &str = "bindings.rs";

/// ### The Header File to Create Bindings From
///
/// This header file is used to create all Rust bindings.
const FFI_HEADER_FILE: &str = "src/ffi.hh";

/// ### The Directory Root for the FFI Code
///
/// Contains the directory name of the root in which FFI code resides.
const FFI_CODE_DIRECTORY: &str = const_format::concatcp!(CARGO_MANIFEST_DIRECTORY, "/src/ffi/");

/// ### The Builder Function
///
/// This function is run when _Cargo_ builds the project. We
///
/// 1. build the C/C++ FFI files and create a static library
/// 2. create Rust bindings from the FFI files
/// 3. add the static library to the arguments for the linker
fn main() -> Result<(), Box<dyn std::error::Error>>
{
	// miscellaneous variable setup
	let bindings_file = std::path::PathBuf::from(std::env::var("OUT_DIR")?).join(BINDINGS_FILE_NAME);

	// add information about then to re-compile
	println!("cargo:rerun-if-changed={}", FFI_CODE_DIRECTORY);
	println!("cargo:rerun-if-changed=build.rs");

	// build all FFI files
	if !std::process::Command::new("make")
		.current_dir(&FFI_CODE_DIRECTORY)
		.status()?
		.success()
	{
		return Err(Box::from("Could not build FFI files"));
	}

	// create Rust bindings from C(++) sources
	bindgen::Builder::default()
		.header(format!("{}{}", FFI_CODE_DIRECTORY, FFI_HEADER_FILE))
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Could not create Rust bindings from C++ FFI header file")
		.write_to_file(bindings_file)?;

	// add path and name information about the (static) library that is to be linked
	println!("cargo:rustc-link-lib=static={}", FFI_LIBRARY_NAME);
	println!(
		"cargo:rustc-link-search={}",
		format_args!("{}/{}/", FFI_CODE_DIRECTORY, "build")
	);

	// ? if you use the C++ standard library, you need to provide
	// ? the C++ shared standard library to the `rustc` linker
	// ? (which is the system linker `ld`). Otherwise, these
	// ? function calls cannot be resolved.
	println!("cargo:rustc-link-lib=stdc++");

	Ok(())
}

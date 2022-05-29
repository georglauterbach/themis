# -----------------------------------------------
# ----  Just  -----------------------------------
# ----  https://github.com/casey/just  ----------
# -----------------------------------------------

set shell             := [ "bash", "-eu", "-o", "pipefail", "-c" ]
set dotenv-load       := false

export ROOT_DIRECTORY := justfile_directory()
CARGO                 := 'cd code && cargo'

# show a dedicated help message
help:
	#! /bin/bash

	printf 'Tools:\n    '
	rustc --version 2>/dev/null || printf "'rustc' not installed or in \$PATH\n"
	printf '    '
	cargo --version 2>/dev/null || printf "'cargo' not installed or in \$PATH\n"
	printf '     %s\n\n' "$(just --version)"
	just --list
	printf '\n'

# -----------------------------------------------
# ----  Build and Test  -------------------------
# -----------------------------------------------

# compile the kernel
@build *arguments:
	{{CARGO}} build {{arguments}}

# run the kernel in QEMU
@run *arguments: (build '--quiet')
	{{CARGO}} run --quiet {{arguments}}

# remove the kernel/target/ directory
@clean:
	{{CARGO}} clean

# lint against rustfmt and clippy
@check *arguments: format
	{{CARGO}} check
	{{CARGO}} clippy --lib --all-features -- -D warnings
	{{CARGO}} clippy --bin yourprojectname --all-features -- -D warnings
	{{CARGO}} doc

# run tests workspace members
@test *arguments:
	{{CARGO}} test {{arguments}}

# -----------------------------------------------
# ----  Format and Lint  ------------------------
# -----------------------------------------------

# format the Rust code with rustfmt
@format:
	{{CARGO}} fmt --message-format human

alias fmt := format

# generically lint the whole code base
@lint *arguments:
	- bash "{{ROOT_DIRECTORY}}/scripts/lint.sh" {{arguments}}

# -----------------------------------------------
# ----  Documentation  --------------------------
# -----------------------------------------------

# build the code documentation with Cargo 
@documentation arguments='':
	cargo doc --lib --document-private-items {{arguments}}

alias doc := documentation
alias docs := documentation

<p align="center">
  <img src=".github/resources/logo.png" height="auto" width="20%" />
</p>

# Themis

_Themis_ is a template repository for _Rust_ projects that aspire **high code quality** and use GitHub's CI/CD. In Greek mythology, _Themis_ is the personification of order, law, and custom. She is also associated with oracles and prophecies, including the _Oracle of Cargo_.

## Features

By using this template, you get the following features

1. Functioning CI/CD pipeline for [GitHub Actions], including unit- and integration tests, linting, general formatting, [Dependabot] and security audits.
2. Complete Rust setup, including toolchain setup, `rustfmt` configuration and a pre-filles `Cargo.toml`
3. A dual library-binary crate with log and CLI arguments setup
4. General formatting guidelines set by `.editorconfig`, `.prettierrc` and `.rustfmt.toml`
5. Proper setup for shell scripts and git (including `.gitattributes` and a `.gitignore`)
6. A [`Justfile`] for automation (instead of using `make`)

All in all, this template contains all you need to get started with high-quality Rust on GitHub actions.

## Licensing

This project is licensed under the [GNU General Public License v3], except for those parts (lines of code from libraries used in this project) already licensed under other licenses.

[//]: # (Links)

[GitHub Actions]: https://github.com/features/actions
[Dependabot]: https://github.com/dependabot
[`Justfile`]: https://github.com/casey/just
[GNU General Public License v3]: https://www.gnu.org/licenses/gpl-3.0.txt

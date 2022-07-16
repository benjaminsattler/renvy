//! This crate provides easy-to-use tool for managing settings files
//! that are based on templates (e.g. `.env` and `.env.dist` files).
//!
//! This crate assumes that the files it manages are `key=value` pairs which it
//! understands. It will add keys to the settings if they exist in the template file
//! and optionally it'll remove keys from the settings file if they are absent from
//! the template file.
//!
//! ## Installation
//!
//! ```sh
//! cargo install renvy
//! ```
//! This will install the binary renvy in your cargo installation root (default `~/.cargo/bin`).
//!
//! ## Usage
//!
//! Invoke renvy with these arguments
//!
//! ```sh
//! renvy [OPTIONS] <SETTINGS> <DEFAULTS>
//!
//! ARGS:
//!     <SETTINGS>
//!     <DEFAULTS>
//!
//! OPTIONS:
//!     -c, --cleanup
//!     -h, --help       Print help information
//!     -V, --version    Print version information
//! ```
//!
//! A typical invocation would look like this:
//!
//! ```sh
//! renvy -c .env .env.dist
//! ```
//!
//! This will ensure that all keys in `.env.dist` exist in `.env` by adding
//! all missing keys to `.env`. Since the `-c` flag is given it will also
//! remove all keys from `.env` that are not present in `.env.dist`.
//!
//! ## API
//!
//! This binary is based on the library `librenvy`. librenvy enables developers
//! to use the same functionality in their applications. For more information
//! about librenvy install it with `cargo install librenvy` or head over
//! to https://renvy.benjaminsattler.net.
//!
use clap::Parser;
use renvy;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(value_parser)]
    settings: String,

    #[clap(value_parser)]
    defaults: String,

    #[clap(short, long)]
    cleanup: bool,
}

fn main() {
    let matches = Args::parse();

    let (settings, defaults) = match (
        renvy::read_file(&matches.settings),
        renvy::read_file(&matches.defaults),
    ) {
        (Ok(settings), Ok(defaults)) => (settings, defaults),
        (Ok(_), Err(_)) => panic!("Error reading defaults file"),
        (Err(_), Ok(_)) => panic!("Error reading settings file"),
        (Err(_), Err(_)) => panic!("Error reading input files"),
    };

    let (settings, defaults) = (renvy::deserialize(&settings), renvy::deserialize(&defaults));

    let merged = renvy::merge(settings, defaults, Some(matches.cleanup));

    let merged = renvy::serialize(&merged);

    renvy::write_file(&matches.settings, &merged).unwrap()
}

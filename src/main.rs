use std::collections::HashMap;

mod merge;
mod io;
mod serde;

fn main() {
    let settings = merge::Settings::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

    let defaults = merge::Settings::from([
        ("domain".into(), Some("https://example.com".into())),
        ("port".into(), Some("433".into())),
    ]);

    let merged = merge::merge(settings, defaults, None);

    println!("{:?}", merged);
}

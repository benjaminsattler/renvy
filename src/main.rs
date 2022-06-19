use std::collections::HashMap;

mod merge;

fn main() {
    let settings = HashMap::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

    let defaults = HashMap::from([
        ("domain".into(), Some("https://example.com".into())),
        ("port".into(), Some("433".into())),
    ]);

    let merged = merge::merge(settings, defaults, None);

    println!("{:?}", merged);
}

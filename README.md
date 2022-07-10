# librenvy

This crate provides easy-to-use functionality for managing settings files
that are based on templates (e.g. `.env` and `.env.dist` files).

This crate assumes that the files it manages are `key=value` pairs which it
understands. It will add keys to the settings if they exist in the template file
and optionally it'll remove keys from the settings file if they are absent from
the template file.

### Installation
```sh
cargo install librenvy
```

### Parsing files

This example shows how you can easily read existing files into [`Settings`] struct's
with the function [`deserialize()`].
[`Settings`] are the foundation for any further processing by this crate.

```rust
// read_file returns a std::io::Result<String>
let settings = renvy::read_file("./.env");
assert!(settings.is_ok());
if let Ok(settings) = settings {
    // renvy::deserialize consumes this String and returns an instance of renvy::Settings
    let settings: renvy::Settings = renvy::deserialize(&settings);
    println!("Number of settings read: {}", &settings.len());
    settings.iter().for_each(|(key, value)| {
        println!("{:?}: {:?}\n", key, value);
    });
} else {
    println!("Unable to read settings file!");
}

// you can use the same function for reading settings and template files
let defaults = renvy::read_file("./.env.dist");
assert!(defaults.is_ok());
if let Ok(defaults) = defaults {
    // we're reusing the same data structure for defaults
    let defaults: renvy::Settings = renvy::deserialize(&defaults);
    println!("Number of defaults read: {}", &defaults.len());
    defaults.iter().for_each(|(key, value)| {
        println!("{:?}: {:?}\n", key, value);
    });
} else {
    println!("Unable to read defaults file!");
}
```

### Updating settings based on template

[`merge()`] allows you to update settings based on an existing template.
New keys in the template will be added to the settings with the default
value given in the template.

```rust
// settings file contains 1 key-value pair
let settings = renvy::Settings::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

// defaults file contains 1 other key-value pair
let defaults = renvy::Settings::from([("port".into(), Some("433".into()))]);

// merging defaults with settings will result in a new object merge::settings
// that contains 2 key-value pairs:
//
// - "domain" because it was already present in `settings`
// - "port" because it was present in defaults
let merged = renvy::merge(settings, defaults, None);

assert!(merged.get("domain").is_some());
assert_eq!(merged.get("domain").unwrap(), &Some(String::from("https://benjaminsattler.net")));
assert!(merged.get("port").is_some());
assert_eq!(merged.get("port").unwrap(), &Some(String::from("433")));
```

### Cleaning up extra settings

You can also remove any key from the user settings that are missing from defaults
by passing `Some(true)` to the optional third parameter of [`merge()`]:

```rust
// settings file contains 1 key-value pair
let settings = renvy::Settings::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

// defaults file contains 1 other key-value pair
let defaults = renvy::Settings::from([("port".into(), Some("433".into()))]);

// merging defaults with settings will result in a new object merge::settings
// that contains only 1 key-value pair. The key "domain" will be removed because
// it does not exist in the defaults:
//
// - "port" because it was present in defaults
let merged = renvy::merge(settings, defaults, Some(true));

assert!(merged.get("domain").is_none());
assert!(merged.get("port").is_some());
assert_eq!(merged.get("port").unwrap(), &Some(String::from("433")));
```

### Writing merged results back to a file

The final step is to persist the merged result back into the settings file
by invoking [`serialize()`] and [`write_file()`].

```rust
// first we're serializing the object renvy::Settings into a String
let merged = renvy::serialize(&merged);

// then we take that String and write it back into the original settings file
let result = renvy::write_file("./.env", &merged);
// write_file returns a std::io::Result<()>
assert!(result.is_ok());
```

License: MIT

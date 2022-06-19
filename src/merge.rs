use std::collections::HashMap;

pub type Key = String;
pub type Value = Option<String>;

pub fn merge(settings: HashMap<Key, Value>, defaults: HashMap<Key, Value>, clean: Option<bool>) -> HashMap<Key, Value> {
    let mut result: HashMap<Key, Value> = settings.clone();
    let clean = if let Some(x) = clean { x } else { false };

    if clean {
        result.retain(|key, _| defaults.contains_key(key));
    }

    for default in defaults {
        let entry = result.entry(default.0);
        entry.or_insert(default.1);
    }

    result
}

#[cfg(test)]
mod test {
    use crate::merge;
    use std::collections::HashMap;

    #[test]
    fn merge_adds_new_defaults() {
        let settings =
            HashMap::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

        let defaults = HashMap::from([
            ("port".into(), Some("433".into())),
        ]);

        let merged = merge::merge(settings, defaults, None);

        assert!(merged.get("port").is_some());
        assert_eq!(merged.get("port").unwrap(), &Some(String::from("433")));
    }

    #[test]
    fn merge_keeps_existing_settings_with_defaults() {
        let settings =
            HashMap::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

        let defaults = HashMap::from([
            ("domain".into(), Some("https://example".into())),
        ]);

        let merged = merge::merge(settings, defaults, None);

        assert!(merged.get("domain").is_some());
        assert_eq!(merged.get("domain").unwrap(), &Some(String::from("https://benjaminsattler.net")));
    }

    #[test]
    fn merge_keeps_settings_without_defaults_if_cleaning_is_default() {
        let settings =
            HashMap::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

        let defaults = HashMap::from([
            ("port".into(), Some("433".into())),
        ]);

        let merged = merge::merge(settings, defaults, None);

        assert!(merged.get("domain").is_some());
        assert_eq!(merged.get("domain").unwrap(), &Some(String::from("https://benjaminsattler.net")));
    }

    #[test]
    fn merge_keeps_settings_without_defaults_if_not_cleaning() {
        let settings =
            HashMap::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

        let defaults = HashMap::from([
            ("port".into(), Some("433".into())),
        ]);

        let merged = merge::merge(settings, defaults, Some(false));

        assert!(merged.get("domain").is_some());
        assert_eq!(merged.get("domain").unwrap(), &Some(String::from("https://benjaminsattler.net")));
    }

    #[test]
    fn merge_discards_settings_without_defaults_if_cleaning() {
        let settings =
            HashMap::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

        let defaults = HashMap::from([
            ("port".into(), Some("433".into())),
        ]);

        let merged = merge::merge(settings, defaults, Some(true));

        assert!(merged.get("domain").is_none());
    }
}

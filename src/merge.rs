use std::collections::BTreeMap;

/// Denotes the type for elements that serve as keys in settings files.
///
/// This type is an alias for keys, e.g. the part of a line of a settings file
/// in front of the first `=`. Even though this is just an alias to `String`
/// for now, this provides better type safety and lets the compiler know what
/// you intend to do with a particular variable. This type is used in the
/// complex type [`Settings`].
///
/// For an example how to use `Key` please refer to the [example section of `Settings`](crate::Settings#how-to-use-settings).
pub type Key = String;

/// Denotes the type for elements that serve as values in settings files.
///
/// This type is an alias for optional values, e.g. the part of a line of a settings
/// file after the first `=`. It's an `Option` because a settings line might end
/// with the first `=` which is when the value of that line would be `None`. As soon
/// as there is any contents after the first `=`, the value will be `Some(X)`, where `X`
/// represents that data.
///
/// For an example how to use `Value` please refer to the [example section of `Settings`](crate::Settings#how-to-use-settings).
pub type Value = Option<String>;

/// Denotes a set of settings as a simple sorted map of Key-Value pairs.
///
/// Each entry in this map consists of a key of type [`Key`] and a value of type [`Option<Value>`].
/// For more information about why values are wrapped in [`Option`] please refer to the
/// documentation of the type [`Value`].
///
/// ## How to use `Settings`
///
/// Since `Settings` is just an alias for [`BTreeMap`], you can construct them fairly easily from an array of tuples:
///
/// ```
/// // Create a new Settings object with 1 key-value pair "url=https://example.com"
/// let my_settings_one = renvy::Settings::from([("url".into(), Some("https://example.com".into()))]);
/// assert_eq!(my_settings_one.get("url").unwrap(), &Some("https://example.com".into()));
///
/// // Create a new Settings object with 1 key-value pair where the value is empty "url="
/// let my_settings_one = renvy::Settings::from([("url".into(), None)]);
/// assert_eq!(my_settings_one.get("url").unwrap(), &None);
/// ```
///
/// Besides that, [`crate::deserialize()`] will also return an instance of `Settings`:
///
/// ```
/// // Settings objects will also be returned from renvy::deserialize()
/// let my_settings_deserialized = renvy::deserialize("url=https://example.com".into());
/// assert_eq!(my_settings_deserialized.get("url".into()).unwrap(), &Some("https://example.com".into()));
/// ```
pub type Settings = BTreeMap<Key, Value>;

/// Merges two instances of [`Settings`] together so that the following rules are satisfied:
/// - all key-value pairs on `defaults` that are missing on `settings` will be added
/// - existing key-value pairs of `settings` retain their value
/// - if the parameter `clean` receives `Some(true)`, then any key-value pair on `settings`
///   which is missing from `defaults` will be removed
///
/// ## Examples
///
/// Default behaviour when `clean` is `None`. This is the same behaviour like passing
/// `Some(false)` explicitly.
///
/// ```
/// // "ssl" exists in both objects, it's "true" here
/// let settings = renvy::Settings::from([
///     ("url".into(), Some(String::from("https://example.com"))),
///     ("ssl".into(), Some(String::from("true")))
/// ]);
///
/// // "ssl" is "false" here
/// let defaults = renvy::Settings::from([
///     ("port".into(), None),
///     ("ssl".into(), Some(String::from("false")))
/// ]);
///
/// let merged = renvy::merge(settings, defaults, None);
///
/// // "ssl" remains "true", "port" is added, and "url" is left intact
/// assert_eq!(merged.get("url".into()).unwrap(), &Some(String::from("https://example.com")));
/// assert_eq!(merged.get("ssl".into()).unwrap(), &Some(String::from("true")));
/// assert_eq!(merged.get("port".into()).unwrap(), &None);
/// ```
///
/// Behaviour when `clean` is disabled with `Some(false)`: Extra keys in `settings`
/// remain untouched. This is the default behaviour that is also applied when `clean`
/// is empty (`None`).
///
/// ```
/// // "ssl" exists in both objects, it's "true" here
/// let settings = renvy::Settings::from([
///     ("url".into(), Some(String::from("https://example.com"))),
///     ("ssl".into(), Some(String::from("true")))
/// ]);
///
/// // "ssl" is "false" here
/// let defaults = renvy::Settings::from([
///     ("port".into(), None),
///     ("ssl".into(), Some(String::from("false")))
/// ]);
///
/// let merged = renvy::merge(settings, defaults, Some(false));
///
/// // "ssl" remains "true", "port" is added, and "url" is left intact
/// assert_eq!(merged.get("url".into()).unwrap(), &Some(String::from("https://example.com")));
/// assert_eq!(merged.get("ssl".into()).unwrap(), &Some(String::from("true")));
/// assert_eq!(merged.get("port".into()).unwrap(), &None);
/// ```
///
/// Behaviour when `clean` is enabled with `Some(true)`: Extra keys in `settings`
/// are being removed so that only keys that exist in `defaults` remain in `settings`.
///
/// ```
/// // "ssl" exists in both objects, it's "true" here
/// // "url" exists only in "settings".
/// let settings = renvy::Settings::from([
///     ("url".into(), Some(String::from("https://example.com"))),
///     ("ssl".into(), Some(String::from("true")))
/// ]);
///
/// // "ssl" is "false" here
/// let defaults = renvy::Settings::from([
///     ("port".into(), None),
///     ("ssl".into(), Some(String::from("false")))
/// ]);
///
/// let merged = renvy::merge(settings, defaults, Some(true));
///
/// // "ssl" remains "true", "port" is added, and "url" is removed
/// assert_eq!(merged.get("url".into()), None);
/// assert_eq!(merged.get("ssl".into()).unwrap(), &Some(String::from("true")));
/// assert_eq!(merged.get("port".into()).unwrap(), &None);
/// ```
///
pub fn merge(settings: Settings, defaults: Settings, clean: Option<bool>) -> Settings {
    let mut result: Settings = settings;
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

    #[test]
    fn merge_adds_new_defaults() {
        let settings =
            merge::Settings::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

        let defaults = merge::Settings::from([("port".into(), Some("433".into()))]);

        let merged = merge::merge(settings, defaults, None);

        assert!(merged.get("port").is_some());
        assert_eq!(merged.get("port").unwrap(), &Some(String::from("433")));
    }

    #[test]
    fn merge_keeps_existing_settings_with_defaults() {
        let settings =
            merge::Settings::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

        let defaults = merge::Settings::from([("domain".into(), Some("https://example".into()))]);

        let merged = merge::merge(settings, defaults, None);

        assert!(merged.get("domain").is_some());
        assert_eq!(
            merged.get("domain").unwrap(),
            &Some(String::from("https://benjaminsattler.net"))
        );
    }

    #[test]
    fn merge_keeps_settings_without_defaults_if_cleaning_is_default() {
        let settings =
            merge::Settings::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

        let defaults = merge::Settings::from([("port".into(), Some("433".into()))]);

        let merged = merge::merge(settings, defaults, None);

        assert!(merged.get("domain").is_some());
        assert_eq!(
            merged.get("domain").unwrap(),
            &Some(String::from("https://benjaminsattler.net"))
        );
    }

    #[test]
    fn merge_keeps_settings_without_defaults_if_not_cleaning() {
        let settings =
            merge::Settings::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

        let defaults = merge::Settings::from([("port".into(), Some("433".into()))]);

        let merged = merge::merge(settings, defaults, Some(false));

        assert!(merged.get("domain").is_some());
        assert_eq!(
            merged.get("domain").unwrap(),
            &Some(String::from("https://benjaminsattler.net"))
        );
    }

    #[test]
    fn merge_discards_settings_without_defaults_if_cleaning() {
        let settings =
            merge::Settings::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

        let defaults = merge::Settings::from([("port".into(), Some("433".into()))]);

        let merged = merge::merge(settings, defaults, Some(true));

        assert!(merged.get("domain").is_none());
    }
}

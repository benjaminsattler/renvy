use crate::merge::Settings;

/// Produces a String representation of a [`Settings`] object which can be written to disk.
///
/// This function produces a string that consists of newline-separated lines. Each line
/// corresponds to one Key-Value pair, where the key is separated from the value with a `=`.
/// This function is the opposite of [`deserialize()`].
///
/// ## Example
///
/// Each key-value pair is being serialized into Strings separated by `=`. All pairs
/// are concatenated by newlines:
///
/// ```
/// let settings = renvy::Settings::from([("url".into(), Some(String::from("https://example.com")))]);
/// let serialized = renvy::serialize(&settings);
/// assert_eq!(serialized, String::from("url=https://example.com\n"));
/// ```
///
/// Settings without values will be serialized correctly, too:
///
/// ```
/// let settings = renvy::Settings::from([("url".into(), None)]);
/// let serialized = renvy::serialize(&settings);
/// assert_eq!(serialized, String::from("url=\n"));
/// ```
pub fn serialize(settings: &Settings) -> String {
    settings
        .iter()
        .map(|(key, value)| format!("{}={}\n", key, if let Some(x) = value { x } else { "" }))
        .collect()
}

/// Parses a String into an object of type [`Settings`].
///
/// This function parses a string that consists of newline-separated lines. Each line
/// must be one Key-Value pair, where the key is separated from the value with a `=`.
/// This function is the opposite of [`serialize()`].
///
/// ## Example
///
/// Each key-value pair is being parsed into one entry. All pairs
/// are collected in one variable of type renvy::Settings.
///
/// ```
/// let input = String::from("url=https://example.com\n");
/// let deserialized = renvy::deserialize(&input);
/// assert_eq!(deserialized.get("url").unwrap(), &Some(String::from("https://example.com")));
/// ```
///
/// Lines without values will be deserialized correctly, too:
///
/// ```
/// let input = String::from("url=\n");
/// let deserialized = renvy::deserialize(&input);
/// assert_eq!(deserialized.get("url").unwrap(), &None);
/// ```
pub fn deserialize(input: &str) -> Settings {
    let mut result = Settings::new();

    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let mut splits = line.split('=').collect::<Vec<&str>>();
            let key: String = splits.remove(0).into();
            let value: Option<String> = if splits.is_empty() || splits.join("=") == "" {
                None
            } else {
                Some(splits.join("="))
            };
            result.insert(key, value);
        });

    result
}

#[cfg(test)]
mod test {

    use crate::{merge, serde};

    #[test]
    fn test_seralization_one_value() {
        let settings =
            merge::Settings::from([("domain".into(), Some("https://benjaminsattler.net".into()))]);

        let expected = String::from("domain=https://benjaminsattler.net\n");

        assert_eq!(serde::serialize(&settings), expected);
    }

    #[test]
    fn test_deseralization_one_value() {
        let input = String::from("domain=https://benjaminsattler.net\n");

        let actual = serde::deserialize(&input);

        assert!(actual.contains_key("domain"));
        assert_eq!(
            actual.get("domain").unwrap(),
            &Some("https://benjaminsattler.net".into())
        );
    }

    #[test]
    fn test_seralization_multiple_values() {
        let settings = merge::Settings::from([
            ("domain".into(), Some("https://benjaminsattler.net".into())),
            ("port".into(), Some("443".into())),
        ]);

        let expected = String::from("domain=https://benjaminsattler.net\nport=443\n");

        assert_eq!(serde::serialize(&settings), expected);
    }

    #[test]
    fn test_deseralization_multiple_values() {
        let input = String::from("port=443\ndomain=https://benjaminsattler.net\n");

        let actual = serde::deserialize(&input);

        assert!(actual.contains_key("domain"));
        assert_eq!(
            actual.get("domain").unwrap(),
            &Some("https://benjaminsattler.net".into())
        );
        assert!(actual.contains_key("port"));
        assert_eq!(actual.get("port").unwrap(), &Some("443".into()));
    }

    #[test]
    fn test_seralization_sorting() {
        let settings = merge::Settings::from([
            ("port".into(), Some("443".into())),
            ("domain".into(), Some("https://benjaminsattler.net".into())),
        ]);

        let expected = String::from("domain=https://benjaminsattler.net\nport=443\n");

        assert_eq!(serde::serialize(&settings), expected);
    }

    #[test]
    fn test_deseralization_sorting() {
        let input = String::from("port=443\ndomain=https://benjaminsattler.net\n");

        let actual = serde::deserialize(&input);

        let mut actual_iter = actual.keys().into_iter();
        assert_eq!(actual_iter.next(), Some(&"domain".into()));
        assert_eq!(actual_iter.next(), Some(&"port".into()));
    }

    #[test]
    fn test_seralization_equals_in_value() {
        let settings =
            merge::Settings::from([("domain".into(), Some("this=value_is_special".into()))]);

        let expected = String::from("domain=this=value_is_special\n");

        assert_eq!(serde::serialize(&settings), expected);
    }

    #[test]
    fn test_deseralization_equals_in_value() {
        let input = String::from("domain=this=value_is_special\n");

        let actual = serde::deserialize(&input);

        assert!(actual.contains_key("domain"));
        assert_eq!(
            actual.get("domain").unwrap(),
            &Some("this=value_is_special".into())
        );
    }

    #[test]
    fn test_seralization_spaces_in_value() {
        let settings =
            merge::Settings::from([("domain".into(), Some("this value has spaces".into()))]);

        let expected = String::from("domain=this value has spaces\n");

        assert_eq!(serde::serialize(&settings), expected);
    }

    #[test]
    fn test_deseralization_spaces_in_value() {
        let input = String::from("domain=this value has spaces\n");

        let actual = serde::deserialize(&input);

        assert!(actual.contains_key("domain"));
        assert_eq!(
            actual.get("domain").unwrap(),
            &Some("this value has spaces".into())
        );
    }

    #[test]
    fn test_seralization_empty_value() {
        let settings = merge::Settings::from([("domain".into(), None)]);

        let expected = String::from("domain=\n");

        assert_eq!(serde::serialize(&settings), expected);
    }

    #[test]
    fn test_deseralization_empty_value() {
        let input = String::from("domain=\n");

        let actual = serde::deserialize(&input);

        assert!(actual.contains_key("domain"));
        assert_eq!(actual.get("domain").unwrap(), &None);
    }
}

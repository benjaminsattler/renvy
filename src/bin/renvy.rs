use clap::Parser;

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

    let (settings, defaults) = (renvy::deserialize(settings), renvy::deserialize(defaults));

    let merged = renvy::merge(settings, defaults, Some(matches.cleanup));

    let merged = renvy::serialize(merged);

    renvy::write_file(&matches.settings, merged).unwrap()
}

#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;
    use uuid::Uuid;

    fn setup(settings_path: &str, settings_contents: &str, defaults_path: &str, defaults_contents: &str) {
        fs::write(settings_path, settings_contents).unwrap();
        fs::write(defaults_path, defaults_contents).unwrap();
    }

    fn teardown(settings_path: &str, defaults_path: &str) {
        fs::remove_file(settings_path).unwrap();
        fs::remove_file(defaults_path).unwrap();
    }

    #[test]
    fn test_merge_without_cleanup() {
        let defaults = "domain=https://example.com\n\
    port=443\n\
    extra=foo=bar\n\
    user-agent=Microsoft Internet Explorer\n\
    empty=\n";

        let settings = "domain=https://benjaminsattler.net\n\
    extra-setting=extra-value\n";

        let expected = "domain=https://benjaminsattler.net\n\
    empty=\n\
    extra=foo=bar\n\
    extra-setting=extra-value\n\
    port=443\n\
    user-agent=Microsoft Internet Explorer\n";

        let settings_path = format!("{}{}.env", std::env::temp_dir().to_string_lossy(), Uuid::new_v4());
        let defaults_path = format!("{}.dist", &settings_path);

        setup(&settings_path, settings, &defaults_path, defaults);

        let mut cmd = Command::cargo_bin("renvy").unwrap();
        let result = cmd.arg(&settings_path).arg(&defaults_path).assert();

        result.success();

        let actual = fs::read_to_string(&settings_path).unwrap();

        teardown(&settings_path, &defaults_path);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_merge_with_cleanup_shorthand() {
        let defaults = "domain=https://example.com\n\
    port=443\n\
    extra=foo=bar\n\
    user-agent=Microsoft Internet Explorer\n\
    empty=\n";

        let settings = "domain=https://benjaminsattler.net\n\
    extra-setting=extra-value\n";

        let expected = "domain=https://benjaminsattler.net\n\
    empty=\n\
    extra=foo=bar\n\
    port=443\n\
    user-agent=Microsoft Internet Explorer\n";

        let settings_path = format!("{}{}.env", std::env::temp_dir().to_string_lossy(), Uuid::new_v4());
        let defaults_path = format!("{}.dist", &settings_path);

        setup(&settings_path, settings, &defaults_path, defaults);

        let mut cmd = Command::cargo_bin("renvy").unwrap();
        let result = cmd
            .arg("-c")
            .arg(&settings_path)
            .arg(&defaults_path)
            .assert();

        result.success();

        let actual = fs::read_to_string(&settings_path).unwrap();

        teardown(&settings_path, &defaults_path);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_merge_with_cleanup_longhand() {
        let defaults = "domain=https://example.com\n\
    port=443\n\
    extra=foo=bar\n\
    user-agent=Microsoft Internet Explorer\n\
    empty=\n";

        let settings = "domain=https://benjaminsattler.net\n\
    extra-setting=extra-value\n";

        let expected = "domain=https://benjaminsattler.net\n\
    empty=\n\
    extra=foo=bar\n\
    port=443\n\
    user-agent=Microsoft Internet Explorer\n";

        let settings_path = format!("{}{}.env", std::env::temp_dir().to_string_lossy(), Uuid::new_v4());
        let defaults_path = format!("{}.dist", &settings_path);

        setup(&settings_path, settings, &defaults_path, defaults);

        let mut cmd = Command::cargo_bin("renvy").unwrap();
        let result = cmd
            .arg("--cleanup")
            .arg(&settings_path)
            .arg(&defaults_path)
            .assert();

        result.success();

        let actual = fs::read_to_string(&settings_path).unwrap();

        teardown(&settings_path, &defaults_path);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_merge_with_missing_parameters() {
        let defaults = "domain=https://example.com\n\
    port=443\n\
    extra=foo=bar\n\
    user-agent=Microsoft Internet Explorer\n\
    empty=\n";

        let settings = "domain=https://benjaminsattler.net\n\
    extra-setting=extra-value\n";

        let settings_path = format!("{}{}.env", std::env::temp_dir().to_string_lossy(), Uuid::new_v4());
        let defaults_path = format!("{}.dist", &settings_path);

        setup(&settings_path, settings, &defaults_path, defaults);

        let mut cmd = Command::cargo_bin("renvy").unwrap();
        let result = cmd.arg("--cleanup").arg(&settings_path).assert();

        result.failure();

        teardown(&settings_path, &defaults_path);
    }

    #[test]
    fn test_merge_with_no_parameters() {
        let mut cmd = Command::cargo_bin("renvy").unwrap();
        let result = cmd.assert();

        result.failure().stderr(
            predicate::str::contains("error")
                .and(predicate::str::contains("USAGE"))
                .and(predicate::str::contains("--help")),
        );
    }

    #[test]
    fn test_merge_help_longhand() {
        let mut cmd = Command::cargo_bin("renvy").unwrap();
        let result = cmd.arg("--help").assert();

        result.success().stdout(
            predicate::str::contains("USAGE")
                .and(predicate::str::contains("-c"))
                .and(predicate::str::contains("--cleanup"))
                .and(predicate::str::contains("-h"))
                .and(predicate::str::contains("--help"))
                .and(predicate::str::contains("-v"))
                .and(predicate::str::contains("--version"))
                .and(predicate::str::contains("SETTINGS"))
                .and(predicate::str::contains("DEFAULTS")),
        );
    }

    #[test]
    fn test_merge_help_shorthand() {
        let mut cmd = Command::cargo_bin("renvy").unwrap();
        let result = cmd.arg("-h").assert();

        result.success().stdout(
            predicate::str::contains("USAGE")
                .and(predicate::str::contains("-c"))
                .and(predicate::str::contains("--cleanup"))
                .and(predicate::str::contains("-h"))
                .and(predicate::str::contains("--help"))
                .and(predicate::str::contains("-v"))
                .and(predicate::str::contains("--version"))
                .and(predicate::str::contains("SETTINGS"))
                .and(predicate::str::contains("DEFAULTS")),
        );
    }

    #[test]
    fn test_merge_version_longhand() {
        let mut cmd = Command::cargo_bin("renvy").unwrap();
        let result = cmd.arg("--version").assert();

        result
            .success()
            .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
    }
}

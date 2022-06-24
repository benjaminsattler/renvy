use predicates::prelude::*;
use std::fs;
use uuid::Uuid;

fn setup(
    settings_path: &str,
    settings_contents: &str,
    defaults_path: &str,
    defaults_contents: &str,
) {
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

    let settings_path = format!(
        "{}/{}.env",
        std::env::temp_dir().to_string_lossy(),
        Uuid::new_v4()
    );
    let defaults_path = format!("{}.dist", &settings_path);

    setup(&settings_path, settings, &defaults_path, defaults);

    let bin_under_test = escargot::CargoBuild::new()
        .bin("renvy")
        .current_release()
        .current_target()
        .run()
        .unwrap();
    let mut cmd = bin_under_test.command();
    let result = cmd
        .arg(&settings_path)
        .arg(&defaults_path)
        .status()
        .unwrap();

    assert!(result.success());

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

    let settings_path = format!(
        "{}/{}.env",
        std::env::temp_dir().to_string_lossy(),
        Uuid::new_v4()
    );
    let defaults_path = format!("{}.dist", &settings_path);

    setup(&settings_path, settings, &defaults_path, defaults);

    let bin_under_test = escargot::CargoBuild::new()
        .bin("renvy")
        .current_release()
        .current_target()
        .run()
        .unwrap();
    let mut cmd = bin_under_test.command();
    let result = cmd
        .arg("-c")
        .arg(&settings_path)
        .arg(&defaults_path)
        .status()
        .unwrap();

    assert!(result.success());

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

    let settings_path = format!(
        "{}/{}.env",
        std::env::temp_dir().to_string_lossy(),
        Uuid::new_v4()
    );
    let defaults_path = format!("{}.dist", &settings_path);

    setup(&settings_path, settings, &defaults_path, defaults);

    let bin_under_test = escargot::CargoBuild::new()
        .bin("renvy")
        .current_release()
        .current_target()
        .run()
        .unwrap();
    let mut cmd = bin_under_test.command();
    let result = cmd
        .arg("--cleanup")
        .arg(&settings_path)
        .arg(&defaults_path)
        .status()
        .unwrap();

    assert!(result.success());

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

    let settings_path = format!(
        "{}/{}.env",
        std::env::temp_dir().to_string_lossy(),
        Uuid::new_v4()
    );
    let defaults_path = format!("{}.dist", &settings_path);

    setup(&settings_path, settings, &defaults_path, defaults);

    let bin_under_test = escargot::CargoBuild::new()
        .bin("renvy")
        .current_release()
        .current_target()
        .run()
        .unwrap();
    let mut cmd = bin_under_test.command();
    let result = cmd.arg("-c").arg(&settings_path).output().unwrap();

    assert!(!result.status.success());

    teardown(&settings_path, &defaults_path);
}

#[test]
fn test_merge_with_no_parameters() {
    let bin_under_test = escargot::CargoBuild::new()
        .bin("renvy")
        .current_release()
        .current_target()
        .run()
        .unwrap();
    let mut cmd = bin_under_test.command();
    let result = cmd.output().unwrap();

    assert!(!result.status.success());

    let stderr = std::str::from_utf8(&result.stderr).unwrap();
    assert!(predicate::str::contains("error")
        .and(predicate::str::contains("USAGE"))
        .and(predicate::str::contains("--help"))
        .eval(&stderr));
}

#[test]
fn test_merge_help_longhand() {
    let bin_under_test = escargot::CargoBuild::new()
        .bin("renvy")
        .current_release()
        .current_target()
        .run()
        .unwrap();
    let mut cmd = bin_under_test.command();
    let result = cmd.arg("--help").output().unwrap();

    assert!(result.status.success());

    let stdout = std::str::from_utf8(&result.stdout).unwrap();
    assert!(predicate::str::contains("USAGE")
        .and(predicate::str::contains("-c"))
        .and(predicate::str::contains("--cleanup"))
        .and(predicate::str::contains("-h"))
        .and(predicate::str::contains("--help"))
        .and(predicate::str::contains("-v"))
        .and(predicate::str::contains("--version"))
        .and(predicate::str::contains("SETTINGS"))
        .and(predicate::str::contains("DEFAULTS"))
        .eval(&stdout));
}

#[test]
fn test_merge_help_shorthand() {
    let bin_under_test = escargot::CargoBuild::new()
        .bin("renvy")
        .current_release()
        .current_target()
        .run()
        .unwrap();
    let mut cmd = bin_under_test.command();
    let result = cmd.arg("-h").output().unwrap();

    assert!(result.status.success());

    let stdout = std::str::from_utf8(&result.stdout).unwrap();
    assert!(predicate::str::contains("USAGE")
        .and(predicate::str::contains("-c"))
        .and(predicate::str::contains("--cleanup"))
        .and(predicate::str::contains("-h"))
        .and(predicate::str::contains("--help"))
        .and(predicate::str::contains("-v"))
        .and(predicate::str::contains("--version"))
        .and(predicate::str::contains("SETTINGS"))
        .and(predicate::str::contains("DEFAULTS"))
        .eval(&stdout));
}

#[test]
fn test_merge_version_longhand() {
    let bin_under_test = escargot::CargoBuild::new()
        .bin("renvy")
        .current_release()
        .current_target()
        .run()
        .unwrap();
    let mut cmd = bin_under_test.command();
    let result = cmd.arg("--version").output().unwrap();

    assert!(result.status.success());

    let stdout = std::str::from_utf8(&result.stdout).unwrap();
    assert!(predicate::str::contains(env!("CARGO_PKG_VERSION")).eval(&stdout));
}

#[test]
fn test_merge_version_shorthand() {
    let bin_under_test = escargot::CargoBuild::new()
        .bin("renvy")
        .current_release()
        .current_target()
        .run()
        .unwrap();
    let mut cmd = bin_under_test.command();
    let result = cmd.arg("-V").output().unwrap();

    assert!(result.status.success());

    let stdout = std::str::from_utf8(&result.stdout).unwrap();
    assert!(predicate::str::contains(env!("CARGO_PKG_VERSION")).eval(&stdout));
}

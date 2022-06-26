use std::{fs, io};

/// Reads a file from disk and returns its contents as a [`std::io::Result<String>`].
///
/// You would use this function to read both the settings and defaults file, before
/// passing their contents to [`crate::deserialize()`] and [`crate::merge()`].
///
/// ## Reading a file into a String
///
/// This shows how to obtain a [`std::io::Result`] that wraps a `String`.
/// ```no_run
/// if let Ok(file_contents) = renvy::read_file("/path/to/my/.env") {
///     println!("File contents: {}", &file_contents);
/// }
/// ```
///
/// You can reuse the same function for reading the settings file as well as the defaults file.
pub fn read_file(filepath: &str) -> io::Result<String> {
    fs::read_to_string(filepath)
}

/// Writes a String data `contents` into the file at `filepath` and returns
/// the result as [`std::io::Result`].
///
/// You would use this function to write the deserialized settings back to the file after merging.
///
/// ## Writing a String into a file
///
/// This shows how to write a `String` which returns a [`std::io::Result`].
/// ```no_run
/// # let deserialized_contents = "key=value";
/// if let Ok(_) = renvy::write_file(&"/path/to/my/.env", deserialized_contents) {
///     println!("File written successfully.");
/// }
/// ```
pub fn write_file(filepath: &str, contents: &str) -> io::Result<()> {
    fs::write(filepath, contents)
}

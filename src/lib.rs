mod io;
mod merge;
pub mod prelude;
mod serde;

pub use io::{read_file, write_file};
pub use merge::{merge, Key, Settings, Value};
pub use serde::{deserialize, serialize};

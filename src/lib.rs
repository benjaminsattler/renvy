mod io;
mod merge;
mod serde;
pub mod prelude;

pub use io::{read_file, write_file};
pub use merge::{Key, Value, Settings, merge};
pub use serde::{deserialize, serialize};

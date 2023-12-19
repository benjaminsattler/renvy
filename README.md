[![Crates.io](https://img.shields.io/crates/v/librenvy?label=librenvy%20%40%20crates.io)](https://crates.io/crates/librenvy)
[![Crates.io](https://img.shields.io/crates/v/renvy?label=renvy%20%40%20crates.io)](https://crates.io/crates/renvy)
[![GitHub](https://img.shields.io/github/license/benjaminsattler/renvy)](https://github.com/benjaminsattler/renvy/blob/master/LICENSE)
[![Push to master](https://github.com/benjaminsattler/renvy/actions/workflows/push-master.yml/badge.svg)](https://github.com/benjaminsattler/renvy)

# renvy

> renvy provides easy-to-use functionality for managing settings files that are based on templates

## Library

Install the library with
```sh
❯ cargo install librenvy
```

or by adding a corresponding entry to your `Cargo.toml`:
```toml
[dependencies]
librenvy = "~0"
```

or by downloading a precompiled library file from [the GitHub Releases page](https://github.com/benjaminsattler/renvy/releases).

After having installed `librenvy` you'll be able to use it's functionality like this
```rust
use renvy;

let (settings, defaults) = match (
renvy::read_file(&matches.settings),
renvy::read_file(&matches.defaults),
) {
(Ok(settings), Ok(defaults)) => (settings, defaults),
(Ok(_), Err(_)) => exit_with_error("Error reading defaults file", -1),
(Err(_), Ok(_)) => exit_with_error("Error reading settings file", -2),
(Err(_), Err(_)) => exit_with_error("Error reading input files", -3),
};

let (settings, defaults) = (renvy::deserialize(&settings), renvy::deserialize(&defaults));

let merged = renvy::merge(settings, defaults, Some(matches.cleanup));

let merged = renvy::serialize(&merged);

renvy::write_file(&matches.settings, &merged).unwrap()
```

### More Information about the Library

You can find more information about the library
- [in the separate Readme of the library crate](https://github.com/benjaminsattler/renvy/blob/master/library/README.md)
- at crates.io: https://crates.io/crates/librenvy
- reading code documentation and usage examples at https://docs.rs/librenvy/ 

You can also fetch the compiled library as a direct download the [GitHub repository releases page](https://github.com/benjaminsattler/renvy/releases).

## Binary Executable

Install the binary executable with

```sh
❯ cargo install renvy
```
which will put it in a place in your `$PATH` (default `~/.cargo/bin`).

Alternatively you can also download a precompiled version from [the GitHub Releases page](https://github.com/benjaminsattler/renvy/releases).

Verify if it launches correctly:
```sh
❯ renvy --help
renvy 0.1.5           
Benjamin Sattler <bsattler.inbox@gmail.com>
.env file manager that merges defaults with custom settings

USAGE:
    renvy [OPTIONS] <SETTINGS> <DEFAULTS>

ARGS:
    <SETTINGS>    
    <DEFAULTS>    

OPTIONS:
    -c, --cleanup    
    -h, --help       Print help information
    -V, --version    Print version information

```
Now you can invoke it like any other executable:
```sh
# Assuming that both files ".env" and ".env.dist" exist
# in the current directory
❯ renvy .env .env.dist
```

### More information about the binary executable
You can find more information about the library
- [in the separate Readme of the binary crate](https://github.com/benjaminsattler/renvy/blob/master/binary/README.md)
- at crates.io: https://crates.io/crates/renvy
- reading code documentation and usage examples at https://docs.rs/renvy/

## Docker Image

We also provide Docker Images, which allow you to run renvy without installation:
```sh
# Assuming that both files ".env" and ".env.dist" exist
# in the current directory
❯ docker run --rm -v "${PWD}":/opt docker.io/benjaminsattler/renvy .env .env.dist
```

### More information about Docker Images
You can find more information about the library
- at the Docker Hub: https://hub.docker.com/repository/docker/benjaminsattler/renvy/
- by reading the [Dockerfile in the repository](https://github.com/benjaminsattler/renvy/blob/master/docker/Dockerfile)

## License

MIT License

Copyright (c) [year] [fullname]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

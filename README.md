# cargo ssearch -- cargo search on steroids

With cargo search you often can miss very used packages, for example `cargo search serialize` does not show `serde` in the default output and it only shows `serde_json` if you ask for the first 20 results.

This search tool gets the results sorted by downloads (or recent downloads) and optionally displayes additional useful resoruces linke homepage and documentation for the user to go and have a look. The output uses colours to distinguish the reults (hopefully) more clearly.

The usage is as simple as typing `cargo ssearch --help`
```
cargo-ssearch-ssearch 0.1.1
Marcello Seri <marcello.seri@gmail.com>
cargo search on steroids
USAGE:
    cargo ssearch [FLAGS] [OPTIONS] <query>
FLAGS:
    -h, --help       Prints help information
    -q, --quiet      quiet output, display only crate, version and downloads
    -r, --recent     sort by recent downloads instead of overall downloads
    -V, --version    Prints version information
OPTIONS:
    -l, --limit <limit>    how many packages to display [default: 10]
        --page <page>      the crates.io search result page to display [default: 1]
ARGS:
    <query>    query string for crates.io
```

The software was a very fastly made hack, but I am using it and I am happy on how it works.
The information are obtained directly from crates.io.

## Build
It is enough to clone the repo and compile it with cargo:
```
$ git clone https://github.com/mseri/scrutch
$ cd scrutch
$ cargo build --release
```

The tool can then be run via cargo itself or installed in a local path and run as standalone app.

## Examples

```
$ cargo ssearch serialize --limit 20 -q
Displaying 20 crates from page 1 out of the 466 found.
serde                   = "1.0.37"      (downloads: 5160233)
rustc-serialize         = "0.3.24"      (downloads: 4389049)
serde_json              = "1.0.13"      (downloads: 2730954)
toml                    = "0.4.6"       (downloads: 2190141)
quote                   = "0.5.1"       (downloads: 2127675)
serde_derive            = "1.0.37"      (downloads: 1870124)
serde_derive_internals  = "0.23.0"      (downloads: 1376928)
xml-rs                  = "0.7.0"       (downloads: 880986)
serde_codegen_internals = "0.14.2"      (downloads: 741492)
serde_codegen           = "0.9.0"       (downloads: 639554)
handlebars              = "1.0.0-beta.1"        (downloads: 440993)
cargo_metadata          = "0.5.4"       (downloads: 378409)
bincode                 = "1.0.0"       (downloads: 331593)
serde_urlencoded        = "0.5.1"       (downloads: 260365)
serde_macros            = "0.8.9"       (downloads: 209024)
serde_cbor              = "0.8.2"       (downloads: 179360)
serde_yaml              = "0.7.3"       (downloads: 162988)
postgres                = "0.15.2"      (downloads: 99893)
rmp                     = "0.8.7"       (downloads: 99743)
sodiumoxide             = "0.0.16"      (downloads: 87682)
```

```
$ cargo ssearch serialize -q --limit 20 -r
Displaying 20 crates from page 1 out of the 466 found.
serde                   = "1.0.37"      (downloads: 5160233)
quote                   = "0.5.1"       (downloads: 2127675)
serde_json              = "1.0.13"      (downloads: 2730954)
serde_derive            = "1.0.37"      (downloads: 1870124)
serde_derive_internals  = "0.23.0"      (downloads: 1376928)
toml                    = "0.4.6"       (downloads: 2190141)
rustc-serialize         = "0.3.24"      (downloads: 4389049)
cargo_metadata          = "0.5.4"       (downloads: 378409)
xml-rs                  = "0.7.0"       (downloads: 880986)
handlebars              = "1.0.0-beta.1"        (downloads: 440993)
serde_urlencoded        = "0.5.1"       (downloads: 260365)
bincode                 = "1.0.0"       (downloads: 331593)
rustc-ap-serialize      = "92.0.0"      (downloads: 69093)
serde_codegen_internals = "0.14.2"      (downloads: 741492)
serde_yaml              = "0.7.3"       (downloads: 162988)
encoding_rs             = "0.7.2"       (downloads: 85705)
serde_cbor              = "0.8.2"       (downloads: 179360)
serde_codegen           = "0.9.0"       (downloads: 639554)
serde_test              = "1.0.37"      (downloads: 82573)
rmp                     = "0.8.7"       (downloads: 99743)
```

```
$ cargo ssearch serialize
Displaying 10 crates from page 1 out of the 466 found.

serde                   = "1.0.37"      (downloads: 5160233)
 -> A generic serialization/deserialization framework
    docs: https://docs.serde.rs/serde/
    home: https://serde.rs

rustc-serialize         = "0.3.24"      (downloads: 4389049)
 -> Generic serialization/deserialization support corresponding to the
`derive(RustcEncodable, RustcDecodable)` mode in the compiler. Also includes
support for hex, base64, and json encoding and decoding.
    docs: https://doc.rust-lang.org/rustc-serialize
    home: https://github.com/rust-lang/rustc-serialize

serde_json              = "1.0.13"      (downloads: 2730954)
 -> A JSON serialization file format
    docs: http://docs.serde.rs/serde_json/

toml                    = "0.4.6"       (downloads: 2190141)
 -> A native Rust encoder and decoder of TOML-formatted files and streams. Provides
implementations of the standard Serialize/Deserialize traits for TOML data to
facilitate deserializing and serializing Rust structures.
    docs: https://docs.rs/toml
    home: https://github.com/alexcrichton/toml-rs

quote                   = "0.5.1"       (downloads: 2127675)
 -> Quasi-quoting macro quote!(...)
    docs: https://docs.rs/quote/

serde_derive            = "1.0.37"      (downloads: 1870124)
 -> Macros 1.1 implementation of #[derive(Serialize, Deserialize)]
    docs: https://serde.rs/codegen.html
    home: https://serde.rs

serde_derive_internals  = "0.23.0"      (downloads: 1376928)
 -> AST representation used by Serde derive macros. Unstable.
    docs: https://docs.serde.rs/serde_derive_internals/
    home: https://serde.rs

xml-rs                  = "0.7.0"       (downloads: 880986)
 -> An XML library in pure Rust
    docs: http://netvl.github.io/xml-rs/

serde_codegen_internals = "0.14.2"      (downloads: 741492)
 -> AST representation used by Serde codegen. Unstable.
    docs: https://docs.serde.rs/serde_codegen_internals/
    home: https://serde.rs

serde_codegen           = "0.9.0"       (downloads: 639554)
 -> Macros to auto-generate implementations for the serde framework
    docs: https://serde.rs/codegen.html
    home: https://serde.rs


```

# TODO

The tool has never been updated as it still works properly.
It would be nice to find the time to update the code to more modern and idiomatic rust.


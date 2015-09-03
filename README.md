# scrutch
Tiny utility to search rust crates directly from the command line

As simple as typing `scrutch -h`
```
Scrutch - Crates Search

Usage:
  scrutch [--info] <query>
  scrutch (-h | --help)
  scrutch --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --info        Show complete details of the crates.
```

The software was a very fastly made hack, but I am using it and I plan to implement a decent interface using [rustbox](https://github.com/gchp/rustbox).
The information are obtained directly from crates.io. To compile `libcurl` is required.

## Examples

```
$ scrutch serialize
scrutch: 10 crates found with query: "serialize"

rustc-serialize      = "0.3.16" (downloads: 292822)
serde                = "0.6.0"  (downloads: 22667)
serde_json           = "0.6.0"  (downloads: 5302)
rmp                  = "0.7.0"  (downloads: 461)
serial               = "0.2.0"  (downloads: 385)
dbus-serialize       = "0.1.1"  (downloads: 210)
serial-win           = "0.1.1"  (downloads: 148)
rmp-serde            = "0.7.0"  (downloads: 27)
rmp-serialize        = "0.7.0"  (downloads: 19)
scrutch              = "0.0.3"  (downloads: 1)
```

```
$ scrutch serialize --info
scrutch: 10 crates found with query: "serialize"

rustc-serialize = "0.3.16"  (downloads: 292822)
 -> Generic serialization/deserialization support corresponding to the
`derive(RustcEncodable, RustcDecodable)` mode in the compiler. Also includes
support for hex, base64, and json encoding and decoding.

    docs: http://doc.rust-lang.org/rustc-serialize
    home: https://github.com/rust-lang/rustc-serialize

serde = "0.6.0" (downloads: 22667)
 -> A generic serialization/deserialization framework
    docs: https://serde-rs.github.io/serde/serde/serde/index.html

serde_json = "0.6.0"  (downloads: 5302)
 -> A JSON serialization file format
    docs: https://serde-rs.github.io/json/serde_json/

rmp = "0.7.0" (downloads: 461)
 -> Pure Rust MessagePack serialization implementation
    docs: https://3hren.github.io/msgpack-rust/rmp/index.html

serial = "0.2.0"  (downloads: 385)
 -> Rust library for accessing serial ports.
    docs: https://dcuddeback.github.io/serial-rs/serial/
    home: https://github.com/dcuddeback/serial-rs

dbus-serialize = "0.1.1"  (downloads: 210)
 -> Encoder / Decoder for D-Bus Types
    docs: http://srwalter.github.io/dbus-serialize/doc/dbus_serialize/types/index.html

serial-win = "0.1.1"  (downloads: 148)
 -> Serial communications in Windows
    docs: http://bryal.github.io/serial-win-rs/serial_win/

rmp-serde = "0.7.0" (downloads: 27)
 -> Serde bindings for RMP
    docs: https://3hren.github.io/msgpack-rust/rmp/index.html

rmp-serialize = "0.7.0" (downloads: 19)
 -> Rust Serialize bindings for RMP
    docs: https://3hren.github.io/msgpack-rust/rmp/index.html

scrutch = "0.0.3" (downloads: 1)
 -> Tiny utility to search rust crates directly from the command line
    docs: https://github.com/mseri/scrutch
    home: https://github.com/mseri/scrutch


```
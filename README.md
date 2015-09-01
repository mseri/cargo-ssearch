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
scrutch: 10 crates found with query: serialize

rustc-serialize (dwl: 289426)
serde (dwl: 22043)
serde_json (dwl: 4994)
rmp (dwl: 446)
serial (dwl: 382)
dbus-serialize (dwl: 208)
bytekey (dwl: 158)
serial-win (dwl: 147)
rmp-serialize (dwl: 18)
rmp-serde (dwl: 17)
```

```
$ scrutch serialize --info
scrutch: 10 crates found with query: serialize

rustc-serialize (dwl: 289426)
-> Generic serialization/deserialization support corresponding to the
`derive(RustcEncodable, RustcDecodable)` mode in the compiler. Also includes
support for hex, base64, and json encoding and decoding.

   docs: http://doc.rust-lang.org/rustc-serialize
   home: https://github.com/rust-lang/rustc-serialize

serde (dwl: 22043)
-> A generic serialization/deserialization framework
   docs: https://serde-rs.github.io/serde/serde/serde/index.html

serde_json (dwl: 4994)
-> A JSON serialization file format
   docs: https://serde-rs.github.io/json/serde_json/

rmp (dwl: 446)
-> Pure Rust MessagePack serialization implementation
   docs: https://3hren.github.io/msgpack-rust/rmp/index.html

serial (dwl: 382)
-> Rust library for accessing serial ports.
   docs: https://dcuddeback.github.io/serial-rs/serial/
   home: https://github.com/dcuddeback/serial-rs

dbus-serialize (dwl: 208)
-> Encoder / Decoder for D-Bus Types
   docs: http://srwalter.github.io/dbus-serialize/doc/dbus_serialize/types/index.html

bytekey (dwl: 158)
-> lexicographic sort-order preserving binary encoding

serial-win (dwl: 147)
-> Serial communications in Windows
   docs: http://bryal.github.io/serial-win-rs/serial_win/

rmp-serialize (dwl: 18)
-> Rust Serialize bindings for RMP
   docs: https://3hren.github.io/msgpack-rust/rmp/index.html

rmp-serde (dwl: 17)
-> Serde bindings for RMP
   docs: https://3hren.github.io/msgpack-rust/rmp/index.html
```
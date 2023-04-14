# filegen
A tool for generating number of files of the same size with random contents.

## Building
### Statically-linked binary
```
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu
```
(https://msfjarvis.dev/posts/building-static-rust-binaries-for-linux/)

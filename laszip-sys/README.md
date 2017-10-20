# laszip-sys

Low-level bindings to [laszip](https://www.laszip.org/).

## Building

Ensure that clang can see the `laszip/laszip_api.h` include file and the `laszip_api` library.

```
git clone https://github.com/gadomski/laszip-rs
cd laszip-rs/laszip-sys
cargo build
```

This crate's test suite includes "EXAMPLE_ONE" from laszip's own dll test, so to run layout tests and "EXAMPLE_ONE":

```bash
cargo test
```

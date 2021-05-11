# laszip-sys

Low-level bindings to the C API of [laszip](https://www.laszip.org/) for Rust.
A native Rust implementation of laz is available at https://github.com/tmontaigu/laz-rs.

[![Build Status](https://travis-ci.org/gadomski/laszip-sys.svg?branch=master)](https://travis-ci.org/gadomski/laszip-sys)

## laszip

LASzip is a free and lossless LiDAR compression format, based on the [las](https://www.asprs.org/committee-general/laser-las-file-format-exchange-activities.html) format.
Compression ratios over las of 4:1 and up to 10:1 have been observed in the wild.

## Building

Ensure that clang can see the `laszip/laszip_api.h` include file and the `laszip_api` library.

```
git clone https://github.com/gadomski/laszip-sys
cd laszip-rs/laszip-sys
cargo build
```

This crate's test suite includes "EXAMPLE_ONE" from laszip's own dll test, so to run layout tests and "EXAMPLE_ONE":

```bash
cargo test
```

## See also

- [PDAL](https://www.pdal.io/)
- [liblas](https://www.liblas.org/)
- [LAStools](https://rapidlasso.com/lastools/)
- [las-rs](https://github.com/gadomski/las-rs)
- [laz-perf](https://github.com/hobu/laz-perf)

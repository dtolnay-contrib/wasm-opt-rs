# Rust bindings for Binaryen's `wasm-opt`

[<img alt="github" src="https://img.shields.io/badge/github-brson/wasm--opt--rs-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/brson/wasm-opt-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/wasm-opt.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/wasm-opt)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-wasm--opt-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/wasm-opt)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/brson/wasm-opt-rs/CI/master?style=for-the-badge" height="20">](https://github.com/brson/wasm-opt-rs/actions?query=branch%3Amaster)

`wasm-opt` is a component of the [Binaryen] toolkit
that optimizes [WebAssembly] modules. It is written
in C++.

[Binaryen]: https://github.com/WebAssembly/binaryen
[WebAssembly]: https://webassembly.org/

This project provides a Rust crate that builds `wasm-opt` and:

1) makes its command-line interface installable via `cargo install`,
2) provides an API to access it programmatically.




## Installing the binary

```
cargo install wasm-opt --locked
```

It should behave exactly the same as `wasm-opt` installed from other sources.




## Using the library

See the [API documentation][api].

[api]: https://docs.rs/wasm-opt




## Building from source

```
git clone https://github.com/brson/wasm-opt-rs
cd wasm-opt-rs
git submodule update --init --recursive
cargo build && cargo test
```




## Toolchain requirements

Requires Rust 1.48+ and a C++ compiler with C++17 support.
It does not require CMake or other C++ build tools.

These are the earliest C++ compiler versions known to work:

- gcc 7
- clang 3.9
- Visual Studio 2019




## Limitations

- The `wasm-opt-sys` crate takes a non-negligible amount of time to build. It
  also does not do any incremental recompilation, so if the build is invalidated
  it will rebuild the C++ code from scratch. The lack of incremental
  recompilation is a limitation self-imposed by not using cmake or other
  external build system.
- `wasm-opt` on Windows does not support extended unicode paths (probably
  anything non-ASCII). This is a [limitation of
  binaryen](https://github.com/brson/wasm-opt-rs/issues/40) and not a regression
  of the bindings. It may or may not be fixed in the future. The APIs will
  return an error if this occurs.
- `cargo tarpaulin` (code coverage) [segfaults running any `wasm-opt`
  crates](https://github.com/brson/wasm-opt-rs/issues/59), reason unknown. This
  behavior could infect other crates that link to `wasm-opt`. If you use
  tarpaulin, you might verify it continues to work.




## Versioning

Binaryen uses a single monotonically-increasing number for versions.
This crate uses the semver minor version to track the Binaryen version,
so e.g. the `wasm-opt` crate version `0.110.0` corresponds to Binaryen 110.
Point releases are used for bugfixes.

Since minor version bumps of crates earlier than `1.0.0` are considered breaking,
users need to explicitly upgrade versions of `wasm-opt` to get new Binaryen releases,
and upgrades may have breaking API changes,
though we don't anticipate significant changes.




## Thanks

This project was created thanks to a [grant from the Web3 Foundation](https://github.com/w3f/Grants-Program/pull/1070).




## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

Binaryen itself, code from which is compiled and linked by this project,
is licensed under the terms of the Apache License, Version 2.0.

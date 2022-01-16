# web-library-rust

A basic CRUD application that represents books in a library.

## Build Instructions
Ensure that you have the Rust toolchain installed. The easiest way to accomplish this is with `rustup`.

Follow the directions at https://www.rust-lang.org/tools/install

- Run `cargo run` to bild and run an unoptimized development build.
- Run `cargo build` to build an unoptimized development build.
- Run `cargo build --release` to build an optimized version of the app.
- Run `cargo docs` to build the documentation for the app.
  - Use the `--open` parameter to open the docs in your browser.

If you wish to build the app in a Docker-compatible container, the usual build commands will suffice.
- `docker build <image>:<tag> .`

# QuBert
## *Query BERT with a style-ish Rust CLI*

This barely qualifies as an app! I wanted to write it in GTK at first but there was namespace collision with the BERT and GTK libraries at linking-time. So here we are -- next best thing.

## Running
Currently, the best way to ensure a clean build is using the Nix flake for a development environment and building with cargo.

On other platforms, you'll only need `libtorch` and `cargo` to install this app. 
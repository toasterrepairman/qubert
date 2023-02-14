# QuBert
## *Query BERT with a style-ish Rust CLI*

This barely qualifies as an app! I wanted to write it in GTK at first but there was namespace collision with the BERT and GTK libraries at linking-time. So here we are -- next best thing.

## Running
Currently, the best way to ensure a clean build is using the Nix flake for a development environment and building with cargo.

On other platforms, you'll only need `libtorch` and `cargo` to install this app. 

Run `cargo run` to build the application.

### Example

```
cargo run -- --prompt "Hello world!"
```

Output:

```
Hello world!

I’ve been working on a project for over a year now and I’m really excited to share it with you.

I’ve been working on this project for over a year now, and I’m really happy to share it with you because it’s the first time I’ve worked on a project that I’ll be working on in the future.
```
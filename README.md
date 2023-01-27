# Webp to PNG

## Disclaimer

This is a quick hack I've created for a personal use case. It probably sucks in a lot of ways, but works for me. Handle with care :)

## Setup

Run `cargo build`.

## Usage

Place desired images in `./input/` directory and run

```
cargo run
```

to convert them to `.png` files with a max-{width,height} of 512px in the `./output/` directory, removing the original source file.

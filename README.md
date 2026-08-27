# Media Module

**Convert HEIC to JPEG**
 
This is part of an ongoing media module toolchain project. Contact jondiscipulo@proton.me for any related concerns.


## convert-heic-jpeg 0.1.0

A small, fast **Rust** command-line tool that converts **HEIC / HEIF** images (as produced by
iPhones and modern cameras) to **JPG**.

It is **fully self-contained** — the HEIC decoder is a pure-Rust implementation
([`heif-oxide`](https://crates.io/crates/heif-oxide)), so there are **no C/C++ system
dependencies** (no `libheif`, no `vcpkg`) to install, and it builds and runs the same on
Windows, macOS, and Linux.

## Features

- Convert `.heic` and `.heif` files to `.jpg`.
- Batch conversion: pass multiple files **and/or directories** (directories are scanned
  recursively).
- Adjustable JPEG quality (`--quality` or `-q`, 1–100; default **100**).
- Write next to each input, or to a single output directory (`--output` or `-o`).
- Safe overwrite policy: existing files are skipped unless `--force` or `-f` is passed; errors on one
  file don't abort the rest.
- No external runtime dependencies.

## Installation

### From source (requires a Rust toolchain)

```sh
cargo install --path .
```

or build and run locally:

```sh
cargo build --release
./target/release/convert-heic-jpeg photo.heic
```

### Binary location

The produced binary is named `convert-heic-jpeg` (or `convert-heic-jpeg.exe` on Windows).

## Usage

```
convert-heic-jpeg [OPTIONS] <INPUTS>...
```

### Arguments

| Argument     | Description                                                        |
| ------------ | ------------------------------------------------------------------ |
| `<INPUTS>...` | One or more `.heic`/`.heif` files or directories. **Required.**   |

### Options

| Flag                   | Description                                              | Default |
| ---------------------- | -------------------------------------------------------- | ------- |
| `-o, --output-dir <DIR>` | Directory for output `.jpg` files (else beside input). | —       |
| `-q, --quality <1-100>`  | JPEG quality.                                          | `100`   |
| `-f, --force`            | Overwrite existing output files.                      | `false` |
| `-h, --help`             | Print help.                                           |         |

## Examples

Convert a single image (writes `photo.jpg` next to `photo.heic`):

```sh
convert-heic-jpeg photo.heic
```

Convert several files at quality 85:

```sh
convert-heic-jpeg -q 85 a.heic b.heic c.heic
```

Convert every HEIC in a folder tree into an `out/` directory, overwriting:

```sh
convert-heic-jpeg -o out -f -q 90 "C:\photos"
```

## How it works

1. `convert_to_jpeg()` reads and decodes the HEIC/HEIF container + HEVC payload in pure Rust.
2. The decoded pixels (`RGBA8`, display-ready sRGB with orientation applied) are flattened to
   `RGB` — the alpha channel is dropped because JPEG has no alpha.
3. The RGB bytes are encoded as a baseline JPEG using the `image` crate's JPEG encoder.

## Limitations

- **Output is JPEG only**, so alpha/transparency is always discarded.
- **Monochrome (4:0:0) alpha auxiliary images are not decoded** by the underlying
  `heif-oxide` decoder (its HEVC decoder is 4:2:0-only). Most iPhone photos use the
  4:2:0 alpha variant and decode fine; files relying on 4:0:0 alpha will come out without
  transparency even when targeting an alpha-capable format.
- 10-/12-bit HEIC sources are converted to 8-bit JPEG.

## License

Licensed under either of

- Apache License, Version 2.0
- MIT license

at your option.

The HEIC decoding is provided by [`heif-oxide`](https://crates.io/crates/heif-oxide)
(MIT OR Apache-2.0).

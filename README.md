# NFT image generator

This is the project to help artists to generate NFT collection, which can be generated from layers of PNG images.

## Usage

You need to make sure that `collection_schema.json` exists and describes collection you want to generate. Also you need to create folder `layers` and put their folders according to the order you specified in `collection_schema.json`.

## Dev notes

You can run project with:

```bash
cargo run
```

You can build it with

```bash
cargo build --release
```

You can cross-compile it for windows with:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```
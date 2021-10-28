# NFT image generator

This is the project to help artists to generate NFT collection, which can be generated from layers of PNG images.

## Usage

0. Get archive for your system in the [releases section](https://github.com/partyparrotclub/nft-image-generator/releases).
1. Unpack it to the directory you want to work in.
2. You need to make sure that `collection_schema.json` exists and describes collection you want to generate. You can start with copying `sample_collection_schema.json`. 
3. Also you need to create folder `layers` and put their folders according to the order you specified in `collection_schema.json`. There are some for generating early versions of Party Parrots.
4. Run `image_generator` binary file. You should run it from Terminal / Console app as `./image_generator`. On Mac terminal can be launched following [this](https://ladedu.com/how-to-open-a-terminal-window-at-any-folder-from-finder-in-macos/) guide.


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

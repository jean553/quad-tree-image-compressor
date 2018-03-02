# quad-tree-image-compressor

Image compressor (*for BMP format images with specific dimensions ONLY !!!*) that uses Quad Tree to recognize similar data and compact it.

Notes:
 * the tool only works with BMP files,
 * the image width and height must be identical,
 * the image dimensions (width and height) must be divisable by 4

## Compilation

```sh
cargo build --release
```

## Usage

```sh
./target/release/quad-tree-image-compressor image.bmp
```

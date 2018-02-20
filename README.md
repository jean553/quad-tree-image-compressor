# quad-tree-image-compressor

Image compressor (*for BMP format images with specific dimensions ONLY !!!*) that uses Quad Tree to recognize similar data and compact it.

Notes:
 * the tool only works with BMP files,
 * the image dimensions (width and height) must be divisable by 4
(if these two conditions are not respected, the behaviour is undefined)

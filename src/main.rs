extern crate piston_window;
extern crate graphics;

mod pixel;

use piston_window::{
    PistonWindow,
    WindowSettings,
    G2d,
    clear,
};

use std::fs::File;
use std::io::Read;
use std::env;

use pixel::Pixel;

/* the C library QuadTreeNode structure is:

   struct QuadTreeNode {
       QuadTreeNode* children[4];
       unsigned int data;
   };
*/

#[repr(C)]
struct QuadTreeNode {
    children: [*mut QuadTreeNode; 4],
    data: u32,
}

#[link(name="quad_tree", kind="static")]
extern {

    fn create() -> QuadTreeNode;

    fn allocateChildren(node: *mut QuadTreeNode);
}

/// Clear the whole window.
fn clear_screen(graphics: &mut G2d) {

    const BLACK_COLOR: f32 = 0.0;
    const COLOR_COMPOSITE_AMOUNT: usize = 4;
    clear(
        [
            BLACK_COLOR;
            COLOR_COMPOSITE_AMOUNT
        ],
        graphics,
    );
}

/// Indicates if the square in the picture (based on start and end) has same color squares.
///
/// # Args:
///
/// `pixels` - the array of pixels to browse
/// `start` - the index of the starting pixel of the square
/// `end` - the index of the ending pixel of the square
/// `dimensions` - the dimensions of the square to browse
///
/// # Returns:
///
/// true if pixels with different colors are within the square
fn square_has_different_pixels(
    pixels: &Vec<Pixel>,
    start: usize,
    end: usize,
    dimensions: usize,
) -> bool {

    let mut horizontal_limit = start + dimensions;

    for index in start..(end + 1) {

        /* prevent browsing the horizontal neighboor
           of the current square */
        if index >= horizontal_limit {

            if index == horizontal_limit + dimensions - 1 {
                horizontal_limit += 2 * dimensions;
            }

            continue;
        }

        if pixels[index] != pixels[end] {
            return true;
        }
    }

    return false;
}

/// Browse the pixels of the current square (starting from `square_start` and ending with `square_end` indices, compress the content into the quad tree node data field if all the pixels of the square are identical; divide the current square into four sub-squares if the pixels are different and call the function recursively for every new created sub-square
///
/// # Args:
///
/// `pixels` - the array of pixels to use
/// `node` - the current node to modify according to the content of the current square
/// `square_dimensions` - the width and height of the current square
/// `square_start` - the first index of the current square
/// `square_end` - the last index of the current square
fn create_node(
    pixels: &Vec<Pixel>,
    node: &mut QuadTreeNode,
    square_dimensions: u32,
    square_start: usize,
    square_end: usize,
) {

    let different_pixels = square_has_different_pixels(
        &pixels,
        square_start,
        square_end,
        square_dimensions as usize,
    );

    if different_pixels && square_dimensions != 1 {

        unsafe {
            allocateChildren(node)
        };

        let sub_square_dimensions = square_dimensions / 2;

        /* Quad tree node children are C-type raw pointers,
           dereferencing them is an unsafe action */

        let bottom_left_square = unsafe {
            &mut (*node.children[0])
        };

        let bottom_left_square_end = (
            square_end -
            (square_dimensions * sub_square_dimensions) as usize -
            sub_square_dimensions as usize
        ) as usize;

        create_node(
            &pixels,
            bottom_left_square,
            sub_square_dimensions,
            square_start,
            bottom_left_square_end,
        );

    } else {

        let pixel: &Pixel = &pixels[square_start];

        /* the data field of the quad tree node is 4 bytes long;
           in our case, the first byte is useless and set to 0,
           the second, third and fourth one store the red color,
           green color and blue color respectively */
        const BITS_PER_COLOR: u8 = 8;
        node.data = pixel.get_red() as u32;
        node.data <<= BITS_PER_COLOR;
        node.data += pixel.get_green() as u32;
        node.data <<= BITS_PER_COLOR;
        node.data += pixel.get_blue() as u32;
    }
}

fn main() {

    let file_name = env::args().nth(1).expect("No input file.");
    let mut file = File::open(file_name).expect("Cannot open file.");
    let mut buffer: Vec<u8> = Vec::new();

    let _ = file.read_to_end(&mut buffer);

    let width = buffer[0x12] as u32;
    let height = buffer[0x16] as u32;

    if width != height {
        panic!("The image width and height must be identical.");
    }

    let dimensions = width;

    if dimensions % 4 != 0 {
        panic!("The image width and height must be divisable by 4.");
    }

    let mut pixels: Vec<Pixel> = Vec::new();

    const BYTES_PER_PIXEL: u32 = 3;
    let last_pixel_index = (dimensions.pow(2) * BYTES_PER_PIXEL - 1) as usize;

    let mut red: u8 = 0;
    let mut green: u8 = 0;
    let mut blue: u8 = 0;

    let mut horizontal_position: u32 = 0;
    let mut vertical_position: u32 = height - 1;

    const OFFSET_BMP_RGB24: usize = 0x36;
    for (index, byte) in buffer.iter().skip(OFFSET_BMP_RGB24).enumerate() {

        let color_index = index as u32 % BYTES_PER_PIXEL;

        if index != 0 && (color_index == 0 || index == last_pixel_index) {

            pixels.push(
                Pixel::new(
                    red,
                    green,
                    blue,
                    horizontal_position,
                    vertical_position,
                )
            );

            horizontal_position += 1;

            if horizontal_position == dimensions {
                vertical_position -= 1;
                horizontal_position = 0;
            }
        }

        if color_index == 0 {
            blue = *byte;
        }
        else if color_index == 1 {
            green = *byte;
        }
        else if color_index == 2 {
            red = *byte;
        }
    }

    let mut window: PistonWindow = WindowSettings::new(
        "Quad Tree Image Compressor",
        [
            dimensions,
            dimensions,
        ]
    )
    .fullscreen(false)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut node = unsafe {
        create()
    };

    /* the first square is the whole picture,
       starting from the index 0 (bottom left)
       to the last index (top right) */
    create_node(
        &pixels,
        &mut node,
        dimensions,
        0,
        (width * height - 1) as usize,
    );

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, graphics| {

                for pixel in pixels.iter() {

                    pixel.display(
                        context,
                        graphics,
                    );
                }

                clear_screen(graphics);
            }
        );
    }
}

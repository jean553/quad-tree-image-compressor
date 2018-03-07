extern crate piston_window;
extern crate graphics;

mod pixel;
mod square;

use piston_window::{
    PistonWindow,
    WindowSettings,
    G2d,
    G2dTexture,
    Texture,
    TextureSettings,
    Flip,
    clear,
    image,
};

use std::fs::File;
use std::io::Read;
use std::env;

use pixel::Pixel;
use square::Square;

const HAS_CHILDREN_NODE: u32 = 0xFF000000;

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
/// `main_dimensions` - the dimensions of the main square
/// `sub_dimensions` - the dimensions of the current browsed sub-square
/// `sub_quare` - true if the square is a sub-square
///
/// # Returns:
///
/// true if pixels with different colors are within the square
fn square_has_different_pixels(
    pixels: &Vec<Pixel>,
    start: usize,
    end: usize,
    main_dimensions: usize,
    sub_dimensions: usize,
    sub_square: bool,
) -> bool {

    let mut horizontal_limit = start + sub_dimensions;

    for index in start..(end + 1) {

        /* prevent browsing the horizontal neighboor
           of the current square */
        if index >= horizontal_limit && sub_square {

            if index == horizontal_limit + main_dimensions - sub_dimensions - 1 {
                horizontal_limit += main_dimensions;
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
/// `main_square_dimensions` - the fixed width of the main square
/// `square_dimensions` - the width and height of the current square
/// `square_start` - the first index of the current square
/// `square_end` - the last index of the current square
/// `sub_node` - true if the created node is a sub-node
fn create_node(
    pixels: &Vec<Pixel>,
    node: &mut QuadTreeNode,
    main_square_dimensions: u32,
    square_dimensions: u32,
    square_start: usize,
    square_end: usize,
    sub_node: bool,
) {

    let different_pixels = square_has_different_pixels(
        &pixels,
        square_start,
        square_end,
        main_square_dimensions as usize,
        square_dimensions as usize,
        sub_node,
    );

    if different_pixels && square_dimensions != 1 {

        unsafe {
            allocateChildren(node)
        };

        let sub_square_dimensions = square_dimensions / 2;

        /* Quad tree node children are C-type raw pointers,
           dereferencing them is an unsafe action */

        node.data = HAS_CHILDREN_NODE;

        let bottom_left_square = unsafe {
            &mut (*node.children[0])
        };

        let bottom_left_square_end = (
            square_end -
            (main_square_dimensions * sub_square_dimensions) as usize -
            sub_square_dimensions as usize
        ) as usize;

        create_node(
            &pixels,
            bottom_left_square,
            main_square_dimensions,
            sub_square_dimensions,
            square_start,
            bottom_left_square_end,
            true,
        );

        let bottom_right_square = unsafe {
            &mut (*node.children[1])
        };

        let bottom_right_square_start = square_start +
            sub_square_dimensions as usize;
        let bottom_right_square_end = bottom_right_square_start as usize +
            sub_square_dimensions as usize +
            (main_square_dimensions * (sub_square_dimensions - 1)) as usize - 1;

        create_node(
            &pixels,
            bottom_right_square,
            main_square_dimensions,
            sub_square_dimensions,
            bottom_right_square_start,
            bottom_right_square_end,
            true,
        );

        let top_left_square = unsafe {
            &mut (*node.children[2])
        };

        let top_left_square_start = square_start +
            (sub_square_dimensions * main_square_dimensions) as usize;
        let top_left_square_end = top_left_square_start +
            (main_square_dimensions * sub_square_dimensions) as usize -
            main_square_dimensions as usize +
            sub_square_dimensions as usize - 1;

        create_node(
            &pixels,
            top_left_square,
            main_square_dimensions,
            sub_square_dimensions,
            top_left_square_start,
            top_left_square_end,
            true,
        );

        let top_right_square = unsafe {
            &mut (*node.children[3])
        };

        let top_right_square_start = square_start +
            ((main_square_dimensions + 1) * sub_square_dimensions) as usize;
        let top_right_square_end = top_right_square_start +
            (main_square_dimensions * sub_square_dimensions) as usize -
            main_square_dimensions as usize +
            sub_square_dimensions as usize - 1;

        create_node(
            &pixels,
            top_right_square,
            main_square_dimensions,
            sub_square_dimensions,
            top_right_square_start,
            top_right_square_end,
            true,
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

/// Recursively create squaresby browsing the quad tree content.
///
/// # Args:
///
/// `squares` - the array of squares where the squares must be added
/// `node` - the current browsed node of the quad tree
/// `square_dimensions` - the dimensions of the current square
/// `square_horizontal_position` - the horizontal position of the current square
/// `square_vertical_position` - the vertical position of the current square
fn create_square(
    squares: &mut Vec<Square>,
    node: &mut QuadTreeNode,
    square_dimensions: u32,
    square_horizontal_position: u32,
    square_vertical_position: u32,
) {

    let square = Square::new(
        square_horizontal_position,
        square_vertical_position,
        square_dimensions,
    );

    squares.push(square);

    if node.data == HAS_CHILDREN_NODE {

        let sub_square_dimensions = square_dimensions / 2;

        let bottom_left_node = unsafe {
            &mut (*node.children[0])
        };

        create_square(
            squares,
            bottom_left_node,
            sub_square_dimensions,
            square_horizontal_position,
            square_vertical_position + sub_square_dimensions,
        );

        let bottom_right_node = unsafe {
            &mut (*node.children[1])
        };

        create_square(
            squares,
            bottom_right_node,
            sub_square_dimensions,
            square_horizontal_position + sub_square_dimensions,
            square_vertical_position + sub_square_dimensions,
        );

        let top_left_node = unsafe {
            &mut (*node.children[2])
        };

        create_square(
            squares,
            top_left_node,
            sub_square_dimensions,
            square_horizontal_position,
            square_vertical_position,
        );

        let top_right_node = unsafe {
            &mut (*node.children[3])
        };

        create_square(
            squares,
            top_right_node,
            sub_square_dimensions,
            square_horizontal_position + sub_square_dimensions,
            square_vertical_position,
        );
    }
}

fn main() {

    let file_name = env::args().nth(1).expect("No input file.");
    let mut file = File::open(&file_name).expect("Cannot open file.");
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

    const OFFSET_BMP_RGB24: usize = 0x36;
    for (index, byte) in buffer.iter().skip(OFFSET_BMP_RGB24).enumerate() {

        let color_index = index as u32 % BYTES_PER_PIXEL;

        if index != 0 && (color_index == 0 || index == last_pixel_index) {

            pixels.push(
                Pixel::new(
                    red,
                    green,
                    blue,
                )
            );
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
        dimensions,
        0,
        (width * height - 1) as usize,
        false,
    );

    let mut squares: Vec<Square> = Vec::new();

    create_square(
        &mut squares,
        &mut node,
        dimensions,
        0,
        0,
    );

    /* display a picture instead of every pixels one by one,
       pixels array is only used to build the quad tree */

    let picture: G2dTexture = Texture::from_path(
        &mut window.factory,
        &file_name,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, graphics| {

                clear_screen(graphics);

                image(
                    &picture,
                    context.transform,
                    graphics,
                );

                for square in squares.iter() {

                    square.display(
                        context,
                        graphics,
                    );
                }
            }
        );
    }
}

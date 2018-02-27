extern crate piston_window;
extern crate graphics;

use piston_window::{
    PistonWindow,
    WindowSettings,
    G2d,
    clear,
};

use graphics::rectangle::Rectangle;
use graphics::context::Context;

use std::fs::File;
use std::io::Read;
use std::env;

/* the C library QuadTreeNode structure is:

   struct QuadTreeNode {
       QuadTreeNode* children[4];
       unsigned int data;
   };
*/
struct QuadTreeNode {
    children: [*mut QuadTreeNode; 4],
    data: u32,
}

#[link(name="quad_tree", kind="static")]
extern {

    fn create() -> QuadTreeNode;

    fn allocateChildren(node: *mut QuadTreeNode);
}

struct Pixel {
    rectangle: Rectangle,
    horizontal_position: f64,
    vertical_position: f64,
    red: u8,
    blue: u8,
    green: u8,
}

impl Pixel {

    /// Initializes a pixel.
    pub fn new(
        red: u8,
        green: u8,
        blue: u8,
        horizontal_position: u32,
        vertical_position: u32,
    ) -> Pixel {

        const MAXIMUM_COLOR_VALUE: f32 = 255.0;
        const ALPHA_COMMON: f32 = 1.0;

        Pixel {
            rectangle: Rectangle::new([
                red as f32 / MAXIMUM_COLOR_VALUE,
                green as f32 / MAXIMUM_COLOR_VALUE,
                blue as f32 / MAXIMUM_COLOR_VALUE,
                ALPHA_COMMON,
            ]),
            horizontal_position: horizontal_position as f64,
            vertical_position: vertical_position as f64,
            red: red,
            green: green,
            blue: blue,
        }
    }

    /// Displays the pixel at its position.
    ///
    /// # Args:
    ///
    /// * `context` - graphical context from the piston window
    /// * `graphics` - 2D graphics from the piston window
    pub fn display(
        &self,
        context: Context,
        graphics: &mut G2d,
    ) {

        const PIXEL_DIMENSION: f64 = 1.0;

        self.rectangle.draw(
            [
                self.horizontal_position,
                self.vertical_position,
                PIXEL_DIMENSION,
                PIXEL_DIMENSION,
            ],
            &context.draw_state,
            context.transform,
            graphics,
        );
    }
}

impl PartialEq for Pixel {

    /// Check if two Pixel objects are identical (based on colors only)
    ///
    /// # Args:
    ///
    /// `other` - the other pixel to compare with the current object
    ///
    /// # Returns:
    ///
    /// true if identical, false if different
    fn eq(
        &self,
        other: &Pixel,
    ) -> bool {

        if (
            self.red == other.red &&
            self.green == other.green &&
            self.blue == other.blue
        ) {
            return true;
        }

        return false;
    }
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
///
/// # Returns:
///
/// true if pixels with different colors are within the square
fn square_has_different_pixels(
    pixels: &Vec<Pixel>,
    mut start: usize,
    end: usize,
) -> bool {

    for index in start..(end + 1) {

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
/// `square_width` - the current square width
/// `square_height` - the current square height
/// `square_start` - the first index of the current square
/// `square_end` - the last index of the current square
///
/// FIXME: the width and height are supposed to be identicial all the time
fn create_node(
    pixels: &Vec<Pixel>,
    node: &mut QuadTreeNode,
    square_width: u32,
    square_height: u32,
    square_start: usize,
    square_end: usize,
) {

    println!("called");

    let different_pixels = square_has_different_pixels(
        &pixels,
        square_start,
        square_end,
    );

    if different_pixels && square_width != 1 {

        unsafe {
            allocateChildren(node as *mut QuadTreeNode)
        };

        let sub_square_width = square_width / 2;
        let sub_square_height = square_height / 2;

        /* bottom left sub-square */
        create_node(
            &pixels,
            node,
            sub_square_width,
            sub_square_height,
            square_start,
            (
                square_end -
                (square_width * sub_square_height) as usize -
                sub_square_width as usize
            ) as usize,
        );

    } else {

        let pixel: &Pixel = &pixels[square_start];

        const BITS_PER_COLOR: u8 = 8;
        node.data = pixel.red as u32;
        node.data <<= BITS_PER_COLOR;
        node.data += pixel.green as u32;
        node.data <<= BITS_PER_COLOR;
        node.data += pixel.blue as u32;
    }
}

fn main() {

    let file_name = env::args().nth(1).expect("No input file.");
    let mut file = File::open(file_name).expect("Cannot open file.");
    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer);

    let width = buffer[0x12] as u32;
    let height = buffer[0x16] as u32;

    if width != height {
        panic!("The image width and height must be identical.");
    }

    if width % 4 != 0 {
        panic!("The image width and height must be divisable by 4.");
    }

    let mut pixels: Vec<Pixel> = Vec::new();

    const BYTES_PER_PIXEL: u32 = 3;
    let last_pixel_index = (width * height * BYTES_PER_PIXEL - 1) as usize;

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

            if horizontal_position == width {
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
            width,
            height
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
        width,
        height,
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

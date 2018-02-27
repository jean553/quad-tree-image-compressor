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
            return false;
        }
    }

    return true;
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

    let mut red: u8 = 0;
    let mut green: u8 = 0;
    let mut blue: u8 = 0;

    let mut horizontal_position: u32 = 0;
    let mut vertical_position: u32 = height - 1;

    const OFFSET_BMP_RGB24: usize = 0x36;
    for (index, byte) in buffer.iter().skip(OFFSET_BMP_RGB24).enumerate() {

        const BYTES_PER_PIXEL: u32 = 3;
        let color_index = index as u32 % BYTES_PER_PIXEL;

        if index != 0 && color_index == 0 {

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

    let mut tree = unsafe {
        create()
    };

    let mut current_start: usize = 0;
    let mut current_end = (width * height - 1) as usize;

    /* FIXME: this should be recursive in order to build the whole quad tree */
    square_has_different_pixels(
        &pixels,
        current_start,
        current_end,
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

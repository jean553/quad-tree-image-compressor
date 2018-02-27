use piston_window::G2d;

use graphics::rectangle::Rectangle;
use graphics::context::Context;

pub struct Pixel {
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

    /// Getter of the red color value
    ///
    /// # Returns:
    ///
    /// red color value
    pub fn get_red(&self) -> u8 {
        self.red
    }

    /// Getter of the green color value
    ///
    /// # Returns:
    ///
    /// green color value
    pub fn get_green(&self) -> u8 {
        self.green
    }

    /// Getter of the blue color value
    ///
    /// # Returns:
    ///
    /// blue color value
    pub fn get_blue(&self) -> u8 {
        self.blue
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


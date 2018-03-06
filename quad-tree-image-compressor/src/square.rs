use piston_window::G2d;

use graphics::rectangle::Rectangle;
use graphics::context::Context;

pub struct Square {
    top_line: Rectangle,
    bottom_line: Rectangle,
    left_line: Rectangle,
    right_line: Rectangle,
    horizontal_position: f64,
    vertical_position: f64,
    dimensions: f64,
}

impl Square {

    /// Initializes a square.
    pub fn new(
        horizontal_position: u32,
        vertical_position: u32,
        dimensions: u32,
    ) -> Square {

        const SQUARE_COLOR: [f32; 4] = [
            1.0,
            0.0,
            0.0,
            1.0,
        ];

        /* TODO: check if using a Line instead of
           a Rectangle makes more sense */

        Square {
            top_line: Rectangle::new(SQUARE_COLOR),
            bottom_line: Rectangle::new(SQUARE_COLOR),
            left_line: Rectangle::new(SQUARE_COLOR),
            right_line: Rectangle::new(SQUARE_COLOR),
            horizontal_position: horizontal_position as f64,
            vertical_position: vertical_position as f64,
            dimensions: dimensions as f64,
        }
    }

    /// Displays the square at its position.
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

        const LINE_DIMENSION: f64 = 1.0;

        self.top_line.draw(
            [
                self.horizontal_position,
                self.vertical_position,
                self.dimensions,
                LINE_DIMENSION,
            ],
            &context.draw_state,
            context.transform,
            graphics,
        );

        self.bottom_line.draw(
            [
                self.horizontal_position,
                self.vertical_position + self.dimensions - 1.0,
                self.dimensions,
                LINE_DIMENSION,
            ],
            &context.draw_state,
            context.transform,
            graphics,
        );

        self.left_line.draw(
            [
                self.horizontal_position,
                self.vertical_position,
                LINE_DIMENSION,
                self.dimensions,
            ],
            &context.draw_state,
            context.transform,
            graphics,
        );

        self.right_line.draw(
            [
                self.horizontal_position + self.dimensions - 1.0,
                self.vertical_position,
                LINE_DIMENSION,
                self.dimensions,
            ],
            &context.draw_state,
            context.transform,
            graphics,
        );
    }
}


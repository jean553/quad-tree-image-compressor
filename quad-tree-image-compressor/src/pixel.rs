pub struct Pixel {
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
    ) -> Pixel {

        Pixel {
            red: red,
            green: green,
            blue: blue,
        }
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

        if self.red == other.red &&
            self.green == other.green &&
            self.blue == other.blue {
            return true;
        }

        return false;
    }
}

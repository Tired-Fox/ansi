use std::fmt::Display;
use std::iter;

/// Defines a foreground or background color.
///
/// Stores the ansi color code representing foreground or background
///
/// # Example
/// ```
/// use ansi::style::COLOR;
///
/// // Represents a foreground color (30)
/// COLOR::Foreground
/// 
/// // Represents a background color (40)
/// COLOR::Background
/// ```
pub enum COLOR {
    /// (30) Foreground Color
    Foreground = 30,
    /// (40) Background Color
    Background = 40,
}

/// Color structure. Represents red, green, and blue color values.
/// 
/// # Example
/// ```
/// use ansi::style::{Color, COLOR}
/// 
/// // Create a new rgb color
/// let red = Color::new(255, 0, 0);
/// 
/// // Create a new rgb color from a hex code
/// let hex = Color::hex("#eee").unwrap();
/// 
/// // Get the red, green, and blud values of the color as a tuple
/// let (r, g, b) = hex.rgb();
/// 
/// // Get the full ansi code for the color, '\x1b[38;2;255;0;0m'
/// let ansi = red.ansi(COLOR::Foreground)
/// 
/// // Get the ansi code for only the color, '48;2;255;0;0'
/// let code = red.code(COLOR::Background)
/// ```
#[derive(Debug ,Clone, Copy, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {

    /// Create a new rgb color
    /// 
    /// # Example
    /// ```
    /// use ansi::style::Color;
    /// 
    /// let red = Color::new(255, 0, 0);
    /// ```
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Get the rgb values as a tuple
    /// 
    /// # Example
    /// ```
    /// use ansi::style::Color;
    /// 
    /// let red = Color::new(255, 0, 0);
    /// let (r, g, b) = red.rgb();
    /// ```
    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Create a new rgb color from a hex code
    /// 
    /// The method can panic since it is parsing a string into an u8 value.
    /// 
    /// # Example
    /// ```
    /// use ansi::style::Color;
    /// 
    /// // With `#`
    /// Color::hex("#FFFFFF").unwrap()
    /// 
    /// // Without `#`
    /// Color::hex("FFFFFF").unwrap()
    /// 
    /// // Length of 3
    /// Color::hex("FFF").unwrap()
    /// ```
    pub fn hex(mut code: &str) -> Result<Self, &str> {
        let mut code = String::from(code);
        if code.starts_with("#") {
            code = code[1..].to_string();
        }

        if code.len() != 6 && code.len() != 3 {
            return Err("Invalid length hex code. Code should be 3 or 6 characters long.");
        }

        if code.len() == 3 {
            code = code
                .clone()
                .chars()
                .flat_map(|c| iter::repeat(c).take(2))
                .collect::<String>();
        }

        let r = u8::from_str_radix(&code[0..2], 16).unwrap();
        let g = u8::from_str_radix(&code[2..4], 16).unwrap();
        let b = u8::from_str_radix(&code[4..6], 16).unwrap();

        Ok(Color { r, g, b })
    }

    /// Get the colors full ansi code including the escape sequence.
    /// 
    /// # Example
    /// ```
    /// use ansi::style::{Color, COLOR};
    /// 
    /// let color = Color::new(255, 0, 0).unwrap()
    /// 
    /// // Full ansi code as a foreground color (\x1b[38;2;255;0;0m)
    /// color.ansi(COLOR::Foreground)
    /// 
    /// // Full ansi code as a background color (\x1b[48;2;255;0;0m)
    /// color.ansi(COLOR::Background)
    /// ```
    pub fn ansi(&self, state: COLOR) -> String {
        // Format based on inner values
        format!(
            "\x1b[{};2;{};{};{}",
            state as u8 + 8,
            self.r,
            self.g,
            self.b
        )
    }

    /// Get the colors ansi code without the escape sequence.
    /// 
    /// # Example
    /// ```
    /// use ansi::style::{Color, COLOR};
    /// 
    /// let color = Color::new(255, 0, 0).unwrap()
    /// 
    /// // Full ansi code as a foreground color (38;2;255;0;0)
    /// color.code(COLOR::Foreground)
    /// 
    /// // Full ansi code as a background color (48;2;255;0;0)
    /// color.code(COLOR::Background)
    /// ```
    pub fn code(&self, state: COLOR) -> String {
        format!("{};2;{};{};{}", state as u8 + 8, self.r, self.g, self.b)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

pub mod color;

pub use self::color::{Color, COLOR};
use std::{fmt::Display, ops};

/// Stores a style/format to apply to text.
///
/// The format include foreground and background color, bold, underline and hyperlink.
///
/// # Example
/// ```
/// use ansi::style::{ Style, Color };
///
/// let style = Style::new()
///     .foreground(Color::new(255, 0, 0))
///     .background(Color::hex("#fff").unwrap())
///     .bold()
///     .underline()
///     .url("https://example.com");
///
/// // Outputs `hello world` as red text on a white background, bold,
/// // and underlined If the terminal supports it, the text will
/// // also be a hyperlink to `https://example.com`
/// println!("{}", style.format("Hello World!"))
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Style {
    foreground: Option<Color>,
    background: Option<Color>,
    bold: bool,
    underline: bool,
    url: Option<String>,
}

impl ops::Add<Style> for Style {
    type Output = Style;

    fn add(self, rhs: Style) -> Self::Output {
        // bold
        let bold = rhs.bold | self.bold;
        // underline
        let underline = rhs.underline | self.underline;

        //? Based on match
        // url
        let url = match rhs.url {
            Some(_) => rhs.url,
            None => self.url,
        };

        // fg
        let foreground = match rhs.foreground {
            Some(_) => rhs.foreground,
            None => self.foreground,
        };

        // bg
        let background = match rhs.background {
            Some(_) => rhs.background,
            None => self.background,
        };

        Style {
            foreground,
            background,
            bold,
            underline,
            url,
        }
    }
}

impl Style {
    pub fn new() -> Self {
        Style {
            foreground: None,
            background: None,
            bold: false,
            underline: false,
            url: None,
        }
    }

    /// Build the ansi code that is placed before the text along with after the text to reset the format.
    ///
    /// # Example
    ///
    /// ```
    /// use ansi::style::{Style, Color};
    ///
    /// // Returns ("\x1b[1m", "\x1b[0m")
    /// let style = Style::new().bold();
    ///
    /// // Returns ("\x1b[38;2;1m", "\x1b[0m")
    /// let style = Style::new()
    ///     .bold()
    ///     .foreground(Color::new(123, 234, 52));
    /// ```
    pub fn ansi(&self) -> (String, String) {
        let mut formatting: Vec<String> = Vec::new();

        if self.bold {
            formatting.push("1".to_string())
        }

        if self.underline {
            formatting.push("4".to_string())
        }

        match self.foreground {
            Some(color) => formatting.push(color.code(COLOR::Foreground)),
            None => (),
        };

        match self.background {
            Some(color) => formatting.push(color.code(COLOR::Background)),
            None => (),
        };

        let mut ansi = String::new();
        let mut closing = String::new();
        if formatting.len() > 0 {
            ansi = format!("\x1b[{}m", formatting.join(";"));
            closing = "\x1b[0m".to_string();
        }

        (ansi, closing)
    }

    /// Formats text with ansi codes for terminal output
    ///
    /// # Arguments
    /// * `text` - A string that will be formatted with ansi
    ///
    /// # Example
    /// ```
    /// use ansi::style::{Style, Color};
    ///
    /// let style = Style::new()
    ///     .foreground(Color::new(255, 56, 128))
    ///     .background(Color::hex("#000").unwrap())
    ///     .bold();
    ///
    /// println!("{}", style.format("Sample Text"));
    /// ```
    pub fn format(&self, text: &str) -> String {
        let (ansi, closing) = self.ansi();

        match self.url.clone() {
            Some(url) => format!(
                "{}{}{}",
                ansi,
                format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, text),
                closing
            ),
            None => format!("{}{}{}", ansi, text, closing),
        }
    }

    /// Change the foreground color to `fg`
    ///
    /// Consumes `self` and returns updated `self`
    ///
    /// # Arguments
    /// * `fg` - Foreground color to style text with
    ///
    /// # Example
    /// ```
    /// use ansi::style::{Style, Color};
    ///
    /// let style = Style::new().foreground(Color::new(142, 123, 34));
    /// ```
    pub fn foreground(mut self, fg: Color) -> Self {
        self.foreground = Some(fg);
        self
    }

    /// Change the background color to `fg`
    ///
    /// Consumes `self` and returns updated `self`
    ///
    /// # Arguments
    /// * `bg` - Background color to style text with
    ///
    /// # Example
    /// ```
    /// use ansi::style::{Style, Color};
    ///
    /// let style = Style::new().background(Color::new(142, 123, 34));
    /// ```
    pub fn background(mut self, bg: Color) -> Self {
        self.background = Some(bg);
        self
    }

    /// Change the bold state of the style
    ///
    /// Consumes `self` and returns updated `self`
    ///
    /// # Example
    /// ```
    /// use ansi::style::{Style, Color};
    ///
    /// // Style is now bold
    /// let style = Style::new().bold();
    ///
    /// // Style is no longer bold
    /// let style = style.bold();
    /// ```
    pub fn bold(mut self) -> Self {
        self.bold = !self.bold;
        self
    }

    /// Change the underline state of the style
    ///
    /// Consumes `self` and returns updated `self`
    ///
    /// # Example
    /// ```
    /// use ansi::style::{Style, Color};
    ///
    /// // Style is now underlined
    /// let style = Style::new().underline();
    ///
    /// // Style is no longer underlined
    /// let style = style.underline();
    /// ```
    pub fn underline(mut self) -> Self {
        self.underline = !self.underline;
        self
    }

    /// Set the text to be a hyperlink along with what the link is to.
    ///
    /// Consumes `self` and returns updated `self`.
    ///
    /// # Example
    /// ```
    /// use ansi::style::{Style, Color};
    ///
    /// // Style is now underlined
    /// let style = Style::new().url("https://example.com");
    /// ```
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }
}

// Check values of style
impl Style {
    /// Bold state of the style
    pub fn is_bold(&self) -> bool {
        self.bold
    }

    /// Underline state of the style
    pub fn is_underline(&self) -> bool {
        self.underline
    }

    /// Foreground color. Either Some(Color) or None
    pub fn fg(&self) -> Color {
        match self.foreground.clone() {
            Some(color) => color,
            None => Color::new(255, 255, 255),
        }
    }

    /// Background color. Either Some(Color) or None
    pub fn bg(&self) -> Color {
        match self.background.clone() {
            Some(color) => color,
            None => Color::new(0, 0, 0),
        }
    }

    /// Url/hyperlink of the style. Either Some(String) or None
    pub fn link(&self) -> String {
        match self.url.clone() {
            Some(color) => color,
            None => String::new(),
        }
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fg = match self.foreground.clone() {
            Some(color) => color,
            None => Color::new(255, 255, 255),
        };

        let bg = match self.background.clone() {
            Some(color) => color,
            None => Color::new(0, 0, 0),
        };

        let url = match self.url.clone() {
            Some(url) => format!("'{}'", url),
            None => String::from("''"),
        };

        write!(
            f,
            "Style(fg={}, bg={}, bold={}, underline={}, url={})",
            fg, bg, self.bold, self.underline, url
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Style};

    #[test]
    fn color_new() {
        let rgb = Color::new(255, 123, 15);
        assert_eq!(rgb.rgb(), (255, 123, 15));
    }

    #[test]
    fn color_hex() {
        // Three character test
        let mut hex = Color::hex("FFF").unwrap();
        assert_eq!(hex.rgb(), (255, 255, 255));

        // Six character test
        hex = Color::hex("FFFFFF").unwrap();
        assert_eq!(hex.rgb(), (255, 255, 255));

        // With `#` test
        hex = Color::hex("#000000").unwrap();
        assert_eq!(hex.rgb(), (0, 0, 0));
    }

    #[test]
    #[should_panic]
    fn color_invalid_hex() {
        // With `#` test
        Color::hex("#00").unwrap();
    }

    #[test]
    fn style_default() {
        let style: Style = Style::new();

        assert_eq!(
            style,
            Style {
                foreground: None,
                background: None,
                bold: false,
                underline: false,
                url: None
            }
        );
    }

    #[test]
    fn style_genrator() {
        let style: Style = Style::new();
        assert_eq!(style.is_bold(), false);
        assert_eq!(style.is_underline(), false);
        assert_eq!(style.fg(), Color::new(255, 255, 255));
        assert_eq!(style.bg(), Color::new(0, 0, 0));
        assert_eq!(style.link(), "".to_string());

        let style = style
            .bold()
            .underline()
            .foreground(Color::new(255, 0, 155))
            .background(Color::new(0, 155, 255))
            .url("https://example.com");

        assert_eq!(style.is_bold(), true);
        assert_eq!(style.is_underline(), true);
        assert_eq!(style.fg(), Color::new(255, 0, 155));
        assert_eq!(style.bg(), Color::new(0, 155, 255));
        assert_eq!(style.link(), String::from("https://example.com"));
    }

    #[test]
    fn style_format() {
        let mut style = Style::new().bold();

        let result = style.format("result");
        assert_eq!(result, String::from("\x1b[1mresult\x1b[0m"));

        let style = style.foreground(Color::new(255, 0, 155));
        let result = style.format("result");
        assert_eq!(result, String::from("\x1b[1;38;2;255;0;155mresult\x1b[0m"))
    }

    #[test]
    fn style_composition() {
        let bold = Style::new().bold();
        let uline: Style = Style::new().underline();

        let bold_underline: Style = bold + uline;

        assert_eq!(
            bold_underline,
            Style {
                foreground: None,
                background: None,
                bold: true,
                underline: true,
                url: None
            }
        );
    }
}

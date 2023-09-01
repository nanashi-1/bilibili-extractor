use std::fmt::Display;

pub trait Colorer {
    /// Color string as a success. (Bold Green)
    fn color_as_success(&self) -> String
    where
        Self: Display,
    {
        format!("\x1b[1;32m{self}\x1b[0m")
    }

    /// Color string as a warning. (Bold Yellow)
    fn color_as_warning(&self) -> String
    where
        Self: Display,
    {
        format!("\x1b[1;33m{self}\x1b[0m")
    }

    /// Color string as an error. (Bold Red)
    fn color_as_error(&self) -> String
    where
        Self: Display,
    {
        format!("\x1b[1;31m{self}\x1b[0m")
    }
}

impl Colorer for String {}
impl Colorer for &str {}

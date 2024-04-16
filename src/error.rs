use std::fmt::Display;

#[derive(Debug)]
pub enum AppError {
    WrongDirection,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

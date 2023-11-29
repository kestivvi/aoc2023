use std::{
    fmt::{Display, Formatter, Result},
    fs, io,
};

pub enum InputType {
    Real,
    Test,
}

impl Display for InputType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InputType::Real => write!(f, "real"),
            InputType::Test => write!(f, "test"),
        }
    }
}

pub fn read_input(day: u8, input_type: InputType) -> io::Result<String> {
    fs::read_to_string(format!("./inputs/day{day:02}_{input_type}.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(read_input(0, InputType::Test).unwrap(), "Hello Santa!");
    }
}

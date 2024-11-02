#[derive(Clone, Copy, Debug)]
pub enum Color {
    /// wrong
    Black,
    /// correct
    Green,
    /// exists in other position
    Yellow,
}

impl TryFrom<u8> for Color {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'0' => Ok(Color::Black),
            b'1' => Ok(Color::Yellow),
            b'2' => Ok(Color::Green),
            _ => Err(ParseError::InvalidDigit),
        }
    }
}

/// returns pairs of alpha cell,
/// an alpha cell contains a color and a lowercase letter.
pub fn parse_line(input: &str) -> Result<Vec<(Color, char)>, ParseError> {
    let Some((alphas, colors)) = input.split_once(' ') else {
        return Err(ParseError::NoSpace);
    };
    alphas
        .bytes()
        .zip(colors.bytes())
        .map(|(alpha, digit)| Ok((digit.try_into()?, alpha as char)))
        .collect()
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("color must be one of \'0\', \'1\' or \'2\'")]
    InvalidDigit,
    #[error("non-alpha found in input")]
    NotAlpha,
    #[error("no space was found in input")]
    NoSpace,
}

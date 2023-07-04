#[derive(Clone, Copy, Debug)]
pub enum Color {
    Black,
    Green,
    Yellow,
}

impl TryFrom<char> for Color {
    type Error = ParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'b' => Ok(Color::Black),
            'y' => Ok(Color::Yellow),
            'g' => Ok(Color::Green),
            _ => Err(ParseError::InvalidChar),
        }
    }
}

/// returns pairs of alpha cell,
/// an alpha cell contains a color and a lowercase letter.
pub fn parse_line(
    input: &str,
) -> impl Iterator<Item = Result<(Color, char), ParseError>> + ExactSizeIterator + Clone + '_ {
    input.as_bytes().chunks_exact(2).map(|pair| {
        let color: Color = (pair[0] as char).try_into()?;
        let alpha = pair[1] as char;
        if !alpha.is_alphabetic() {
            return Err(ParseError::NotAlpha);
        }
        Ok((color, alpha.to_ascii_lowercase()))
    })
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("color must be one of \'b\', \'y\' or \'g\'")]
    InvalidChar,
    #[error("non-alpha found in input")]
    NotAlpha,
}

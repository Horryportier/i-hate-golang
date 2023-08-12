use std::fmt::Display;

use nu_ansi_term::{AnsiString, Style};

use crate::traits::{FromStyled, Renderable};

pub struct Span<T: Display> {
   pub content: T,
   pub style: Style,
}

impl<T: Display + Clone> From<T> for Span<T> {
    fn from(content: T) -> Span<T> {
        let style = Style::default();
        Span { content, style }
    }
}

impl<T: Display + Clone> FromStyled<T> for Span<T> {
    fn styled(content: T, style: Style) -> Span<T> {
        Span { content, style }
    }
}

impl<T: Display + Clone> Renderable for Span<T> {
    fn rander(&self, raw: bool) -> AnsiString {
        if raw {
            return format!("{}", self.content).into();
        }
        self.style.paint(self.content.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_span() {
        let style = Style::new()
            .on(nu_ansi_term::Color::Red)
            .fg(nu_ansi_term::Color::White);
        let span = Span::styled(true, style);
        println!("{}", span.rander(false));
        assert_eq!(1, 2)
    }
}

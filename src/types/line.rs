use std::fmt::Display;

use nu_ansi_term::{AnsiString, Style};

use crate::traits::{FromStyled, Renderable};

use super::span::Span;

struct Line<T: Display + Clone>(Vec<Span<T>>);

impl<T: Display + Clone> From<Vec<T>> for Line<T> {
    fn from(value: Vec<T>) -> Self {
        let spans = value
            .iter()
            .map(|f| {
                let style = Style::default();
                let a = Span {
                    content: f.clone(),
                    style,
                };
                a
            })
            .collect::<Vec<Span<T>>>();
        Line(spans)
    }
}

impl<T: Display + Clone> FromStyled<Vec<T>> for Line<T> {
    fn styled(value: Vec<T>, style: Style) -> Self {
        let spans = value
            .iter()
            .map(|f| {
                let a = Span {
                    content: f.clone(),
                    style,
                };
                a
            })
            .collect::<Vec<Span<T>>>();
        Line(spans)
    }
}

impl<T: Display + Clone> From<T> for Line<T> {
    fn from(value: T) -> Self {
        let span = Span::from(value);
        Line(vec![span])
    }
}

impl<T: Display + Clone> FromStyled<T> for Line<T> {
    fn styled(value: T, style: Style) -> Self {
        Line(vec![Span::styled(value, style)])
    }
}

impl<T: Display + Clone> Renderable for Line<T> {
    fn rander(&self, raw: bool) -> AnsiString {
        self.0
            .iter()
            .map(|f| f.rander(raw).to_string())
            .collect::<Vec<String>>()
            .join(" ")
            .into()
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line() {
        let style = Style::new()
            .on(nu_ansi_term::Color::Red)
            .fg(nu_ansi_term::Color::White);
        let line = Line::styled(vec!["how", "are", "you"], style);
        println!("{}", line.rander(false));
        assert_eq!(1, 2)
    }
}


use nu_ansi_term::{Style, AnsiString};


pub trait  Renderable  {
    fn rander(&self, raw: bool) -> AnsiString;
}

pub trait FromStyled<T> {
    fn styled(value: T, style: Style) -> Self;  
}



use crate::style::{align::*, color::Color, text::Text};
use crate::unit::*;

pub struct Attr {
    pub(crate) anchor: Point,
    pub(crate) selected: bool,
    pub(crate) hide: bool,
    pub(crate) width: Unit,
    pub(crate) height: Unit,
    pub(crate) alignx: AlignX,
    pub(crate) aligny: AlignY,
    pub(crate) text_style: Text,
    pub(crate) text_color: Color,
    pub(crate) arc: bool,
    pub(crate) hide_border: bool,
    pub(crate) border_color: Color,
    pub(crate) title: String,
    pub(crate) hide_title: bool,
    pub(crate) title_align: AlignX,
}

impl Attr {
    pub fn new() -> Attr {
        Attr {
            anchor: Point::cor(1, 1),
            selected: false,
            hide: false,
            width: Unit::Cor(0),
            height: Unit::Cor(0),
            alignx: AlignX::Left,
            aligny: AlignY::Top,
            text_style: Text::NoBold,
            text_color: Color::None,
            arc: false,
            hide_border: false,
            border_color: Color::None,
            title: String::new(),
            hide_title: true,
            title_align: AlignX::Left,
        }
    }

    pub fn width(mut self, value: Unit) -> Attr {
        self.width = value;
        self
    }

    pub fn height(mut self, value: Unit) -> Attr {
        self.height = value;
        self
    }

    pub fn alignx(mut self, value: AlignX) -> Attr {
        self.alignx = value;
        self
    }

    pub fn aligny(mut self, value: AlignY) -> Attr {
        self.aligny = value;
        self
    }

    pub fn text_style(mut self, value: Text) -> Attr {
        self.text_style = value;
        self
    }

    pub fn text_color(mut self, value: Color) -> Attr {
        self.text_color = value;
        self
    }

    pub fn arc(mut self, value: bool) -> Attr {
        self.arc = value;
        self
    }

    pub fn hide_border(mut self, value: bool) -> Attr {
        self.hide_border = value;
        self
    }

    pub fn border_color(mut self, value: Color) -> Attr {
        self.border_color = value;
        self
    }

    pub fn title(mut self, value: String) -> Attr {
        self.title = value;
        self.hide_title = false;
        self
    }

    pub fn hide_title(mut self, value: bool) -> Attr {
        self.hide_title = value;
        self
    }

    pub fn title_align(mut self, value: AlignX) -> Attr {
        self.title_align = value;
        self
    }
}

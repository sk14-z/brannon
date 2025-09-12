use crate::{
    key::Key,
    style::{
        align::*,
        color::{Color, ColorBG},
        orientation::Orientation,
        text::TextStyle,
    },
    unit::*,
    widget::Widget,
};

#[derive(Clone)]
pub struct Attr {
    pub(crate) hide: bool,

    pub(crate) tag: &'static str,
    pub(crate) selected: bool,
    pub(crate) flex: bool,

    pub(crate) orientation: Orientation,

    pub(crate) width: Unit,
    pub(crate) height: Unit,

    pub(crate) padding_top: Unit,
    pub(crate) padding_right: Unit,
    pub(crate) padding_bottom: Unit,
    pub(crate) padding_left: Unit,

    pub(crate) alignx: AlignX,
    pub(crate) aligny: AlignY,

    pub(crate) text_style: TextStyle,
    pub(crate) text_color: Color,

    pub(crate) fill: ColorBG,

    pub(crate) arc: bool,

    pub(crate) hide_border: bool,
    pub(crate) border_color: Color,
    pub(crate) border_fill: ColorBG,

    pub(crate) title: String,
    pub(crate) hide_title: bool,
    pub(crate) title_align: AlignX,

    pub(crate) binds: Vec<(Key, String)>,
    pub(crate) hide_binds: bool,
    pub(crate) binds_align: AlignX,
}

impl Default for Attr {
    fn default() -> Self {
        Attr::new()
    }
}

impl Attr {
    pub fn new() -> Attr {
        Attr {
            hide: false,
            tag: "",
            selected: false,
            flex: false,
            orientation: Orientation::Vertical,
            width: Unit::Cor(0),
            height: Unit::Cor(0),
            padding_top: Unit::Cor(0),
            padding_right: Unit::Cor(0),
            padding_bottom: Unit::Cor(0),
            padding_left: Unit::Cor(0),
            alignx: AlignX::Left,
            aligny: AlignY::Top,
            fill: ColorBG::None,
            text_style: TextStyle::NoBold,
            text_color: Color::None,
            arc: false,
            hide_border: false,
            border_color: Color::None,
            border_fill: ColorBG::None,
            title: String::new(),
            hide_title: true,
            title_align: AlignX::Left,
            binds: vec![],
            hide_binds: true,
            binds_align: AlignX::Left,
        }
    }

    pub fn hidden(&mut self) -> bool {
        self.hide
    }

    pub fn wrap(&mut self) -> Option<Attr> {
        Some(self.to_owned())
    }

    pub fn tag(&mut self, value: &'static str) -> &mut Attr {
        self.tag = value;
        self
    }

    pub fn flex(&mut self, value: bool) -> &mut Attr {
        self.flex = value;
        self
    }

    pub fn orientation(&mut self, value: Orientation) -> &mut Attr {
        self.orientation = value;
        self
    }

    pub fn size<T: Into<Unit>>(&mut self, xvalue: T, yvalue: T) -> &mut Attr {
        self.width = xvalue.into();
        self.height = yvalue.into();
        self
    }

    pub fn width(&mut self, value: impl Into<Unit>) -> &mut Attr {
        self.width = value.into();
        self
    }

    pub fn total_width(&self) -> Unit {
        self.padding_left + self.width + self.padding_right
    }

    pub fn inc_width(&mut self, value: impl Into<Unit>) -> &mut Attr {
        self.width += value.into();
        self
    }

    pub fn dec_width(&mut self, value: impl Into<Unit>) -> &mut Attr {
        self.width -= value.into();
        self
    }

    pub fn height(&mut self, value: impl Into<Unit>) -> &mut Attr {
        self.height = value.into();
        self
    }

    pub fn total_height(&self) -> Unit {
        self.padding_top + self.height + self.padding_bottom
    }

    pub fn inc_height(&mut self, value: impl Into<Unit>) -> &mut Attr {
        self.height += value.into();
        self
    }

    pub fn dec_height(&mut self, value: impl Into<Unit>) -> &mut Attr {
        self.height -= value.into();
        self
    }

    pub fn padding(&mut self, value: impl Into<Unit>) -> &mut Attr {
        let value = value.into();
        self.padding_top = value;
        self.padding_right = value;
        self.padding_bottom = value;
        self.padding_left = value;
        self
    }

    pub fn paddingd<T: Into<Unit> + Copy>(&mut self, value: &[T]) -> &mut Attr {
        match value.len() {
            1 => {
                let p = value[0].into();
                self.padding_top = p;
                self.padding_right = p;
                self.padding_bottom = p;
                self.padding_left = p;
            }
            2 => {
                let pttb = value[0].into();
                let pltr = value[1].into();
                self.padding_top = pttb;
                self.padding_right = pltr;
                self.padding_bottom = pttb;
                self.padding_left = pltr;
            }
            4 => {
                self.padding_top = value[0].into();
                self.padding_right = value[1].into();
                self.padding_bottom = value[2].into();
                self.padding_left = value[3].into();
            }
            _ => {}
        }
        self
    }

    pub fn paddingx(&mut self, value: impl Into<Unit>) -> &mut Attr {
        let value = value.into();
        self.padding_right = value;
        self.padding_left = value;
        self
    }

    pub fn paddingy(&mut self, value: impl Into<Unit>) -> &mut Attr {
        let value = value.into();
        self.padding_top = value;
        self.padding_bottom = value;
        self
    }

    pub fn padding_top(&mut self, value: impl Into<Unit>) -> &mut Attr {
        self.padding_top = value.into();
        self
    }

    pub fn padding_right(&mut self, value: impl Into<Unit>) -> &mut Attr {
        self.padding_right = value.into();
        self
    }

    pub fn padding_bottom(&mut self, value: impl Into<Unit>) -> &mut Attr {
        self.padding_bottom = value.into();
        self
    }

    pub fn padding_left(&mut self, value: impl Into<Unit>) -> &mut Attr {
        self.padding_left = value.into();
        self
    }

    pub fn align(&mut self, xvalue: AlignX, yvalue: AlignY) -> &mut Attr {
        self.alignx = xvalue;
        self.aligny = yvalue;
        self
    }

    pub fn alignx(&mut self, value: AlignX) -> &mut Attr {
        self.alignx = value;
        self
    }

    pub fn aligny(&mut self, value: AlignY) -> &mut Attr {
        self.aligny = value;
        self
    }

    pub fn fill(&mut self, value: ColorBG) -> &mut Attr {
        self.fill = value;
        self.border_fill = value;
        self
    }

    pub fn text(&mut self, style_value: TextStyle, color_value: Color) -> &mut Attr {
        self.text_style = style_value;
        self.text_color = color_value;
        self
    }

    pub fn text_style(&mut self, value: TextStyle) -> &mut Attr {
        self.text_style = value;
        self
    }

    pub fn text_color(&mut self, value: Color) -> &mut Attr {
        self.text_color = value;
        self
    }

    pub fn arc(&mut self, value: bool) -> &mut Attr {
        self.arc = value;
        self
    }

    pub fn hide_border(&mut self, value: bool) -> &mut Attr {
        self.hide_border = value;
        self
    }

    pub fn border_color(&mut self, value: Color) -> &mut Attr {
        self.border_color = value;
        self
    }

    pub fn border_fill(&mut self, value: ColorBG) -> &mut Attr {
        self.border_fill = value;
        self
    }

    pub fn title(&mut self, value: String) -> &mut Attr {
        self.title = value;
        self.hide_title = false;
        self
    }

    pub fn hide_title(&mut self, value: bool) -> &mut Attr {
        self.hide_title = value;
        self
    }

    pub fn title_align(&mut self, value: AlignX) -> &mut Attr {
        self.title_align = value;
        self
    }

    pub fn binds(&mut self, value: Vec<(Key, String)>) -> &mut Attr {
        self.binds = value;
        self.hide_binds = false;
        self
    }

    pub fn hide_binds(&mut self, value: bool) -> &mut Attr {
        self.hide_binds = value;
        self
    }

    pub fn binds_align(&mut self, value: AlignX) -> &mut Attr {
        self.binds_align = value;
        self
    }
}

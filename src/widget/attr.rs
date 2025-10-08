use crate::{
    input::binds::Binds,
    style::{
        align::*,
        color::{Color, ColorBG},
        orientation::Orientation,
        text::TextStyle,
    },
    unit::*,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Attr {
    pub hide: bool,

    pub(crate) tag: String,
    pub(crate) should_fill: bool,
    pub(crate) selected: bool,

    pub flex: bool,

    pub orientation: Orientation,

    pub width: Unit,
    pub height: Unit,

    pub padding_top: Unit,
    pub padding_right: Unit,
    pub padding_bottom: Unit,
    pub padding_left: Unit,

    pub alignx: AlignX,
    pub aligny: AlignY,

    pub text_style: TextStyle,
    pub text_color: Color,

    pub fill: ColorBG,

    pub arc: bool,

    pub hide_border: bool,
    pub border_color: Color,
    pub border_fill: ColorBG,

    pub title: String,
    pub hide_title: bool,
    pub title_align: AlignX,

    pub binds: Binds,
    pub hide_binds: bool,
    pub binds_align: AlignX,
}

impl Attr {
    pub fn new() -> Attr {
        Attr {
            hide: false,
            tag: String::new(),
            should_fill: false,
            selected: false,
            flex: false,
            orientation: Orientation::Vertical,
            width: Unit::CoR(0),
            height: Unit::CoR(0),
            padding_top: Unit::CoR(1),
            padding_right: Unit::CoR(1),
            padding_bottom: Unit::CoR(1),
            padding_left: Unit::CoR(1),
            alignx: AlignX::Left,
            aligny: AlignY::Top,
            fill: ColorBG::None,
            text_style: TextStyle::NoBold,
            text_color: Color::White,
            arc: false,
            hide_border: false,
            border_color: Color::White,
            border_fill: ColorBG::None,
            title: String::new(),
            hide_title: true,
            title_align: AlignX::Left,
            binds: Binds::new(),
            hide_binds: true,
            binds_align: AlignX::Left,
        }
    }

    pub fn with(attr: &Attr) -> Attr {
        attr.clone()
    }

    pub fn from(attr: &Attr) -> Option<Attr> {
        Some(attr.clone())
    }

    pub fn wrap(&mut self) -> Option<Attr> {
        Some(self.to_owned())
    }

    pub fn hidden(&mut self) -> bool {
        self.hide
    }

    pub fn tag(&mut self, value: impl Into<String>) -> &mut Attr {
        self.tag = value.into();
        self
    }

    pub fn flex(&mut self) -> &mut Attr {
        self.flex = true;
        self
    }

    pub fn no_flex(&mut self) -> &mut Attr {
        self.flex = false;
        self
    }

    pub fn orientation(&mut self, value: Orientation) -> &mut Attr {
        self.orientation = value;
        self
    }

    pub fn horizontal(&mut self) -> &mut Attr {
        self.orientation = Orientation::Horizontal;
        self
    }

    pub fn vertical(&mut self) -> &mut Attr {
        self.orientation = Orientation::Vertical;
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

    pub fn pad(&mut self, value: impl Into<Unit>) -> &mut Attr {
        let value = value.into();
        self.padding_top = value;
        self.padding_right = value;
        self.padding_bottom = value;
        self.padding_left = value;
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

    pub fn center(&mut self) -> &mut Attr {
        self.alignx = AlignX::Center;
        self.aligny = AlignY::Center;
        self
    }

    pub fn center_all(&mut self) -> &mut Attr {
        self.alignx = AlignX::Center;
        self.aligny = AlignY::Center;
        self.title_align = AlignX::Center;
        self.binds_align = AlignX::Center;
        self
    }

    pub fn alignx(&mut self, value: AlignX) -> &mut Attr {
        self.alignx = value;
        self
    }

    pub fn centerx(&mut self) -> &mut Attr {
        self.alignx = AlignX::Center;
        self
    }

    pub fn aligny(&mut self, value: AlignY) -> &mut Attr {
        self.aligny = value;
        self
    }

    pub fn centery(&mut self) -> &mut Attr {
        self.aligny = AlignY::Center;
        self
    }

    pub fn fill(&mut self, value: ColorBG) -> &mut Attr {
        self.fill = value;
        self.border_fill = value;
        self
    }

    pub fn fg(&mut self, value: Color) -> &mut Attr {
        self.text_color = value;
        self.border_color = value;
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

    pub fn bold(&mut self) -> &mut Attr {
        self.text_style = TextStyle::Bold;
        self
    }

    pub fn italic(&mut self) -> &mut Attr {
        self.text_style = TextStyle::Italic;
        self
    }

    pub fn underline(&mut self) -> &mut Attr {
        self.text_style = TextStyle::Underline;
        self
    }

    pub fn text_color(&mut self, value: Color) -> &mut Attr {
        self.text_color = value;
        self
    }

    pub fn arc(&mut self) -> &mut Attr {
        self.arc = true;
        self
    }

    pub fn square(&mut self) -> &mut Attr {
        self.arc = false;
        self
    }

    pub fn show_border(&mut self, value: bool) -> &mut Attr {
        self.hide_border = !value;
        self
    }

    pub fn hide_border(&mut self) -> &mut Attr {
        self.hide_border = true;
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

    pub fn title(&mut self, value: impl Into<String>) -> &mut Attr {
        self.title = value.into();
        self.hide_title = false;
        self
    }

    pub fn show_title(&mut self) -> &mut Attr {
        self.hide_title = false;
        self
    }

    pub fn hide_title(&mut self) -> &mut Attr {
        self.hide_title = true;
        self
    }

    pub fn title_align(&mut self, value: AlignX) -> &mut Attr {
        self.title_align = value;
        self
    }

    pub fn binds(&mut self, value: impl Into<Binds>) -> &mut Attr {
        self.binds = value.into();
        self.hide_binds = false;
        self
    }

    pub fn show_binds(&mut self) -> &mut Attr {
        self.hide_binds = false;
        self
    }

    pub fn hide_binds(&mut self) -> &mut Attr {
        self.hide_binds = true;
        self
    }

    pub fn binds_align(&mut self, value: AlignX) -> &mut Attr {
        self.binds_align = value;
        self
    }
}

impl Default for Attr {
    fn default() -> Self {
        Attr::new()
    }
}

mod tests {
    use super::*;

    #[test]
    fn default_new_matches_expectations() {
        let a = Attr::new();
        assert!(!a.hide);
        assert_eq!(a.tag, "");
        assert_eq!(a.width, Unit::CoR(0));
        assert_eq!(a.height, Unit::CoR(0));
        assert_eq!(a.padding_top, Unit::CoR(1));
        assert_eq!(a.padding_right, Unit::CoR(1));
        assert_eq!(a.padding_bottom, Unit::CoR(1));
        assert_eq!(a.padding_left, Unit::CoR(1));
        assert_eq!(a.alignx, AlignX::Left);
        assert_eq!(a.aligny, AlignY::Top);
        assert_eq!(a.fill, ColorBG::None);
        assert_eq!(a.text_style, TextStyle::NoBold);
        assert_eq!(a.text_color, Color::White);
        assert!(!a.arc);
        assert!(!a.hide_border);
        assert_eq!(a.border_color, Color::White);
        assert_eq!(a.border_fill, ColorBG::None);
        assert!(a.title.is_empty());
        assert!(a.hide_title);
        assert!(a.hide_binds);
    }

    #[test]
    fn size_and_total_dimensions() {
        let mut a = Attr::new();
        a.width(10usize).height(5usize);
        assert_eq!(a.width, Unit::CoR(10));
        assert_eq!(a.height, Unit::CoR(5));
        // default padding 1 on each side
        assert_eq!(a.total_width(), Unit::CoR(12)); // 1 + 10 + 1
        assert_eq!(a.total_height(), Unit::CoR(7)); // 1 + 5 + 1
    }

    #[test]
    fn inc_and_dec_dimensions() {
        let mut a = Attr::new();
        a.width(5usize).inc_width(3usize).dec_width(2usize);
        assert_eq!(a.width, Unit::CoR(6));
        a.height(8usize).inc_height(4usize).dec_height(3usize);
        assert_eq!(a.height, Unit::CoR(9));
    }

    #[test]
    fn paddingd_variants() {
        let mut a = Attr::new();
        a.paddingd(&[3usize]);
        assert_eq!(a.padding_top, Unit::CoR(3));
        assert_eq!(a.padding_right, Unit::CoR(3));
        assert_eq!(a.padding_bottom, Unit::CoR(3));
        assert_eq!(a.padding_left, Unit::CoR(3));

        a.paddingd(&[2usize, 5usize]);
        assert_eq!(a.padding_top, Unit::CoR(2));
        assert_eq!(a.padding_bottom, Unit::CoR(2));
        assert_eq!(a.padding_right, Unit::CoR(5));
        assert_eq!(a.padding_left, Unit::CoR(5));

        a.paddingd(&[1usize, 2usize, 3usize, 4usize]);
        assert_eq!(a.padding_top, Unit::CoR(1));
        assert_eq!(a.padding_right, Unit::CoR(2));
        assert_eq!(a.padding_bottom, Unit::CoR(3));
        assert_eq!(a.padding_left, Unit::CoR(4));
    }

    #[test]
    fn pad_helpers() {
        let mut a = Attr::new();
        a.pad(4usize);
        assert_eq!(a.padding_top, Unit::CoR(4));
        assert_eq!(a.padding_left, Unit::CoR(4));
        a.paddingx(7usize);
        assert_eq!(a.padding_left, Unit::CoR(7));
        assert_eq!(a.padding_right, Unit::CoR(7));
        a.paddingy(2usize);
        assert_eq!(a.padding_top, Unit::CoR(2));
        assert_eq!(a.padding_bottom, Unit::CoR(2));
    }

    #[test]
    fn alignment_helpers() {
        let mut a = Attr::new();
        a.center();
        assert_eq!(a.alignx, AlignX::Center);
        assert_eq!(a.aligny, AlignY::Center);

        a.center_all();
        assert_eq!(a.alignx, AlignX::Center);
        assert_eq!(a.aligny, AlignY::Center);
        assert_eq!(a.title_align, AlignX::Center);
        assert_eq!(a.binds_align, AlignX::Center);

        a.align(AlignX::Right, AlignY::Bottom);
        assert_eq!(a.alignx, AlignX::Right);
        assert_eq!(a.aligny, AlignY::Bottom);
    }

    #[test]
    fn orientation_and_flex() {
        let mut a = Attr::new();
        a.horizontal();
        assert_eq!(a.orientation, Orientation::Horizontal);
        a.vertical();
        assert_eq!(a.orientation, Orientation::Vertical);
        a.flex();
        assert!(a.flex);
        a.no_flex();
        assert!(!a.flex);
    }

    #[test]
    fn fill_and_fg_affect_border() {
        let mut a = Attr::new();
        a.fill(ColorBG::Red);
        assert_eq!(a.fill, ColorBG::Red);
        assert_eq!(a.border_fill, ColorBG::Red);

        a.fg(Color::Blue);
        assert_eq!(a.text_color, Color::Blue);
        assert_eq!(a.border_color, Color::Blue);
    }

    #[test]
    fn text_helpers() {
        let mut a = Attr::new();
        a.text(TextStyle::Bold, Color::Green);
        assert_eq!(a.text_style, TextStyle::Bold);
        assert_eq!(a.text_color, Color::Green);

        a.italic();
        assert_eq!(a.text_style, TextStyle::Italic);
        a.underline();
        assert_eq!(a.text_style, TextStyle::Underline);
        a.bold();
        assert_eq!(a.text_style, TextStyle::Bold);
    }

    #[test]
    fn border_controls() {
        let mut a = Attr::new();
        a.show_border(true);
        assert!(!a.hide_border);
        a.show_border(false);
        assert!(a.hide_border);
        a.hide_border();
        assert!(a.hide_border);
        a.border_color(Color::Cyan);
        assert_eq!(a.border_color, Color::Cyan);
    }

    #[test]
    fn title_controls() {
        let mut a = Attr::new();
        a.title("Hello");
        assert_eq!(a.title, "Hello");
        assert!(!a.hide_title);
        a.hide_title();
        assert!(a.hide_title);
        a.show_title();
        assert!(!a.hide_title);
    }

    #[test]
    fn binds_controls() {
        let mut a = Attr::new();
        let kb = Binds::new();
        a.binds(kb.clone());
        assert!(!a.hide_binds);
        a.hide_binds();
        assert!(a.hide_binds);
        a.show_binds();
        assert!(!a.hide_binds);
    }

    #[test]
    fn tag_and_arc_square() {
        let mut a = Attr::new();
        a.tag("panel-1");
        assert_eq!(a.tag, "panel-1");
        a.arc();
        assert!(a.arc);
        a.square();
        assert!(!a.arc);
    }
}

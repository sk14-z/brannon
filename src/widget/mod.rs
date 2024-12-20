pub mod attr;
pub mod label;

pub trait Widget {
    fn style(&mut self) -> &mut attr::Attr;

    fn set_style(&mut self, attr: attr::Attr) {
        (*self.style()) = attr;
    }

    fn render(&self);

    fn show(&mut self) {
        self.style().hide = false;
    }

    fn hide(&mut self) {
        self.style().hide = true;
    }
}

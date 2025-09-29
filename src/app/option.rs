use crate::{input::key::Protocol, theme::Theme};

#[allow(clippy::large_enum_variant)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AppOption {
    Theme(Theme),
    NoInterrupt(bool),
    RefreshRate(usize),
    CaptureMouse(bool),
    KeyProtocol(Protocol),
}

// TODO
// Distinguish between what can be changed at runtime and what requires a restart

pub struct AppOptions {
    pub(crate) theme: Theme,
    pub no_interrupt: bool,
    pub(crate) refresh_rate: usize,
    pub(crate) capture_mouse: bool,
    pub(crate) key_protocol: Protocol,
}

impl AppOptions {
    pub(crate) fn new() -> Self {
        Self {
            theme: Theme::new(),
            no_interrupt: true,
            refresh_rate: 30,
            capture_mouse: false,
            key_protocol: Protocol::Default,
        }
    }

    pub fn set(&mut self, option: AppOption) {
        match option {
            AppOption::Theme(t) => self.theme = t,
            AppOption::NoInterrupt(b) => self.no_interrupt = b,
            AppOption::RefreshRate(n) => self.refresh_rate = n,
            AppOption::CaptureMouse(b) => self.capture_mouse = b,
            AppOption::KeyProtocol(p) => self.key_protocol = p,
        }
    }
}

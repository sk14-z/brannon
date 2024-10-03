use std::fs::File;

use brannon::{
    app::App,
    draw::cursor::left,
    key::Key,
    log::Logger,
    panel::Panel,
    style::{
        align::{AlignX, AlignY},
        color::{Color, ColorBG},
        orientation::Orientation,
        text::Text,
    },
    unit::Unit,
    widget::{
        alert::{Alert, AlertType},
        attr::Attr,
        container::Container,
        label::Label,
        progress_bar::ProgressBar,
        Widget,
    },
};

fn init(app: &mut App) {
    app.frame
        .attr
        .alignx(AlignX::Center)
        .aligny(AlignY::Center)
        .title(String::from("Brannon"));

    let label = Label::new(
        String::from("C to change color"),
        Attr::new()
            .tag("la")
            .width(Unit::Cor(30))
            .height(Unit::Cor(5))
            .padding(Unit::Cor(1))
            .alignx(AlignX::Center)
            .aligny(AlignY::Center)
            .text_color(Color::Cyan)
            .text_style(Text::Underline)
            .border_color(Color::Cyan)
            .wrap(),
    );

    let mut c1 = Container::new(Attr::new().tag("c1").height(Unit::Cor(0)).wrap());
    c1.attr.orientation(Orientation::Horizontal);
    let mut c2 = Container::new(Attr::new().tag("c2").height(Unit::Cor(0)).wrap());
    c2.attr.orientation(Orientation::Horizontal);

    let l1 = Label::new(
        String::from("This isn't hidden"),
        Attr::new()
            .tag("12")
            .width(Unit::Cor(30))
            .height(Unit::Cor(5))
            .padding(Unit::Cor(1))
            .alignx(AlignX::Center)
            .aligny(AlignY::Center)
            .text_color(Color::Cyan)
            .text_style(Text::Underline)
            .border_color(Color::Cyan)
            .wrap(),
    );
    let l2 = Label::new(
        String::from("This also isn't hidden"),
        Attr::new()
            .tag("l2")
            .width(Unit::Cor(30))
            .height(Unit::Cor(5))
            .padding(Unit::Cor(1))
            .alignx(AlignX::Center)
            .aligny(AlignY::Center)
            .text_color(Color::Cyan)
            .text_style(Text::Underline)
            .border_color(Color::Cyan)
            .wrap(),
    );
    let hidden = Label::new(
        String::from("Should be hidden but isn't"),
        Attr::new()
            .tag("target")
            .width(Unit::Cor(30))
            .height(Unit::Cor(5))
            .padding(Unit::Cor(1))
            .alignx(AlignX::Center)
            .aligny(AlignY::Center)
            .text_color(Color::Cyan)
            .text_style(Text::Underline)
            .border_color(Color::Cyan)
            .wrap(),
    );

    c1.add(l1);
    c1.add(label);
    c2.add(l2);
    c2.add(hidden);

    app.frame.add(c1);
    app.frame.add(c2);
}

fn run(app: &mut App, input: Option<Key>) -> Option<usize> {
    if let Some(key) = input {
        if key == Key::q {
            return None;
        } else if key == Key::c {
            app.get_widget("target")?.as_label()?.hide();
            app.frame.children[1].as_container()?.shrink();
        } else if key == Key::C {
            app.get_widget("target")?.as_label()?.show();
            app.frame.children[1].as_container()?.flex();
        }
    }

    Some(0)
}

fn main() {
    let mut app = App::new();
    app.init = init;
    app.run = run;
    app.start();
}

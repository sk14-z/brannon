use std::{fs::File, ops::Deref};

use brannon::{
    app::{cache::*, App},
    draw::cursor::left,
    key::Key,
    log::Logger,
    panel::Panel,
    printf,
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
        .align(AlignX::Center, AlignY::Center)
        .title(String::from("Brannon"));

    let mut label_style = Attr::new()
        .size(30, 5)
        .padding(1)
        .align(AlignX::Center, AlignY::Center)
        .text_color(Color::Cyan)
        .text_style(Text::Underline)
        .border_color(Color::Cyan)
        .clone();

    let label = Label::new(String::from("Press c dummy"), label_style.tag("la").wrap());
    let l1 = ProgressBar::new(ColorBG::Cyan, label_style.tag("11").wrap());
    let l2 = Label::new(
        String::from("This also isn't hidden"),
        label_style.tag("l2").wrap(),
    );
    let hidden = Label::new(
        String::from("This isn't hidden"),
        label_style.tag("target").wrap(),
    );

    let mut c1 = Container::new(Attr::new().tag("c1").height(0).wrap());
    c1.attr
        .orientation(Orientation::Horizontal)
        .aligny(AlignY::Center);
    c1.addm(vec![l1, label]);

    let mut c2 = Container::new(
        Attr::new()
            .tag("c2")
            .height(0)
            .binds(vec![(Key::C, String::from("Unhide it dummy"))])
            .binds_align(AlignX::Center)
            .wrap(),
    );
    c2.attr.orientation(Orientation::Horizontal);
    c2.addm(vec![l2, hidden]);

    app.frame.addm(vec![c1, c2]);
}

fn run(app: &mut App, input: Option<Key>) -> Option<usize> {
    if let Some(key) = input {
        if key == Key::q {
            return None;
        } else if key == Key::c {
            let w = app.frame.children[1].as_container()?.remove("target");

            if let Some(label) = w {
                app.cache::<Box<dyn Widget>>().add("removed", label);
                app.frame.style_all(|w| {
                    w.style_mut().border_color(Color::Green);
                });
            }
        } else if key == Key::C {
            let w = app.cache::<Box<dyn Widget>>().remove("removed");

            if let Some(label) = w {
                app.frame.children[1].as_container()?.add(*label);
                app.frame.style_all(|w| {
                    w.style_mut().border_color(Color::Cyan);
                });
            }
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

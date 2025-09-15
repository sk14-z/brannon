mod widget;

use brannon::{
    app::App,
    key::{Key, binds::KeyBinds},
    log::Logger,
    panel::Panel,
    printf,
    style::{
        align::{AlignX, AlignY},
        color::{Color, ColorBG},
        orientation::Orientation,
        text::TextStyle,
    },
    unit::Unit,
    widget::{Widget, attr::Attr, container::Container, label::Label, progress_bar::ProgressBar},
};

use widget::MyWidget;

fn init(app: &mut App) {
    app.frame.attr.center().title("Brannon");

    let mut label_style = Attr::new().size(30, 5).pad(1).center().clone();

    let mine = MyWidget::new(label_style.tag("mine").wrap());

    let label = Label::new(String::from("Press c dummy"), label_style.tag("la").wrap());

    let l1 = ProgressBar::new(
        ColorBG::Cyan,
        label_style.clone().tag("11").width(75).wrap(),
    );

    let l2 = Label::new(
        format!("{}", l1.attr.height.calc()),
        label_style.tag("l2").wrap(),
    );

    let hidden = Label::new(
        String::from("This isn't hidden"),
        label_style.tag("target").wrap(),
    );

    let gd = Label::new(
        String::from("0"),
        Attr::new()
            // label_style
            .tag("gd")
            // .size(15, 5)
            .size(Unit::Cor(15), Unit::PctV(30))
            .center()
            .fg(Color::Red)
            .wrap(),
    );

    let mut c1 = Container::new(
        Attr::new()
            .tag("c1")
            .fill(ColorBG::Green)
            .horizontal()
            .centery()
            .flex()
            .wrap(),
    );
    c1.addm(vec![l1, mine, label]);

    let mut c2 = Container::new(
        Attr::new()
            .tag("c2")
            .horizontal()
            .flex()
            .center_all()
            .binds(
                KeyBinds::new()
                    .bind(Key::C, "Hide")
                    .bind(Key::I, "Inc")
                    .bind(Key::D, "Dec"),
            )
            .wrap(),
    );

    c2.addm(vec![l2, hidden, gd]);

    app.frame.addm(vec![c1, c2]);

    // app.map_all(|w| {
    //     if w.as_panel().is_none() && w.style().fill == ColorBG::None {
    //         w.style_mut().fill = ColorBG::Blue;
    //     }
    // });
}

fn run(app: &mut App, input: Option<Key>) -> Option<usize> {
    let c = app.get_widget::<Container>("c2")?;
    let s = format!("{} {}", c.bounds().1.calc(), c.attr.height.calc());
    let l = app.get_widget::<Label>("gd")?;
    l.text = format!("inner/outer: {}, h: {}", s, l.attr.height.calc());

    if let Some(key) = input {
        if key == Key::q {
            return None;
        } else if key == Key::c {
            app.toggle_widget("target");
        } else if key == Key::i {
            app.get_widget::<ProgressBar>("11")?.inc_progress(10);
        } else if key == Key::d {
            app.get_widget::<ProgressBar>("11")?.dec_progress(10);
        }
    }

    Some(0)
}

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let mut app = App::new();

    app.init = init;
    app.run = run;
    app.start();
}

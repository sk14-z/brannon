mod widget;

use brannon::{
    app::App,
    key::Key,
    log::Logger,
    panel::Panel,
    printf,
    style::{
        align::{AlignX, AlignY},
        color::{Color, ColorBG},
        orientation::Orientation,
        text::TextStyle,
    },
    widget::{attr::Attr, container::Container, label::Label, progress_bar::ProgressBar, Widget},
};

use widget::MyWidget;

fn init(app: &mut App) {
    app.frame
        .attr
        .align(AlignX::Center, AlignY::Center)
        .fill(ColorBG::RGB(255, 255, 255))
        .border_fill(ColorBG::RGB(100, 100, 100))
        .title(String::from("Brannon"));

    let mut label_style = Attr::new()
        .size(30, 5)
        .padding(1)
        .align(AlignX::Center, AlignY::Center)
        .fill(ColorBG::RGB(50, 0, 100))
        .clone();

    let mine = MyWidget::new(label_style.tag("mine").wrap());

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

    let mut c1 = Container::new(Attr::new().tag("c1").fill(ColorBG::Green).flex(true).wrap());
    c1.attr
        .orientation(Orientation::Horizontal)
        .aligny(AlignY::Center);
    c1.addm(vec![l1, mine, label]);

    let mut c2 = Container::new(
        Attr::new()
            .tag("c2")
            .flex(true)
            .binds(vec![(Key::c, String::from("Unhide it dummy"))])
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
    let mut app = App::new();

    app.init = init;
    app.run = run;
    app.start();
}

// mod widget;

use std::any::Any;

use brannon::{
    app::App,
    input::{binds::KeyBinds, key::Key},
    make_scene_key,
    panel::{Panel, frame::Frame},
    printf,
    scene::SceneKey,
    style::{
        color::{Color, ColorBG},
        text::TextStyle,
    },
    widget::{attr::Attr, label::Label},
};

// use widget::MyWidget;

#[derive(Clone, Copy, PartialEq)]
enum MyScenes {
    Scene1,
    Scene2,
}

make_scene_key!(MyScenes);

fn init(app: &mut App) {
    let label_style = Attr::new().size(30, 5).fg(Color::Black).center().to_owned();

    app.create_scene(MyScenes::Scene1, {
        let mut frame = Frame::new(
            Attr::new()
                .center_all()
                .title("Welcome")
                .fg(Color::Black)
                .fill(ColorBG::White)
                .binds(KeyBinds::new().bind(Key::Enter, "Continue"))
                .wrap(),
        );

        let welcome = Label::new(
            String::from("Welcome to Brannon!"),
            Attr::with(&label_style)
                .tag("welcome")
                .fg(Color::Blue)
                .bold()
                .wrap(),
        );

        let info = Label::new(
            String::from("Press Enter..."),
            Attr::with(&label_style).text_style(TextStyle::Dim).wrap(),
        );

        frame.addm(vec![welcome, info]);

        frame.style_all(|a| {
            a.fill(ColorBG::White);
        });

        frame
    });

    app.create_scene(MyScenes::Scene2, {
        let mut frame = Frame::new(
            Attr::new()
                .centery()
                .title("Content")
                .fg(Color::Black)
                .fill(ColorBG::Red)
                .binds(KeyBinds::new().bind(Key::h, "Toggle that loudmouth"))
                .wrap(),
        );

        let info2 = Label::new(
            String::from("This is the second scene!"),
            Attr::from(&label_style),
        );

        let to_be_hidden = Label::new(
            String::from("Do it! I'm not scared. You can't get rid of me."),
            Attr::with(&label_style)
                .fill(ColorBG::Red)
                .tag("target")
                .wrap(),
        );

        frame.addm(vec![info2, to_be_hidden]);

        frame.style_all(|a| {
            a.fill(ColorBG::Cyan);
            a.hide_border();
        });

        frame
    });

    app.change_scene(&mut MyScenes::Scene1);
}

fn run(app: &mut App, input: Option<Key>) -> Option<usize> {
    if let Some(key) = input {
        if key == Key::q {
            return None;
        } else {
            app.get_widget::<Label>("welcome")?.text = format!("{}", key);
            printf!("Key")
        }

        // match app.current_scene_key()? {
        //     MyScenes::Scene1 => {
        //         if key == Key::LF {
        //             app.change_scene(&mut MyScenes::Scene2);
        //         }
        //     }
        //     MyScenes::Scene2 => {
        //         if key == Key::h {
        //             app.toggle_visiblity_of("target");
        //         }
        //     }
        // }
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
    // brannon::event::arg::test_some_shit();
    // printf!("\x1b]2;Yo\u{7}");
    // std::thread::sleep(Duration::from_secs(5))
}

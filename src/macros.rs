#[macro_export]
macro_rules! printf {
    ($($fmt:tt)*) => {{
        use std::io::{stdout, Write};
        print!($($fmt)*);
        stdout().flush().unwrap();
    }};
}

#[macro_export]
macro_rules! printlnf {
    ($($fmt:tt)*) => {{
        use std::io::{stdout, Write};
        println!($($fmt)*);
        stdout().flush().unwrap();
    }};
}

#[macro_export]
macro_rules! impl_widget_base {
    ($t:ty) => {
        use $crate::widget::WidgetBase;
        impl WidgetBase for $t {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            fn style(&self) -> &Attr {
                &(self.attr)
            }

            fn style_mut(&mut self) -> &mut Attr {
                &mut (self.attr)
            }

            fn wclone(&self) -> Box<dyn Widget> {
                Box::new(self.clone())
            }

            fn weq(&self, other: &dyn Widget) -> bool {
                other
                    .as_any()
                    .downcast_ref::<Self>()
                    .map_or(false, |o| self == o)
            }
        }
    };
}

#[macro_export]
macro_rules! panel_shared {
    () => {
        fn split(&self) -> (&Attr, &Vec<Box<dyn Widget>>) {
            (&self.attr, &self.children.0)
        }

        fn split_mut(&mut self) -> (&mut Attr, &mut Vec<Box<dyn Widget>>) {
            (&mut self.attr, &mut self.children.0)
        }
    };
}

#[macro_export]
macro_rules! make_scene_key {
    ($t:ty) => {
        impl SceneKey for $t {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn eq(&self, other: &dyn SceneKey) -> bool {
                other
                    .as_any()
                    .downcast_ref::<Self>()
                    .map_or(false, |o| self == o)
            }
        }
    };
}

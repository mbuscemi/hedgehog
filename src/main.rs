use relm::{Relm, Update, Widget};
use gtk::{ContainerExt, GtkWindowExt, Orientation::Vertical, WidgetExt, Window, WindowType};
use relm_derive::Msg;

const APP_TITLE: &str = "Hedgehog";

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Model;

#[derive(Clone)]
struct Widgets {
    window: Window,
}

struct Win {
    _model: Model,
    widgets: Widgets,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(_: &Relm<Self>, model: Self::Model) -> Self {
        let vbox = gtk::Box::new(Vertical, 0);
        
        let window = Window::new(WindowType::Toplevel);
        window.add(&vbox);

        window.set_title(APP_TITLE);
        window.show_all();

        Win {
            _model: model,
            widgets: Widgets {
                window
            }
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}

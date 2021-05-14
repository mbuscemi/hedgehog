use relm::{Relm, Update, Widget};
use gtk::{Button, GtkWindowExt, HeaderBar, HeaderBarExt, IconSize, WidgetExt, Window, WindowType};
use relm_derive::Msg;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Model;

#[derive(Clone)]
struct Widgets {
    header_bar: HeaderBar,
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
        let button = Button::from_icon_name(Some("open-menu-symbolic"), IconSize::Button);

        let header_bar = HeaderBar::new();
        header_bar.pack_end(&button);
        header_bar.set_show_close_button(true);
        
        let window = Window::new(WindowType::Toplevel);
        window.set_titlebar(Some(&header_bar));

        window.show_all();

        Win {
            _model: model,
            widgets: Widgets {
                header_bar,
                window
            }
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}

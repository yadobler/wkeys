use gtk::glib::clone;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use relm4::{gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

use tracing::info;

use crate::service::KeyboardHandle;

pub struct UIModel {
    keyboard_handle: Box<dyn KeyboardHandle>,
}

#[derive(Debug)]
pub enum UIMessage {
    Press,
    Release,
}

impl SimpleComponent for UIModel {
    type Init = Box<dyn KeyboardHandle>;

    type Input = UIMessage;
    type Output = ();
    type Root = gtk::Window;
    type Widgets = ();

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Simple app")
            .default_width(300)
            .default_height(100)
            .build()
    }

    // view! {
    //     gtk::Window {
    //         init_layer_shell: (),
    //         set_layer: Layer::Overlay,

    //         set_anchor: (Edge::Left, true),
    //         set_anchor: (Edge::Right, true),
    //         set_anchor: (Edge::Top, false),
    //         set_anchor: (Edge::Bottom, true),

    //         gtk::Box {
    //             set_orientation: gtk::Orientation::Horizontal,
    //             set_valign(gtk::Align::Center),
    //             set_spacing: 5,
    //             set_margin_all: 5,

    //             gtk::Button {
    //                 set_label: "Press",
    //                 connect_clicked => UIMessage::Press

    //             },

    //             gtk::Button::with_label("Release") {
    //                 connect_clicked => UIMessage::Release
    //             },
    //         }
    //     }
    // }

    // Initialize the UI.
    fn init(
        handle: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        window.init_layer_shell();
        window.set_layer(Layer::Overlay);

        let anchors = [
            (Edge::Left, true),
            (Edge::Right, true),
            (Edge::Top, false),
            (Edge::Bottom, true),
        ];

        for (anchor, state) in anchors {
            window.set_anchor(anchor, state);
        }

        let model = UIModel {
            keyboard_handle: handle,
        };

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .build();

        let inc_button = gtk::Button::with_label("Press");
        let dec_button = gtk::Button::with_label("Release");

        window.set_child(Some(&vbox));
        vbox.set_margin_all(5);
        vbox.set_align(gtk::Align::Center);
        vbox.append(&inc_button);
        vbox.append(&dec_button);

        let inc_sender = sender.clone();
        inc_button.connect_clicked(move |_| {
            inc_sender.input(UIMessage::Press);
        });

        let dec_sender = sender.clone();
        dec_button.connect_clicked(move |_| {
            dec_sender.input(UIMessage::Release);
        });

        let widgets = ();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            UIMessage::Press => {
                info!("Press");
                self.keyboard_handle.key_press(evdev::Key::KEY_A);
            }
            UIMessage::Release => {
                info!("Release");
                self.keyboard_handle.key_release(evdev::Key::KEY_A);
            }
        }
    }

    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}

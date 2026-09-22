use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label, Button};
use gtk4_layer_shell::{Layer, LayerShell};

fn main() {
    let app = Application::builder()
        .application_id("no.johron.Rabbagast")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);

        window.init_layer_shell();
        
        window.set_layer(Layer::Top);
        window.set_anchor(gtk4_layer_shell::Edge::Bottom, true);
        window.set_anchor(gtk4_layer_shell::Edge::Left, true);
        window.set_anchor(gtk4_layer_shell::Edge::Right, true);

        window.auto_exclusive_zone_enable();

        let window_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        window_container.set_hexpand(true);
        window.set_child(Some(&window_container));

        let left_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
        window_container.append(&left_container);
        left_container.set_hexpand(true);
        left_container.set_halign(gtk4::Align::Start);
        
        let center_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
        window_container.append(&center_container);
        center_container.set_hexpand(true);
        center_container.set_halign(gtk4::Align::Center);
        
        let right_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
        window_container.append(&right_container);
        right_container.set_hexpand(true);
        right_container.set_halign(gtk4::Align::End);

        let ws_button = Button::with_label("Test");
        left_container.append(&ws_button);

        let label = Label::new(Some("Du er ein ordentleg rabbagast asso!"));
        label.set_margin_top(8);
        label.set_margin_bottom(8);
        center_container.append(&label);

        let pill_button = Button::with_label("Test");
        right_container.append(&pill_button);

        window.present();
    });

    app.run();
}

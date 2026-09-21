use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label};
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

        let label = Label::new(Some("Du er ein ordentleg rabbagast asso!"));
        label.set_margin_top(8);
        label.set_margin_bottom(8);
        window.set_child(Some(&label));

        window.present();
    });

    app.run();
}

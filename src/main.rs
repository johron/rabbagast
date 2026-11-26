use std::env;

use adw::Application as AdwApplication;
use gtk4::glib;
use gtk4::prelude::*;

mod bar;
mod model;
mod util;

use crate::bar::render_bar;
use crate::model::hyprland::WMHyprland;
use crate::model::model::Model;
use crate::model::model::WindowManager;

fn main() -> glib::ExitCode {
    let app = AdwApplication::builder()
        .application_id("com.github.johron.Rabbagast")
        .build();

    app.connect_activate(move |app| {
        // for now just assume hyprland
        // let mut window_manager = WMHyprland::new();

        let mut window_manager: Option<Box<dyn WindowManager>> =
            match env::var("XDG_CURRENT_DESKTOP") {
                Ok(val) => match val.to_lowercase().as_str() {
                    "hyprland" => Some(Box::new(WMHyprland::new())),
                    // "sway" => WMSway::new(),
                    _ => None,
                },
                Err(e) => None,
            };

        if let Some(window_manager) = &mut window_manager {
            let mut model = Model::new();

            window_manager.update();
            window_manager.populate(&mut model);

            render_bar(app, &model);
        } else {
            panic!("Unsupported desktop")
        }
    });

    app.run()
}

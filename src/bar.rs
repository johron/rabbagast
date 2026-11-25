use adw::Application as AdwApplication;
use gtk4::prelude::*;
use gtk4::{Align, ApplicationWindow, Box, Button, Label, Orientation};
use gtk4_layer_shell::{Layer, LayerShell};

use chrono::{Datelike, Local};

use crate::model::Model;
use crate::util::get_chrono_locale;

pub fn render_bar(app: &AdwApplication, model: &Model) {
    let window = ApplicationWindow::new(app);
    window.init_layer_shell();

    window.set_layer(Layer::Top);
    window.set_anchor(gtk4_layer_shell::Edge::Bottom, true);
    window.set_anchor(gtk4_layer_shell::Edge::Left, true);
    window.set_anchor(gtk4_layer_shell::Edge::Right, true);
    window.auto_exclusive_zone_enable();

    let window_container = Box::new(Orientation::Horizontal, 0);
    window_container.set_hexpand(true);
    window_container.set_homogeneous(true);
    window.set_child(Some(&window_container));

    let left_container = Box::new(Orientation::Horizontal, 5);
    window_container.append(&left_container);
    render_workspaces(model, &left_container);

    let center_wrapper = Box::new(Orientation::Horizontal, 0);
    let center_container = Box::new(Orientation::Horizontal, 5);
    center_container.set_hexpand(true);
    center_container.set_halign(Align::Center);
    center_wrapper.append(&center_container);
    window_container.append(&center_wrapper);
    render_date(model, &center_container);

    let right_container = Box::new(Orientation::Horizontal, 5);
    window_container.append(&right_container);

    let pill_button = Button::with_label("Test");
    pill_button.set_hexpand(true);
    pill_button.set_halign(Align::End);
    right_container.append(&pill_button);

    window.present();
}

fn render_workspaces(model: &Model, container: &Box) {
    for ws in model.workspaces.clone() {
        let label = Label::new(Some(&ws.to_string()));
        container.append(&label);
    }
}

fn render_date(model: &Model, container: &Box) {
    let now = Local::now();
    let locale = get_chrono_locale();

    let time_label = Label::new(Some(&now.format("%H:%M").to_string()));
    container.append(&time_label);

    let day_raw = now.format_localized("%A", locale).to_string();
    let day = day_raw.trim();

    let short_month: String = now
        .format_localized("%B", locale)
        .to_string()
        .chars()
        .filter(|c| c.is_alphabetic())
        .take(3)
        .collect();

    let date_text = format!("{}, {}. {}.", day, now.day(), short_month);
    let date_label = Label::new(Some(&date_text));
    container.append(&date_label);
}

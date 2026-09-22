use gtk4::prelude::*;
use gtk4::{glib, ApplicationWindow, Box, Label, Orientation, Button};
use adw::Application as AdwApplication;
use gtk4_layer_shell::{Layer, LayerShell};

use std::cell::RefCell;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::mpsc::{self, Sender};
use std::time::Duration;
use std::{env, thread};

#[derive(Debug)]
enum HyprEvent {
    WorkspaceChanged(String),
    WorkspaceCreated(String),
    WorkspaceDestroyed(String),
}

fn main() -> glib::ExitCode {
    let app = AdwApplication::builder()
        .application_id("no.johron.Rabbagast")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &AdwApplication) {
    let window = ApplicationWindow::new(app);
    window.init_layer_shell();
    
    window.set_layer(Layer::Top);
    window.set_anchor(gtk4_layer_shell::Edge::Bottom, true);
    window.set_anchor(gtk4_layer_shell::Edge::Left, true);
    window.set_anchor(gtk4_layer_shell::Edge::Right, true);
    window.auto_exclusive_zone_enable();

    let window_container = Box::new(Orientation::Horizontal, 0);
    window_container.set_hexpand(true);
    window.set_child(Some(&window_container));

    let left_container = Box::new(Orientation::Horizontal, 5);
    left_container.set_hexpand(true);
    left_container.set_halign(gtk4::Align::Start);
    window_container.append(&left_container);
    
    let center_container = Box::new(Orientation::Horizontal, 5);
    center_container.set_hexpand(true);
    center_container.set_halign(gtk4::Align::Center);
    window_container.append(&center_container);
    
    let right_container = Box::new(Orientation::Horizontal, 5);
    right_container.set_hexpand(true);
    right_container.set_halign(gtk4::Align::End);
    window_container.append(&right_container);

    let ws_button = Button::with_label("Test");
    left_container.append(&ws_button);

    let status_label = Label::new(Some("Active Workspace: 1"));
    status_label.set_margin_top(8);
    status_label.set_margin_bottom(8);
    center_container.append(&status_label);

    let pill_button = Button::with_label("Test");
    right_container.append(&pill_button);

    let (tx, rx) = mpsc::channel::<HyprEvent>();
    let rx = Rc::new(RefCell::new(rx));

    glib::timeout_add_local(Duration::from_millis(16), {
        let rx = Rc::clone(&rx);
        let label = status_label.clone();

        move || {
            while let Ok(event) = rx.borrow_mut().try_recv() {
                match event {
                    HyprEvent::WorkspaceChanged(name) => {
                        label.set_text(&format!("Active Workspace: {}", name));
                    }
                    HyprEvent::WorkspaceCreated(name) => {
                        println!("Workspace created: {}", name);
                    }
                    HyprEvent::WorkspaceDestroyed(name) => {
                        println!("Workspace destroyed: {}", name);
                    }
                }
            }
            glib::ControlFlow::Continue
        }
    });

    thread::spawn(move || {
        if let Err(e) = run_hyprland_listener(tx) {
            eprintln!("Hyprland socket error: {}", e);
        }
    });

    window.present();
}

fn run_hyprland_listener(tx: Sender<HyprEvent>) -> std::io::Result<()> {
    let xdg_runtime = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR not set");
    let instance = env::var("HYPRLAND_INSTANCE_SIGNATURE").expect("Not running inside Hyprland");

    let socket_path = PathBuf::from(xdg_runtime)
        .join("hypr")
        .join(instance)
        .join(".socket2.sock");

    let stream = UnixStream::connect(socket_path)?;
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let line = line?;
        if let Some((event_type, args)) = line.split_once(">>") {
            println!("{:?}", event_type);
            match event_type {
                "workspace" => {
                    let _ = tx.send(HyprEvent::WorkspaceChanged(args.to_string()));
                }
                "createworkspace" => {
                    let _ = tx.send(HyprEvent::WorkspaceCreated(args.to_string()));
                }
                "destroyworkspace" => {
                    let _ = tx.send(HyprEvent::WorkspaceDestroyed(args.to_string()));
                }
                _ => {}
            }
        }
    }

    Ok(())
}

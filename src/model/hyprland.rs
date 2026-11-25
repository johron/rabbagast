use serde::Deserialize;
use std::env;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

use crate::model::model::{Model, WindowManager};

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub monitor: String,
    pub windows: i32,
}

pub struct WMHyprland {
    workspaces: Vec<Workspace>,
    active_workspace: Option<Workspace>,
}

impl WMHyprland {
    pub fn new() -> Self {
        Self {
            workspaces: vec![],
            active_workspace: None,
        }
    }
}

impl WindowManager for WMHyprland {
    fn populate(&self, model: &mut Model) {
        model.workspaces = self.workspaces.iter().map(|w| w.id as usize).collect();

        if let Some(active_workspace) = &self.active_workspace {
            model.active_workspace = active_workspace.id as usize;
        }
    }

    fn update(&mut self) {
        if let Ok(workspaces) = get_workspaces() {
            self.workspaces = workspaces;
        }

        if let Ok(active_workspace) = get_active_workspace() {
            self.active_workspace = Some(active_workspace);
        } else {
            self.active_workspace = None;
        }
    }
}

fn socket_path(name: &str) -> Result<PathBuf, String> {
    let runtime_dir =
        env::var("XDG_RUNTIME_DIR").map_err(|_| "XDG_RUNTIME_DIR not set".to_string())?;
    let his = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .map_err(|_| "HYPRLAND_INSTANCE_SIGNATURE not set".to_string())?;

    Ok(PathBuf::from(runtime_dir).join("hypr").join(his).join(name))
}

fn query_json<T: for<'de> Deserialize<'de>>(cmd: &str) -> Result<T, Box<dyn std::error::Error>> {
    let path = socket_path(".socket.sock")?;

    let mut stream = UnixStream::connect(path)?;
    stream.write_all(cmd.as_bytes())?;
    stream.shutdown(std::net::Shutdown::Write)?;

    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;

    Ok(serde_json::from_str(&buf)?)
}

fn get_workspaces() -> Result<Vec<Workspace>, Box<dyn std::error::Error>> {
    query_json("j/workspaces")
}

fn get_active_workspace() -> Result<Workspace, Box<dyn std::error::Error>> {
    query_json("j/activeworkspace")
}

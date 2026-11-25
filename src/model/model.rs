pub struct Model {
    pub workspaces: Vec<usize>,
    pub active_workspace: usize,
}

pub trait WindowManager {
    fn populate(&self, model: &mut Model);
    fn update(&mut self);
}

impl Model {
    pub fn new() -> Self {
        Self {
            workspaces: vec![],
            active_workspace: 0,
        }
    }
}

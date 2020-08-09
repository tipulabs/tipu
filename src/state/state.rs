#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ApplicationFocus {
    Chat,
    GitChat,
    Terminal,
    ScriptBar,
}

#[derive(Debug)]
pub struct ApplicationState {
    pub current_focus_position: ApplicationFocus,
}

impl Default for ApplicationState {
    fn default() -> Self {
        ApplicationState {
            current_focus_position: ApplicationFocus::Terminal,
        }
    }
}

impl ApplicationState {
    pub fn update_focus_state(&mut self, focus_state: ApplicationFocus) {
        self.current_focus_position = focus_state
    }
}

// Application State ->

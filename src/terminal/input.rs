use std::collections::HashMap;

#[derive(Clone)]
pub enum Action {
    Submit,
    Clear,
    Exit,
    HistoryUp,
    HistoryDown,
    TabComplete,
    None,
}

pub struct InputHandler {
    keybindings: HashMap<String, Action>,
}

impl InputHandler {
    pub fn new() -> Self {
        let mut keybindings = HashMap::new();
        keybindings.insert("Enter".to_string(), Action::Submit);
        keybindings.insert("Ctrl+C".to_string(), Action::Exit);
        keybindings.insert("Ctrl+L".to_string(), Action::Clear);
        keybindings.insert("Up".to_string(), Action::HistoryUp);
        keybindings.insert("Down".to_string(), Action::HistoryDown);
        keybindings.insert("Tab".to_string(), Action::TabComplete);
        Self { keybindings }
    }

    pub fn parse_key(&self, input: &str) -> Action {
        self.keybindings.get(input).cloned().unwrap_or(Action::None)
    }
}

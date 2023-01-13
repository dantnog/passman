use crate::enums::InputMode;
use tui::widgets::ListState;

pub struct Password {
    title: String,
    username: String,
    password: String,
}

pub struct PassMan {
    pub(in super) mode: InputMode,
    pub(in super) list_state: ListState,
    pub(in super) passwords: Vec<Password>,
    pub(in super) new_title: String,
    pub(in super) new_username: String,
    pub(in super) new_password: String,
    pub(in super) search_list: Vec<Password>,
    pub(in super) search_text: String,
}

impl PassMan {
    pub fn new() -> PassMan {
        PassMan {
            mode: InputMode::Normal,
            list_state: ListState::default(),
            passwords: vec![],
            new_title: String::new(),
            new_username: String::new(),
            new_password: String::new(),
            search_list: vec![],
            search_text: String::new(),
        }
    }

    pub fn change_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }

    pub fn clear_inputs(&mut self) {
        self.new_title.clear();
        self.new_username.clear();
        self.new_password.clear();
    }
}
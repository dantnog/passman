use crate::enums::{InputMode, Move};
use tui::widgets::ListState;

#[derive(Clone)]
pub struct Password {
    pub title: String,
    pub username: String,
    pub password: String,
}

impl Password {
    pub fn new(title: String, username: String, password: String) -> Password {
        Password {
            title,
            username,
            password 
        }
    }
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
        self.search_text.clear();
    }

    pub fn save(&mut self) {
        let password = Password::new(
            self.new_title.to_owned(),
            self.new_username.to_owned(),
            self.new_password.to_owned(),
        );
        self.passwords.push(password);
        self.clear_inputs();
        self.change_mode(InputMode::Normal);
    }

    pub fn search(&mut self) {
        if self.search_text == "" { return }
        self.search_list = self.passwords.clone()
            .into_iter()
            .filter(|item| {
                item.title.starts_with(&self.search_text.to_owned())
            })
            .collect();
    }

    pub fn change_list_state(&mut self, movement: Move) {
        let last = self.list_state.selected().unwrap();
        match movement {
            Move::Up => {
                if last == 0 { return };
                self.list_state.select(Some(last - 1));
            },
            Move::Down => {
                self.list_state.select(Some(last + 1));
            },
            _ => {}
        }
    }
}
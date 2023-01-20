use crate::enums::{InputMode, Move};
use tui::widgets::ListState;
use crate::db;

#[derive(Clone)]
pub struct Password {
    pub id: usize,
    pub title: String,
    pub username: String,
    pub password: String,
}

impl Password {
    pub fn new(id: usize, title: String, username: String, password: String) -> Password {
        Password {
            id,
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
        db::insert(&self.new_title, &self.new_username, &self.new_password);

        self.clear_inputs();
        self.change_mode(InputMode::Normal);

        self.fetch();
    }

    pub fn fetch(&mut self) {
        if self.passwords.len() > 0 { return }

        self.passwords = db::fetch().unwrap();
    }

    pub fn delete(&mut self) {
        let index: usize = self.list_state.selected().unwrap();
        let selected_ref: Option<&Password> = match self.mode {
            InputMode::List => self.passwords.get(index),
            InputMode::Search => self.search_list.get(index),
            _ => None
        };

        if index == 0 {
            self.list_state.select(Some(0));
        } else {
            self.list_state.select(Some(index - 1))
        }

        let selected: Password = selected_ref.unwrap().to_owned();

        db::delete(selected.id);
        self.passwords.remove(index);
    }

    pub fn search(&mut self) {
        if self.search_text == "" { return }
        if self.passwords.len() == 0 {
            self.passwords = db::fetch().unwrap();
        }

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
                if last == 0 {
                    return self.list_state.select(Some(self.passwords.len() - 1));
                };
                self.list_state.select(Some(last - 1));
            },
            Move::Down => {
                if last == self.passwords.len() - 1 {
                    return self.list_state.select(Some(0));
                };
                self.list_state.select(Some(last + 1));
            },
            _ => {}
        }
    }
}
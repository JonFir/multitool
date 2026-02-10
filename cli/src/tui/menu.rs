use crossterm::event::KeyCode;

use super::screens::ScreenId;

pub enum MenuAction {
    None,
    Open(ScreenId),
}

pub struct Menu {
    lines: Vec<String>,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            lines: vec![
                "Добро пожаловать в TUI режим".to_string(),
                "Нажмите 1 для Tracker, 2 для LLM, q для выхода".to_string(),
            ],
        }
    }

    pub fn handle_key(&self, code: KeyCode) -> MenuAction {
        match code {
            KeyCode::Char('1') => MenuAction::Open(ScreenId::Tracker),
            KeyCode::Char('2') => MenuAction::Open(ScreenId::Llm),
            _ => MenuAction::None,
        }
    }

    pub fn title(&self) -> &'static str {
        "Menu"
    }

    pub fn input_title(&self) -> &'static str {
        "Меню: 1-Tracker, 2-LLM, q-Выход"
    }

    pub fn output_text(&self) -> String {
        self.lines.join("\n\n")
    }
}

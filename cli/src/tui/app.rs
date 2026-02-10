use std::{io, time::Duration};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use super::{
    menu::{Menu, MenuAction},
    screens::{llm::LlmScreen, tracker::TrackerScreen, Screen, ScreenEvent, ScreenId},
};

enum ActiveView {
    Menu,
    Screen(ScreenId),
}

pub struct App {
    active_view: ActiveView,
    menu: Menu,
    tracker: TrackerScreen,
    llm: LlmScreen,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_view: ActiveView::Menu,
            menu: Menu::new(),
            tracker: TrackerScreen::new(),
            llm: LlmScreen::new(),
        }
    }

    pub async fn run(
        mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> Result<()> {
        loop {
            terminal.draw(|f| self.draw(f))?;

            if !event::poll(Duration::from_millis(100))? {
                continue;
            }

            let Event::Key(key) = event::read()? else {
                continue;
            };

            if key.kind != KeyEventKind::Press {
                continue;
            }

            if self.handle_global_key(key.code) {
                return Ok(());
            }

            self.handle_key(key.code).await;
        }
    }

    fn handle_global_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Char('q') => true,
            KeyCode::Esc => {
                self.active_view = ActiveView::Menu;
                false
            }
            _ => false,
        }
    }

    async fn handle_key(&mut self, code: KeyCode) {
        match self.active_view {
            ActiveView::Menu => match self.menu.handle_key(code) {
                MenuAction::None => {}
                MenuAction::Open(screen_id) => self.active_view = ActiveView::Screen(screen_id),
            },
            ActiveView::Screen(screen_id) => {
                let event = self.screen_mut(screen_id).handle_key(code);
                if let ScreenEvent::Submit(input) = event {
                    let command = self.screen(screen_id).command_preview(&input);
                    self.screen_mut(screen_id).push_output(command);
                    let response = self.screen_mut(screen_id).execute(input).await;
                    self.screen_mut(screen_id).push_output(response);
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(frame.area());

        let (title, output, input_title, input_text) = match self.active_view {
            ActiveView::Menu => (
                self.menu.title(),
                self.menu.output_text(),
                self.menu.input_title(),
                "".to_string(),
            ),
            ActiveView::Screen(screen_id) => {
                let screen = self.screen(screen_id);
                (
                    screen.title(),
                    screen.output_text(),
                    screen.input_title(),
                    screen.input_text().to_string(),
                )
            }
        };

        let header = Paragraph::new(format!("you tui | Режим: {title} | q: выход | Esc: меню"))
            .block(Block::default().borders(Borders::ALL).title("Статус"));

        let content = Paragraph::new(output)
            .block(Block::default().borders(Borders::ALL).title("Вывод"))
            .wrap(Wrap { trim: false });

        let input = Paragraph::new(input_text)
            .block(Block::default().borders(Borders::ALL).title(input_title))
            .wrap(Wrap { trim: true });

        frame.render_widget(header, chunks[0]);
        frame.render_widget(content, chunks[1]);
        frame.render_widget(input, chunks[2]);
    }

    fn screen(&self, id: ScreenId) -> &dyn Screen {
        match id {
            ScreenId::Tracker => &self.tracker,
            ScreenId::Llm => &self.llm,
        }
    }

    fn screen_mut(&mut self, id: ScreenId) -> &mut dyn Screen {
        match id {
            ScreenId::Tracker => &mut self.tracker,
            ScreenId::Llm => &mut self.llm,
        }
    }
}

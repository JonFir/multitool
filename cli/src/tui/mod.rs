use std::io;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use llm_lib::{LlmClient, LlmClientTrait, LlmConfig};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use tracker_lib::{task::format_issue_output, TrackerClient};

#[derive(Clone, Copy)]
enum Mode {
    Menu,
    Tracker,
    Llm,
}

struct App {
    mode: Mode,
    input: String,
    output: Vec<String>,
}

impl App {
    fn new() -> Self {
        Self {
            mode: Mode::Menu,
            input: String::new(),
            output: vec![
                "Добро пожаловать в TUI режим".to_string(),
                "Нажмите 1 для Tracker, 2 для LLM, q для выхода".to_string(),
            ],
        }
    }

    fn mode_name(&self) -> &'static str {
        match self.mode {
            Mode::Menu => "Menu",
            Mode::Tracker => "Tracker",
            Mode::Llm => "LLM",
        }
    }

    fn input_title(&self) -> &'static str {
        match self.mode {
            Mode::Menu => "Меню: 1-Tracker, 2-LLM, q-Выход",
            Mode::Tracker => "Tracker: введите ключ задачи и нажмите Enter",
            Mode::Llm => "LLM: введите промпт и нажмите Enter",
        }
    }

    fn push_output(&mut self, text: impl Into<String>) {
        self.output.push(text.into());
        if self.output.len() > 200 {
            let drain_count = self.output.len().saturating_sub(200);
            self.output.drain(0..drain_count);
        }
    }
}

pub async fn run_tui() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| draw_ui(f, &app))?;

        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.kind != KeyEventKind::Press {
            continue;
        }

        match key.code {
            KeyCode::Char('q') => return Ok(()),
            KeyCode::Esc => {
                app.mode = Mode::Menu;
                app.input.clear();
                app.push_output("Возврат в меню");
            }
            KeyCode::Char('1') if matches!(app.mode, Mode::Menu) => {
                app.mode = Mode::Tracker;
                app.input.clear();
                app.push_output("Режим Tracker активирован");
            }
            KeyCode::Char('2') if matches!(app.mode, Mode::Menu) => {
                app.mode = Mode::Llm;
                app.input.clear();
                app.push_output("Режим LLM активирован");
            }
            KeyCode::Backspace if !matches!(app.mode, Mode::Menu) => {
                app.input.pop();
            }
            KeyCode::Char(c) if !matches!(app.mode, Mode::Menu) => {
                app.input.push(c);
            }
            KeyCode::Enter if !matches!(app.mode, Mode::Menu) => {
                execute_action(&mut app).await;
            }
            _ => {}
        }
    }
}

async fn execute_action(app: &mut App) {
    let input = app.input.trim().to_string();
    if input.is_empty() {
        return;
    }

    match app.mode {
        Mode::Tracker => {
            app.push_output(format!("> tracker issue {input}"));
            match fetch_tracker_issue(&input).await {
                Ok(output) => app.push_output(output),
                Err(err) => app.push_output(format!("Ошибка Tracker: {err}")),
            }
        }
        Mode::Llm => {
            app.push_output(format!("> llm ask {input}"));
            match ask_llm(&input).await {
                Ok(output) => app.push_output(output),
                Err(err) => app.push_output(format!("Ошибка LLM: {err}")),
            }
        }
        Mode::Menu => {}
    }

    app.input.clear();
}

async fn fetch_tracker_issue(issue_id: &str) -> Result<String> {
    let client = TrackerClient::from_env()?;
    let issue = client.get_issue(issue_id, None).await?;
    Ok(format_issue_output(&issue))
}

async fn ask_llm(prompt: &str) -> Result<String> {
    let config = LlmConfig::new("anthropic/claude-3.5-sonnet".to_string())?;
    let client = LlmClient::new(config)?;
    let response = client.complete(prompt.to_string()).await?;
    Ok(response)
}

fn draw_ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new(format!(
        "you tui | Режим: {} | q: выход | Esc: меню",
        app.mode_name()
    ))
    .block(Block::default().borders(Borders::ALL).title("Статус"));

    let content = Paragraph::new(app.output.join("\n\n"))
        .block(Block::default().borders(Borders::ALL).title("Вывод"))
        .wrap(Wrap { trim: false });

    let input = Paragraph::new(app.input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.input_title()),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(header, chunks[0]);
    frame.render_widget(content, chunks[1]);
    frame.render_widget(input, chunks[2]);
}

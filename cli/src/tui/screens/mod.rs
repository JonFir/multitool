use std::{future::Future, pin::Pin};

use crossterm::event::KeyCode;

pub mod llm;
pub mod tracker;

#[derive(Clone, Copy)]
pub enum ScreenId {
    Tracker,
    Llm,
}

pub enum ScreenEvent {
    None,
    Submit(String),
}

pub trait Screen {
    fn title(&self) -> &'static str;
    fn input_title(&self) -> &'static str;
    fn input_text(&self) -> &str;
    fn output_text(&self) -> String;
    fn handle_key(&mut self, key: KeyCode) -> ScreenEvent;
    fn push_output(&mut self, text: String);
    fn command_preview(&self, input: &str) -> String;
    fn execute<'a>(&'a mut self, input: String) -> Pin<Box<dyn Future<Output = String> + 'a>>;
}

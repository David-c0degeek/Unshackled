//! The event loop and input handling.
//!
//! The loop is driven by an iterator of [`AppInput`] so it runs deterministically
//! under a scripted source in tests; the real CLI feeds it crossterm events and a
//! mapped runtime-event stream.

use ratatui::backend::Backend;
use ratatui::Terminal;

use crate::render::render;
use crate::state::{AppState, Mode, Profile, UiEvent};

/// A terminal key press, decoupled from any specific terminal backend. The CLI's
/// terminal driver maps crossterm key events into these.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Enter,
    Backspace,
    Esc,
    Up,
    Down,
    CtrlC,
}

/// One input to the loop: a mapped runtime event or a key press.
#[derive(Debug, Clone)]
pub enum AppInput {
    Ui(UiEvent),
    Key(Key),
}

/// A parsed slash command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlashAction {
    SetMode(Mode),
    SetProfile(Profile),
    ToggleThinking,
    Quit,
    Unknown(String),
}

/// Parse a slash command from an input line.
#[must_use]
pub fn parse_slash(line: &str) -> Option<SlashAction> {
    let command = line.trim().strip_prefix('/')?.trim();
    Some(match command {
        "agent" => SlashAction::SetMode(Mode::Agent),
        "harness" => SlashAction::SetMode(Mode::Harness),
        "default" => SlashAction::SetProfile(Profile::Default),
        "relaxed" => SlashAction::SetProfile(Profile::Relaxed),
        "bypass" => SlashAction::SetProfile(Profile::Bypass),
        "think" | "thinking" => SlashAction::ToggleThinking,
        "quit" | "q" => SlashAction::Quit,
        other => SlashAction::Unknown(other.to_string()),
    })
}

/// Apply one input to the state.
pub fn handle_input(state: &mut AppState, input: AppInput) {
    match input {
        AppInput::Ui(event) => state.apply(event),
        AppInput::Key(key) => handle_key(state, key),
    }
}

fn handle_key(state: &mut AppState, key: Key) {
    // Modal dialogs capture keys first.
    if state.approval.is_some() {
        if matches!(key, Key::Char('y') | Key::Char('n') | Key::Esc) {
            state.approval = None;
        }
        return;
    }
    if let Some(picker) = state.picker.as_mut() {
        match key {
            Key::Up => picker.selected = picker.selected.saturating_sub(1),
            Key::Down => {
                if picker.selected + 1 < picker.options.len() {
                    picker.selected += 1;
                }
            }
            Key::Enter | Key::Esc => state.picker = None,
            _ => {}
        }
        return;
    }

    match key {
        Key::Esc | Key::CtrlC => state.should_quit = true,
        Key::Enter => submit_input(state),
        Key::Backspace => {
            state.input.pop();
        }
        Key::Char(c) => state.input.push(c),
        _ => {}
    }
}

fn submit_input(state: &mut AppState) {
    let line = std::mem::take(&mut state.input);
    if line.trim().is_empty() {
        return;
    }
    if let Some(action) = parse_slash(&line) {
        apply_slash(state, action);
    } else {
        state.apply(UiEvent::UserMessage(line));
    }
}

fn apply_slash(state: &mut AppState, action: SlashAction) {
    match action {
        SlashAction::SetMode(mode) => state.mode = mode,
        SlashAction::SetProfile(profile) => state.profile = profile,
        SlashAction::ToggleThinking => state.thinking.visible = !state.thinking.visible,
        SlashAction::Quit => state.should_quit = true,
        SlashAction::Unknown(_) => {}
    }
}

/// Run the loop against a backend and a scripted input source, drawing after each
/// input until the state requests quit or the source is exhausted.
///
/// # Errors
/// Returns any drawing error from the terminal backend.
pub fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    state: &mut AppState,
    inputs: impl IntoIterator<Item = AppInput>,
) -> std::io::Result<()> {
    terminal.draw(|frame| render(frame, state))?;
    for input in inputs {
        handle_input(state, input);
        terminal.draw(|frame| render(frame, state))?;
        if state.should_quit {
            break;
        }
    }
    Ok(())
}

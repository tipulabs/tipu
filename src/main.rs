#[macro_use]
extern crate lazy_static;

mod components;
mod state;
#[allow(dead_code)]
mod util;

use crate::components::{
    app::App, git_window::GitWindow, script_bar::ScriptBar, shared_window::SharedWindow,
    terminal::Terminal as TerminalComponent,
};
use crate::state::{ApplicationFocus, ApplicationState};
use crate::util::event::{Event, Events};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    // Application State
    let mut app_state = ApplicationState::default();

    // Main loops
    loop {
        terminal.draw(|f| {
            // drawing the global borders
            let window_size = f.size();
            App::default()
                .title("Tipu")
                .size(window_size)
                .is_focused(false)
                .render(f);

            // dividing chunks
            let main_chunk = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
                .split(window_size);

            ScriptBar::default()
                .size(main_chunk[0])
                .title("ScriptBar [3]")
                .is_focused(app_state.current_focus_position == ApplicationFocus::ScriptBar)
                .render(f);

            // lower_chunk
            let lower_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
                .split(main_chunk[1]);

            // rendering terminal
            TerminalComponent::default()
                .size(lower_chunk[0])
                .title("Terminal [1]")
                .is_focused(app_state.current_focus_position == ApplicationFocus::Terminal)
                .render(f);

            // lower right chunk
            let lower_right_chunk = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(lower_chunk[1]);
            // chat window
            SharedWindow::default()
                .size(lower_right_chunk[0])
                .title("Shared Window [2]")
                .is_focused(app_state.current_focus_position == ApplicationFocus::Chat)
                .render(f);

            // git window
            GitWindow::default()
                .size(lower_right_chunk[1])
                .title("Git Window [4]")
                .is_focused(app_state.current_focus_position == ApplicationFocus::GitChat)
                .render(f);
        })?;

        // handling keyboard inputs
        if let Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') => break,
                Key::Char('2') => app_state.update_focus_state(ApplicationFocus::Chat),
                Key::Char('3') => app_state.update_focus_state(ApplicationFocus::ScriptBar),
                Key::Char('1') => app_state.update_focus_state(ApplicationFocus::Terminal),
                Key::Char('4') => app_state.update_focus_state(ApplicationFocus::GitChat),
                _ => {}
            }
        }
    }
    Ok(())
}
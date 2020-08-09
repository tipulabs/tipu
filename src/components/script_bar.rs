use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame,
};

use std::{fs::File, io::Read};

use serde_json::Value;

#[derive(Debug, Clone, Copy)]
struct Script<'a> {
    name: &'a str,
    script: &'a str,
}

impl<'a> Default for Script<'a> {
    fn default() -> Self {
        Script {
            name: "",
            script: "echo NO SCRIPT FOUND!!",
        }
    }
}

impl<'a> Script<'a> {
    fn name(mut self, name: &'a str) -> Script<'a> {
        self.name = name;
        self
    }
    fn script(mut self, script: &'a str) -> Script<'a> {
        self.script = script;
        self
    }
}

pub struct ScriptBar<'a> {
    size: Rect,
    title: &'a str,
    is_focused: bool,
}

impl<'a> Default for ScriptBar<'a> {
    fn default() -> Self {
        ScriptBar {
            title: "Script Bar",
            size: Rect::default(),
            is_focused: false,
        }
    }
}

impl<'a> ScriptBar<'a> {
    pub fn is_focused(mut self, is_focused: bool) -> ScriptBar<'a> {
        self.is_focused = is_focused;
        self
    }

    pub fn title(mut self, title: &'a str) -> ScriptBar<'a> {
        self.title = title;
        self
    }
    pub fn size(mut self, size: Rect) -> ScriptBar<'a> {
        self.size = size;
        self
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>) -> &ScriptBar<'a> {
        // main
        let mut block = Block::default()
            .borders(Borders::ALL)
            .title(self.title)
            .border_type(BorderType::Plain);

        if self.is_focused {
            block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(self.title, Style::default().fg(Color::Red)))
                .border_type(BorderType::Thick);
        }
        f.render_widget(block, self.size);

        // getting packages
        let mut file = File::open("package.json").unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();
        let foo: Value = serde_json::from_str(&buff).unwrap();

        let mut scripts: Vec<Script> = vec![];

        for (name, obj) in foo["scripts"].as_object().unwrap().iter() {
            scripts.push(Script::default().name(name).script(obj.as_str().unwrap()));
        }
        // buttons
        // generating constriants
        let mut button_constraints: Vec<Constraint> = vec![];

        let max_width = scripts.len();
        for _ in 0..max_width {
            button_constraints.push(Constraint::Percentage(
                (1.0 / max_width as f64 * 100.0) as u16,
            ))
        }
        // 1. Layout
        let button_container = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(button_constraints.as_ref())
            .split(self.size);

        for idx in 0..max_width {
            let button = Block::default()
                .borders(Borders::ALL)
                .title(scripts[idx].name)
                .border_type(BorderType::Plain);
            f.render_widget(button, button_container[idx])
        }

        self
    }
}

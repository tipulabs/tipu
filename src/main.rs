use std::fs::File;
use std::io::Read;
// use std::process::Command;

use tuikit::prelude::*;

use serde_json::{Value};

struct Model(String);

impl Draw for Model {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        let (width, height) = canvas.size()?;
        let message_width = self.0.len();
        let left = (width - message_width) / 2;
        let top = height / 2;
        let _ = canvas.print(top, left, &self.0);
        Ok(())
    }
}

impl Widget for Model{}

fn main() {
    let mut script_to_run = "";
    let mut window_on_focus = "bash";
    // TODO: Handle panic when package.json
    let mut file = File::open("package.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
 
    let foo: Value = serde_json::from_str(&buff).unwrap();
    println!("Scripts: {}", foo["scripts"]);

    for (name, obj) in foo["scripts"].as_object().unwrap().iter() {
        println!("{} is {:?}", name, obj);
    }

    let term = Term::with_height(TermHeight::Percent(70)).unwrap();
    let model = Model("middle!".to_string());

    while let Ok(ev) = term.poll_event() {
        let _ = term.clear();
        let _ = term.print(0, 0, "press arrow key to move the text, (q) to quit");

        println!("Current Focused Window: {}", window_on_focus);

        match ev {
            Event::Key(Key::ESC) | Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Ctrl('w')) => script_to_run = "yarn start",
            Event::Key(Key::Ctrl('e')) => script_to_run = "yarn build",
            Event::Key(Key::Ctrl('r')) => script_to_run = "yarn test",
            Event::Key(Key::Up) => window_on_focus = "script", // scripts
            Event::Key(Key::Left) => window_on_focus = "bash", // bash
            Event::Key(Key::Down) => window_on_focus = "chat", // chat
            Event::Key(Key::Right) => window_on_focus = "session", // shared session
            _ => {}
        }

        // TODO: Find a way to use the selected script inside the central bash
        if script_to_run != "" {
            println!("Shortcut invoked: {}", script_to_run);
            // Command::new(script_to_run).spawn().expect("failed to execute process")
        }

        // TODO: Hide the topmost Window in case package.json is not found
        // TODO: Intelligently show the buttons based on the number of scripts from package.json
        let vsplit = VSplit::default()
            .split(
                HSplit::default()
                    .basis(Size::Percent(70))
                    .split(Win::new(&model).border(true).basis(Size::Percent(30)))
                    .split(Win::new(&model).border(true).basis(Size::Percent(30)))
                    .split(Win::new(&model).border(true).basis(Size::Percent(30)))
                    .split(Win::new(&model).border(true).basis(Size::Percent(30)))
            )
            .split(
                HSplit::default()
                    .split(Win::new(&model).border(true))
                    .split(
                        VSplit::default()
                            .basis(Size::Percent(30))
                            .split(Win::new(&model).border(true).basis(Size::Percent(30)))
                            .split(Win::new(&model).border(true).basis(Size::Percent(30)))
                    ),
            );
        let _ = term.draw(&vsplit);
        let _ = term.present();
    }
}
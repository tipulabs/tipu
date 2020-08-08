use std::fs::File;
use std::io::Read;

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
    // TODO: Handle panic when package.json
    let mut file = File::open("package.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
 
    let foo: Value = serde_json::from_str(&buff).unwrap();
    println!("Scripts: {}", foo["scripts"]);

    let term = Term::with_height(TermHeight::Percent(70)).unwrap();
    let model = Model("middle!".to_string());

    while let Ok(ev) = term.poll_event() {
        if let Event::Key(Key::Char('q')) = ev {
            break;
        }
        let _ = term.print(0, 0, "press 'q' to exit");

        // TODO: Hide the topmost Window in case package.json is not found
        let vsplit = VSplit::default()
            .split(Win::new(&model).border(true))
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
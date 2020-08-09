use std::io::{self, Write};
use std::process::{self, Command};

use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::util::{parse};

pub struct Terminal<'a> {
    size: Rect,
    title: &'a str,
    is_focused: bool,
}

impl<'a> Default for Terminal<'a> {
    fn default() -> Self {
        Terminal {
            title: "Terminal",
            size: Rect::default(),
            is_focused: false,
        }
    }
}

impl<'a> Terminal<'a> {
    pub fn is_focused(mut self, is_focused: bool) -> Terminal<'a> {
        self.is_focused = is_focused;
        self
    }

    pub fn title(mut self, title: &'a str) -> Terminal<'a> {
        self.title = title;
        self
    }
    pub fn size(mut self, size: Rect) -> Terminal<'a> {
        self.size = size;
        self
    }

    pub fn render<B: Backend>(self, f: &mut Frame<B>) -> Terminal<'a> {
        let mut block = Block::default()
            .borders(Borders::ALL)
            .title(self.title)
            .border_type(BorderType::Plain);

        // let mut input = String::new();
        // print!("$ ");
        // io::stdout().flush().unwrap();
        // match io::stdin().read_line(&mut input) {
        //     Ok(0) => break,
        //     Ok(_) => if let Some((path, args)) = eval_input(input.trim()) {
        //         exec_input(&path, args.iter().map(AsRef::as_ref).collect());
        //     },
        //     Err(_) => process::exit(1),
        // }
        // input.clear();
        // println!("Bye!");

        if self.is_focused {
            block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(self.title, Style::default().fg(Color::Red)))
                .border_type(BorderType::Thick);
        }
        f.render_widget(block, self.size);

        self
    }
}

fn eval_input(input: &str) -> Option<(String, Vec<String>)> {
    match parse::parse(input) {
        Ok(mut expr) => {
            let args = expr
                .split_off(1)
                .iter()
                .map(|x| x.unwrap())
                .collect();
            let path = expr[0].unwrap();

            Some((path, args))
        },
        Err(msg) => {
            eprintln!("error: {}", msg);
            None
        },
    }
}


fn exec_input(path: &str, args: Vec<&str>) {
    match Command::new(path).args(args).spawn() {
        Ok(mut child) => if let Ok(exit_status) = child.wait() {
            println!("process exited with code {}", exit_status.code().unwrap_or(0));
        },
        Err(e) => {
            println!("{}", e);
        },
    }
}

use std::thread;
use std::fs::File;
use std::time::Duration;
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::io::{self, ErrorKind, Read, Write};

use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub struct SharedWindow<'a> {
    size: Rect,
    title: &'a str,
    is_focused: bool,
}

impl<'a> Default for SharedWindow<'a> {
    fn default() -> Self {
        SharedWindow {
            title: "SharedWindow",
            size: Rect::default(),
            is_focused: false,
        }
    }
}

const MSG_SIZE: usize = 32;
const LOCAL: &str = "https://protected-everglades-84717.herokuapp.com:80";

impl<'a> SharedWindow<'a> {
    pub fn is_focused(mut self, is_focused: bool) -> SharedWindow<'a> {
        self.is_focused = is_focused;
        self
    }

    pub fn title(mut self, title: &'a str) -> SharedWindow<'a> {
        self.title = title;
        self
    }
    pub fn size(mut self, size: Rect) -> SharedWindow<'a> {
        self.size = size;
        self
    }

    pub fn render<B: Backend>(self, f: &mut Frame<B>) -> SharedWindow<'a> {
        let mut block = Block::default()
            .borders(Borders::ALL)
            .title(self.title)
            .border_type(BorderType::Plain);

            // let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
            // client
            //     .set_nonblocking(true)
            //     .expect("Failed to initiate non-blocking");
        
            // let (tx, rx) = mpsc::channel::<String>();
        
            // thread::spawn(move || loop {
            //     let mut buff = vec![0; MSG_SIZE];
            //     match client.read_exact(&mut buff) {
            //         Ok(_) => {
            //             let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
            //             println!("Received message: {:?}", String::from_utf8(msg).unwrap());
            //         }
            //         Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            //         Err(_) => {
            //             println!("Connection with server lost.");
            //             break;
            //         }
            //     }
        
            //     match rx.try_recv() {
            //         Ok(msg) => {
            //             let mut buffer = msg.clone().into_bytes();
            //             buffer.resize(MSG_SIZE, 0);
            //             client.write_all(&buffer).expect("Writing to socket failed");
            //             println!("Sent message: {:?}", msg);
            //         }
            //         Err(TryRecvError::Empty) => (),
            //         Err(TryRecvError::Disconnected) => break,
            //     }
        
            //     thread_sleep();
            // });
        
            // println!("Write a Message:");
            // loop {
            //     let mut buff = String::new();
            //     io::stdin()
            //         .read_line(&mut buff)
            //         .expect("Reading from stdin failed.");
            //     let msg = buff.trim().to_string();
            //     if msg == ":q" || tx.send(msg).is_err() {
            //         break;
            //     }
            // }

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

fn thread_sleep() {
    thread::sleep(Duration::from_millis(120));
}
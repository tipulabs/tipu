use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub struct GitWindow<'a> {
    size: Rect,
    title: &'a str,
    is_focused: bool,
}

impl<'a> Default for GitWindow<'a> {
    fn default() -> Self {
        GitWindow {
            title: "GitWindow",
            size: Rect::default(),
            is_focused: false,
        }
    }
}

impl<'a> GitWindow<'a> {
    pub fn is_focused(mut self, is_focused: bool) -> GitWindow<'a> {
        self.is_focused = is_focused;
        self
    }

    pub fn title(mut self, title: &'a str) -> GitWindow<'a> {
        self.title = title;
        self
    }
    pub fn size(mut self, size: Rect) -> GitWindow<'a> {
        self.size = size;
        self
    }

    pub fn render<B: Backend>(self, f: &mut Frame<B>) -> GitWindow<'a> {
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

        self
    }
}

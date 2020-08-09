pub struct App<'a> {
    size: Rect,
    title: &'a str,
    is_focused: bool,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        App {
            title: "",
            size: Rect::default(),
            is_focused: false,
        }
    }
}

impl<'a> App<'a> {
    pub fn is_focused(mut self, is_focused: bool) -> App<'a> {
        self.is_focused = is_focused;
        self
    }

    pub fn title(mut self, title: &'a str) -> App<'a> {
        self.title = title;
        self
    }
    pub fn size(mut self, size: Rect) -> App<'a> {
        self.size = size;
        self
    }

    pub fn render<B: Backend>(self, f: &mut Frame<B>) -> App<'a> {
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

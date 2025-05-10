// Ratatui App
use color_eyre::Result;
use ratatui::{
    style::Stylize, text::Line,
    widgets::{Block, Paragraph},
    DefaultTerminal,
    Frame,
};

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();
        let text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`ratatui::crossterm::event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match ratatui::crossterm::event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            ratatui::crossterm::event::Event::Key(key) if key.kind == ratatui::crossterm::event::KeyEventKind::Press => self.on_key_event(key),
            ratatui::crossterm::event::Event::Mouse(_) => {}
            ratatui::crossterm::event::Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: ratatui::crossterm::event::KeyEvent) {
        match (key.modifiers, key.code) {
            (_, ratatui::crossterm::event::KeyCode::Esc | ratatui::crossterm::event::KeyCode::Char('q'))
            | (ratatui::crossterm::event::KeyModifiers::CONTROL, ratatui::crossterm::event::KeyCode::Char('c') | ratatui::crossterm::event::KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}

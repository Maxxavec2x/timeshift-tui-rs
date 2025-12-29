use crate::app::App;
use crate::app::InputMode;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Paragraph};
use ratatui::{
    buffer::Buffer,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Widget,
};

pub struct CursorPosition {
    pub x: u16,
    pub y: u16,
}

impl App {
    /// This method create a popup to create a snapshot. It allow for a state where you are editing
    /// in the popup, and a state where you can exit editing without closing the windows BUT i do
    /// not use the former, because for now I only need a comment. I close the windows whenever we
    /// press enter or esc to fix this. Later, I want to add the possibility to create snapshots
    /// periodically, and this will require another input field.
    pub fn render_creation_popup(&self, area: Rect, buf: &mut Buffer) -> Option<CursorPosition> {
        let border_style = match self.input_mode {
            InputMode::Normal => Style::default().fg(Color::White),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        };

        let main_block = &Block::bordered()
            .title(" Creating a Snapshot ".bold())
            .border_style(border_style);

        main_block.render(area, buf);

        // Zone fait par claude parce que flm de faire du front là
        // Zone intérieure
        let inner_area = main_block.inner(area);

        // Layout vertical : instructions + input + aide
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2), // Instructions
                Constraint::Length(3), // Input
                Constraint::Min(1),    // Espace
                Constraint::Length(1), // Aide en bas
            ])
            .split(inner_area);

        // Instructions en haut
        let instructions = Paragraph::new("Enter a comment for this snapshot:");
        instructions.render(chunks[0], buf);

        // Champ d'input
        let width = chunks[1].width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);

        let input_style = match self.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        };

        let input_block = Block::bordered().title("Comment").border_style(input_style);

        let input_inner = input_block.inner(chunks[1]);
        input_block.render(chunks[1], buf);

        let input = Paragraph::new(self.input.value())
            .style(input_style)
            .scroll((0, scroll as u16));

        input.render(input_inner, buf);

        let help_text = match self.input_mode {
            InputMode::Editing => Line::from(vec![
                Span::styled("Esc", Style::default().fg(Color::Yellow).bold()),
                Span::styled(" to finish editing | ", Style::default().fg(Color::Gray)),
                Span::styled("Enter", Style::default().fg(Color::Green).bold()),
                Span::styled(" to create", Style::default().fg(Color::Gray)),
            ]),
            InputMode::Normal => Line::from(vec![
                Span::styled("Enter", Style::default().fg(Color::Green).bold()),
                Span::styled(" to create | ", Style::default().fg(Color::Gray)),
                Span::styled("Esc", Style::default().fg(Color::Red).bold()),
                Span::styled(" to cancel", Style::default().fg(Color::Gray)),
            ]),
        };

        let help = Paragraph::new(help_text);
        help.render(chunks[3], buf);

        // Retourner la position du curseur si en mode édition
        if self.input_mode == InputMode::Editing {
            let x = self.input.visual_cursor().max(scroll) - scroll;
            return Some(CursorPosition {
                x: input_inner.x + x as u16,
                y: input_inner.y,
            });
        }

        None
    }
}

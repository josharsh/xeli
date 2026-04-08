use crate::ui::theme::{get_theme_colors, ThemeColors};
use crate::app::Theme;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;
use ratatui::Terminal;
use std::path::PathBuf;
use std::time::Duration;

struct PickerState {
    all_files: Vec<PathBuf>,
    query: String,
    cursor: usize,
    colors: ThemeColors,
}

impl PickerState {
    fn filtered(&self) -> Vec<&PathBuf> {
        if self.query.is_empty() {
            return self.all_files.iter().collect();
        }
        let q = self.query.to_lowercase();
        self.all_files
            .iter()
            .filter(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| fuzzy_contains(&n.to_lowercase(), &q))
                    .unwrap_or(false)
            })
            .collect()
    }
}

/// Simple subsequence match — every char of needle appears in haystack in order.
fn fuzzy_contains(haystack: &str, needle: &str) -> bool {
    let mut hay = haystack.chars();
    'outer: for nc in needle.chars() {
        for hc in hay.by_ref() {
            if hc == nc {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

/// Run a small TUI to pick one of the listed data files. Returns the chosen
/// path, or Ok(None) if the user cancels with Esc/q.
pub fn pick(
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    files: Vec<PathBuf>,
    theme: &Theme,
) -> Result<Option<PathBuf>> {
    let mut state = PickerState {
        all_files: files,
        query: String::new(),
        cursor: 0,
        colors: get_theme_colors(theme),
    };

    loop {
        terminal.draw(|f| draw(f, &state))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => return Ok(None),
                    (_, KeyCode::Esc) => return Ok(None),
                    (_, KeyCode::Enter) => {
                        let filtered = state.filtered();
                        if let Some(pick) = filtered.get(state.cursor) {
                            return Ok(Some((*pick).clone()));
                        }
                    }
                    (_, KeyCode::Up) => {
                        if state.cursor > 0 {
                            state.cursor -= 1;
                        }
                    }
                    (_, KeyCode::Down) => {
                        let len = state.filtered().len();
                        if state.cursor + 1 < len {
                            state.cursor += 1;
                        }
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('p')) => {
                        if state.cursor > 0 {
                            state.cursor -= 1;
                        }
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('n')) => {
                        let len = state.filtered().len();
                        if state.cursor + 1 < len {
                            state.cursor += 1;
                        }
                    }
                    (_, KeyCode::Backspace) => {
                        state.query.pop();
                        state.cursor = 0;
                    }
                    (_, KeyCode::Char(c)) => {
                        state.query.push(c);
                        state.cursor = 0;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn draw(f: &mut Frame, state: &PickerState) {
    let area = centered_rect(70, 70, f.area());

    let block = Block::default()
        .title(" Pick a data file ")
        .title_style(Style::default().fg(state.colors.accent).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(state.colors.border))
        .style(Style::default().bg(state.colors.bg));

    f.render_widget(ratatui::widgets::Clear, area);
    f.render_widget(block.clone(), area);

    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // search
            Constraint::Length(1), // separator
            Constraint::Min(1),    // list
            Constraint::Length(1), // hint footer
        ])
        .split(inner);

    // Search bar
    let search = Line::from(vec![
        Span::styled(" Find: ", Style::default().fg(state.colors.muted)),
        Span::styled(&state.query, Style::default().fg(state.colors.fg)),
        Span::styled(
            "_",
            Style::default()
                .fg(state.colors.accent)
                .add_modifier(Modifier::SLOW_BLINK),
        ),
    ]);
    f.render_widget(
        Paragraph::new(search).style(Style::default().bg(state.colors.bg)),
        chunks[0],
    );

    // Separator
    let sep = Line::from(Span::styled(
        "─".repeat(inner.width as usize),
        Style::default().fg(state.colors.border),
    ));
    f.render_widget(Paragraph::new(sep), chunks[1]);

    // File list
    let filtered = state.filtered();
    let max_items = chunks[2].height as usize;
    let mut lines: Vec<Line> = Vec::new();

    if filtered.is_empty() {
        let msg = if state.all_files.is_empty() {
            "No supported data files in this directory.  Try: xeli path/to/file.csv"
        } else {
            "No matches"
        };
        lines.push(Line::from(Span::styled(
            format!("  {}", msg),
            Style::default().fg(state.colors.muted),
        )));
    } else {
        let start = state.cursor.saturating_sub(max_items.saturating_sub(1));
        for (offset, path) in filtered.iter().skip(start).take(max_items).enumerate() {
            let idx = start + offset;
            let is_cursor = idx == state.cursor;
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("?")
                .to_string();
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_uppercase())
                .unwrap_or_else(|| "?".to_string());
            let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
            let size_str = human_size(size);

            let style = if is_cursor {
                Style::default()
                    .fg(state.colors.cursor_fg)
                    .bg(state.colors.cursor_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(state.colors.fg)
            };

            let prefix = if is_cursor { " > " } else { "   " };
            lines.push(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(format!("[{:<4}] ", ext), Style::default().fg(state.colors.muted)),
                Span::styled(name, style),
                Span::styled(format!("   {}", size_str), Style::default().fg(state.colors.muted)),
            ]));
        }
    }

    f.render_widget(Paragraph::new(lines), chunks[2]);

    // Footer hints
    let footer = Line::from(Span::styled(
        " Type to filter · ↑↓ select · Enter:open · Esc:quit ",
        Style::default().fg(state.colors.muted),
    ));
    f.render_widget(Paragraph::new(footer), chunks[3]);
}

fn human_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

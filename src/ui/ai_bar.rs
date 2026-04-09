use crate::app::{AiKeyStage, App, ComputedColumnStage};
use crate::ui::theme::ThemeColors;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn render_search(f: &mut Frame, area: Rect, app: &App, colors: &ThemeColors) {
    let match_info = if !app.search_matches.is_empty() {
        format!(
            " ({}/{})",
            app.search_match_idx + 1,
            app.search_matches.len()
        )
    } else if !app.search_query.is_empty() {
        " (no matches)".to_string()
    } else {
        String::new()
    };

    let line = Line::from(vec![
        Span::styled(
            " / ",
            Style::default()
                .fg(colors.yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(&app.search_query, Style::default().fg(colors.fg)),
        Span::styled(
            &match_info,
            Style::default().fg(colors.muted),
        ),
        Span::styled("_", Style::default().fg(colors.accent).add_modifier(Modifier::SLOW_BLINK)),
    ]);

    let paragraph = Paragraph::new(line).style(Style::default().bg(colors.bg));
    f.render_widget(paragraph, area);
}

pub fn render_ai(f: &mut Frame, area: Rect, app: &App, colors: &ThemeColors) {
    let line = if app.ai_loading {
        Line::from(vec![
            Span::styled(
                " AI ",
                Style::default()
                    .fg(colors.bg)
                    .bg(colors.purple)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" thinking...", Style::default().fg(colors.purple)),
        ])
    } else if !app.ai_response.is_empty() {
        Line::from(vec![
            Span::styled(
                " AI ",
                Style::default()
                    .fg(colors.bg)
                    .bg(colors.purple)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" {} ", app.ai_response),
                Style::default().fg(colors.fg),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled(
                " AI ",
                Style::default()
                    .fg(colors.bg)
                    .bg(colors.purple)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ", Style::default().fg(colors.fg)),
            Span::styled(&app.input_buffer, Style::default().fg(colors.fg)),
            Span::styled("_", Style::default().fg(colors.accent).add_modifier(Modifier::SLOW_BLINK)),
        ])
    };

    let paragraph = Paragraph::new(line).style(Style::default().bg(colors.bg));
    f.render_widget(paragraph, area);
}

pub fn render_sql(f: &mut Frame, area: Rect, app: &App, colors: &ThemeColors) {
    let line = Line::from(vec![
        Span::styled(
            " SQL ",
            Style::default()
                .fg(colors.bg)
                .bg(colors.cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" ", Style::default().fg(colors.fg)),
        Span::styled(&app.sql_input, Style::default().fg(colors.fg)),
        Span::styled("_", Style::default().fg(colors.accent).add_modifier(Modifier::SLOW_BLINK)),
    ]);

    let paragraph = Paragraph::new(line).style(Style::default().bg(colors.bg));
    f.render_widget(paragraph, area);
}

pub fn render_formula(f: &mut Frame, area: Rect, app: &App, colors: &ThemeColors) {
    let line = Line::from(vec![
        Span::styled(
            " = ",
            Style::default()
                .fg(colors.bg)
                .bg(colors.green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" ", Style::default().fg(colors.fg)),
        Span::styled(&app.formula_input, Style::default().fg(colors.fg)),
        Span::styled("_", Style::default().fg(colors.accent).add_modifier(Modifier::SLOW_BLINK)),
    ]);

    let paragraph = Paragraph::new(line).style(Style::default().bg(colors.bg));
    f.render_widget(paragraph, area);
}

pub fn render_ai_key_setup(f: &mut Frame, area: Rect, app: &App, colors: &ThemeColors) {
    let badge = Span::styled(
        " AI SETUP ",
        Style::default()
            .fg(colors.bg)
            .bg(colors.purple)
            .add_modifier(Modifier::BOLD),
    );

    let body = match &app.ai_key_stage {
        AiKeyStage::PickProvider => {
            let providers = [
                ("Anthropic (Claude)", 0),
                ("OpenAI (GPT)", 1),
            ];
            let mut spans = vec![badge, Span::styled(
                "  No API key configured. Choose a provider: ",
                Style::default().fg(colors.fg),
            )];
            for (label, idx) in providers {
                let is_selected = app.ai_key_provider_idx == idx;
                let style = if is_selected {
                    Style::default()
                        .fg(colors.cursor_fg)
                        .bg(colors.cursor_bg)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(colors.muted)
                };
                let prefix = if is_selected { "[ " } else { "  " };
                let suffix = if is_selected { " ]" } else { "  " };
                spans.push(Span::styled(format!("{}{}{}", prefix, label, suffix), style));
            }
            Line::from(spans)
        }
        AiKeyStage::EnterKey => {
            let provider_label = if app.ai_key_provider_idx == 0 { "Anthropic" } else { "OpenAI" };
            // Mask the key but show last 4 chars for confidence.
            let len = app.ai_key_input.len();
            let masked = if len <= 4 {
                "*".repeat(len)
            } else {
                let tail = &app.ai_key_input[len.saturating_sub(4)..];
                format!("{}{}", "*".repeat(len - 4), tail)
            };
            Line::from(vec![
                badge,
                Span::styled(
                    format!("  {} key: ", provider_label),
                    Style::default().fg(colors.fg),
                ),
                Span::styled(masked, Style::default().fg(colors.accent)),
                Span::styled(
                    "_",
                    Style::default()
                        .fg(colors.accent)
                        .add_modifier(Modifier::SLOW_BLINK),
                ),
            ])
        }
    };

    let paragraph = Paragraph::new(body).style(Style::default().bg(colors.bg));
    f.render_widget(paragraph, area);
}

pub fn render_computed_column(f: &mut Frame, area: Rect, app: &App, colors: &ThemeColors) {
    let content = match &app.computed_col_stage {
        ComputedColumnStage::EnterName => {
            format!("Name: {}", app.computed_col_name)
        }
        ComputedColumnStage::EnterExpression => {
            format!("{} = {}", app.computed_col_name, app.computed_col_expr)
        }
    };

    let line = Line::from(vec![
        Span::styled(
            " +COL ",
            Style::default()
                .fg(colors.bg)
                .bg(colors.green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" ", Style::default().fg(colors.fg)),
        Span::styled(content, Style::default().fg(colors.fg)),
        Span::styled("_", Style::default().fg(colors.accent).add_modifier(Modifier::SLOW_BLINK)),
    ]);

    let paragraph = Paragraph::new(line).style(Style::default().bg(colors.bg));
    f.render_widget(paragraph, area);
}

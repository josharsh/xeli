use crate::app::{App, FilterStage};
use crate::ui::theme::ThemeColors;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn render(f: &mut Frame, area: Rect, app: &App, colors: &ThemeColors) {
    let mut spans = vec![Span::styled(
        " Filters: ",
        Style::default()
            .fg(colors.pink)
            .add_modifier(Modifier::BOLD),
    )];

    for (i, filter) in app.filters.iter().enumerate() {
        let style = if filter.enabled {
            Style::default()
                .fg(colors.bg)
                .bg(colors.pink)
        } else {
            Style::default()
                .fg(colors.muted)
                .bg(colors.border)
        };

        spans.push(Span::styled(
            format!(" {} ", filter.display()),
            style,
        ));

        if i < app.filters.len() - 1 {
            spans.push(Span::styled(" AND ", Style::default().fg(colors.muted)));
        }
    }

    spans.push(Span::styled(
        " (F to clear) ",
        Style::default().fg(colors.muted),
    ));

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).style(Style::default().bg(colors.bg));
    f.render_widget(paragraph, area);
}

pub fn render_input(f: &mut Frame, area: Rect, app: &App, colors: &ThemeColors) {
    let operators = ["=", "!=", ">", ">=", "<", "<=", "contains", "starts with", "ends with", "is null", "is not null", "regex"];
    let vis_cols = app.visible_columns();

    let line = match &app.filter_stage {
        FilterStage::SelectColumn => {
            let col_name = vis_cols
                .get(app.filter_selected_col)
                .map(|(_, c)| c.name.as_str())
                .unwrap_or("?");

            Line::from(vec![
                Span::styled(
                    " Filter ",
                    Style::default()
                        .fg(colors.bg)
                        .bg(colors.pink)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" Column: {} ", col_name),
                    Style::default().fg(colors.fg),
                ),
                Span::styled(
                    " (j/k or arrows to select, Enter to confirm) ",
                    Style::default().fg(colors.muted),
                ),
            ])
        }
        FilterStage::SelectOperator => {
            let col_name = vis_cols
                .get(app.filter_selected_col)
                .map(|(_, c)| c.name.as_str())
                .unwrap_or("?");
            let op = operators.get(app.filter_selected_op).unwrap_or(&"=");

            Line::from(vec![
                Span::styled(
                    " Filter ",
                    Style::default()
                        .fg(colors.bg)
                        .bg(colors.pink)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" {} ", col_name),
                    Style::default().fg(colors.accent),
                ),
                Span::styled(
                    format!(" {} ", op),
                    Style::default()
                        .fg(colors.fg)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " (j/k to select operator, Enter to confirm) ",
                    Style::default().fg(colors.muted),
                ),
            ])
        }
        FilterStage::EnterValue => {
            let col_name = vis_cols
                .get(app.filter_selected_col)
                .map(|(_, c)| c.name.as_str())
                .unwrap_or("?");
            let op = operators.get(app.filter_selected_op).unwrap_or(&"=");

            Line::from(vec![
                Span::styled(
                    " Filter ",
                    Style::default()
                        .fg(colors.bg)
                        .bg(colors.pink)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" {} {} ", col_name, op),
                    Style::default().fg(colors.accent),
                ),
                Span::styled(&app.filter_input, Style::default().fg(colors.fg)),
                Span::styled("_", Style::default().fg(colors.accent).add_modifier(Modifier::SLOW_BLINK)),
            ])
        }
    };

    let paragraph = Paragraph::new(line).style(Style::default().bg(colors.bg));
    f.render_widget(paragraph, area);
}

// src/ui/dashboard.rs

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::society::Society;
use crate::game::Game;

/// Determines color based on mood
fn mood_color(mood: f64) -> Color {
    if mood > 0.5 {
        Color::Green
    } else if mood > 0.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}

/// Draws the main dashboard view
pub fn draw_dashboard(f: &mut Frame, game: &Game, selected_group_index: usize) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(15), // Summary
            Constraint::Percentage(70), // Interest Groups
            Constraint::Percentage(15), // Issues
        ])
        .split(size);

    draw_summary(f, &game, chunks[0]);
    draw_interest_groups(f, &game.society, chunks[1], selected_group_index);
    draw_active_issues(f, &game.society, chunks[2]);
}

/// Draws the overall mood summary at the top
fn draw_summary(f: &mut Frame, game: &Game, area: ratatui::layout::Rect) {
    let lines = vec![
        Line::from(vec![
            Span::raw(format!("Turn: {}", game.turn)),
        ]),
        Line::from(vec![
            Span::raw("Overall Mood: "),
            Span::styled(
                format!("{:.2}", game.society.overall_mood()),
                Style::default().fg(mood_color(game.society.overall_mood())),
            ),
        ]),
    ];

    let summary = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title("Summary"))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(summary, area);
}

/// Draws interest groups and their subgroups
fn draw_interest_groups(f: &mut Frame, society: &Society, area: ratatui::layout::Rect, selected_group_index: usize) {
    let mut items = Vec::new();
    let mut selectable_indices = Vec::new(); // Track indices of groups (not subgroups)

    for (i, group) in society.interest_groups.iter().enumerate() {
        // Record this index for selectable groups
        selectable_indices.push(items.len());

        let group_item = ListItem::new(Line::from(vec![
            Span::styled(
                format!("{} ({:.1}%)", group.name, group.population_ratio * 100.0),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" - Mood "),
            Span::styled(
                format!("{:.2}", group.weighted_mood()),
                Style::default().fg(mood_color(group.weighted_mood())),
            ),
        ]));
        items.push(group_item);

        // Add subgroups (non-selectable)
        for subgroup in &group.subgroups {
            let subgroup_item = ListItem::new(Line::from(vec![
                Span::raw(format!("    - {} ({:.1}%)", subgroup.name, subgroup.ratio * 100.0)),
                Span::raw(" Mood "),
                Span::styled(
                    format!("{:.2}", subgroup.mood),
                    Style::default().fg(mood_color(subgroup.mood)),
                ),
            ]));
            items.push(subgroup_item);
        }
    }

    let mut list_state = ListState::default();
    if !selectable_indices.is_empty() {
        list_state.select(Some(selectable_indices[selected_group_index]));
    }

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Interest Groups"))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut list_state);
}

/// Draws currently active issues at the bottom
fn draw_active_issues(f: &mut Frame, society: &Society, area: ratatui::layout::Rect) {
    let issue_items: Vec<ListItem> = society.issues.iter()
        .map(|issue| {
            ListItem::new(Line::from(vec![
                Span::raw("- "),
                Span::styled(
                    issue.get_name(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]))
        })
        .collect();

    let issues_list = List::new(issue_items)
        .block(Block::default().borders(Borders::ALL).title("Active Issues"));

    f.render_widget(issues_list, area);
}


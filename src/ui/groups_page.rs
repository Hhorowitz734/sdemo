// src/ui/groups_page.rs

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::interest_group::InterestGroup;

/// Returns color based on mood level
fn mood_color(mood: f64) -> Color {
    if mood > 0.5 {
        Color::Green
    } else if mood > 0.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}

/// Draws the detailed view of a single InterestGroup
pub fn draw_group_page(f: &mut Frame, group: &InterestGroup) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Header
            Constraint::Min(5),      // Subgroups
            Constraint::Length(5),   // Applicable Issues
        ])
        .split(f.size());

    // ===== Header block =====
    let header_text = Line::from(vec![
        Span::styled("Group: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(&group.name),
        Span::raw(" ("),
        Span::raw(format!("{:.1}%", group.population_ratio * 100.0)),
        Span::raw(" of population)"),
    ]);

    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title("Interest Group"))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(header, chunks[0]);

    // ===== Subgroups block =====
    let subgroup_items: Vec<ListItem> = group.subgroups.iter().map(|sub| {
        ListItem::new(Line::from(vec![
            Span::raw(format!("{} ({:.1}%) - Mood ", sub.name, sub.ratio * 100.0)),
            Span::styled(
                format!("{:.2}", sub.mood),
                Style::default().fg(mood_color(sub.mood)),
            ),
        ]))
    }).collect();

    let subgroup_list = List::new(subgroup_items)
        .block(Block::default().borders(Borders::ALL).title("Subgroups"));

    f.render_widget(subgroup_list, chunks[1]);

    // ===== Applicable Issues block =====
    let issue_items: Vec<ListItem> = group.applicable_issues.iter().map(|issue| {
        ListItem::new(Span::raw(format!("- {}", issue)))
    }).collect();

    let issues_list = List::new(issue_items)
        .block(Block::default().borders(Borders::ALL).title("Applicable Issues"));

    f.render_widget(issues_list, chunks[2]);
}

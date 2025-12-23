//! UI rendering module.
//!
//! This module handles all the TUI rendering using ratatui,
//! implementing the cyber-command center aesthetic.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::app::{App, LogLevel, Tab};
use crate::particles::ParticleWidget;
use crate::timeline::{TimelineStatusWidget, TimelineWidget};

/// Neon color palette
pub mod colors {
    use ratatui::style::Color;

    pub const BG_DARK: Color = Color::Rgb(10, 10, 20);
    pub const BG_MEDIUM: Color = Color::Rgb(20, 20, 35);
    pub const BORDER: Color = Color::Rgb(0, 200, 200);
    pub const BORDER_DIM: Color = Color::Rgb(50, 100, 100);
    pub const CYAN: Color = Color::Rgb(0, 255, 255);
    pub const MAGENTA: Color = Color::Rgb(255, 0, 255);
    pub const GREEN: Color = Color::Rgb(0, 255, 128);
    pub const YELLOW: Color = Color::Rgb(255, 255, 0);
    pub const RED: Color = Color::Rgb(255, 50, 50);
    pub const TEXT: Color = Color::Rgb(200, 200, 200);
    pub const TEXT_DIM: Color = Color::Rgb(100, 100, 100);
}

/// Render the entire UI
pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Render background particles first
    frame.render_widget(ParticleWidget::new(&app.particle_system), area);

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Status bar / tabs
            Constraint::Min(10),    // Main content
            Constraint::Length(5),  // Log area
        ])
        .split(area);

    // Render components
    render_tabs(frame, app, chunks[0]);
    render_main_content(frame, app, chunks[1]);
    render_logs(frame, app, chunks[2]);

    // Render overlays (error popup, help)
    if app.error_popup.is_some() {
        render_error_popup(frame, app, area);
    }

    if app.show_help {
        render_help_overlay(frame, area);
    }
}

/// Render the tab bar
fn render_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = [Tab::Clients, Tab::Timeline, Tab::Users]
        .iter()
        .map(|tab| {
            let style = if *tab == app.active_tab {
                Style::default()
                    .fg(colors::CYAN)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(colors::TEXT_DIM)
            };
            Line::from(Span::styled(format!(" {} ", tab.name()), style))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .title(" SWEeM Cyber Command ")
                .title_style(Style::default().fg(colors::MAGENTA).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors::BORDER))
                .style(Style::default().bg(colors::BG_MEDIUM)),
        )
        .select(match app.active_tab {
            Tab::Clients => 0,
            Tab::Timeline => 1,
            Tab::Users => 2,
        })
        .style(Style::default().fg(colors::TEXT))
        .highlight_style(Style::default().fg(colors::CYAN).add_modifier(Modifier::BOLD))
        .divider(Span::styled(" │ ", Style::default().fg(colors::BORDER_DIM)));

    frame.render_widget(tabs, area);
}

/// Render the main content area based on active tab
fn render_main_content(frame: &mut Frame, app: &App, area: Rect) {
    match app.active_tab {
        Tab::Clients => render_clients_view(frame, app, area),
        Tab::Timeline => render_timeline_view(frame, app, area),
        Tab::Users => render_users_view(frame, app, area),
    }
}

/// Render the timeline view
fn render_timeline_view(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(1)])
        .split(area);

    // Render timeline
    let timeline = TimelineWidget::new(&app.projects, &app.timeline_state);
    frame.render_widget(timeline, chunks[0]);

    // Render status
    let status = TimelineStatusWidget::new(&app.timeline_state, app.projects.len());
    frame.render_widget(status, chunks[1]);
}

/// Render the clients list view
fn render_clients_view(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .clients
        .iter()
        .enumerate()
        .map(|(i, client)| {
            let is_selected = i == app.list_selected;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(colors::CYAN)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(colors::TEXT)
            };

            let content = Line::from(vec![
                Span::styled(
                    format!("{:20}", client.display_name()),
                    style,
                ),
                Span::styled(" │ ", Style::default().fg(colors::BORDER_DIM)),
                Span::styled(
                    format!("{:30}", client.address.as_deref().unwrap_or("-")),
                    style.fg(if is_selected { Color::Black } else { colors::TEXT_DIM }),
                ),
                Span::styled(" │ ", Style::default().fg(colors::BORDER_DIM)),
                Span::styled(
                    format!("Projects: {}/{}", client.projects_completed, client.projects_total),
                    style.fg(if is_selected { Color::Black } else { colors::GREEN }),
                ),
            ]);

            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Clients ")
                .title_style(Style::default().fg(colors::CYAN).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors::BORDER))
                .style(Style::default().bg(colors::BG_DARK)),
        )
        .style(Style::default());

    frame.render_widget(list, area);

    // Render empty state
    if app.clients.is_empty() {
        render_empty_state(frame, area, "No clients found", app.is_loading);
    }
}

/// Render the users list view
fn render_users_view(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .users
        .iter()
        .enumerate()
        .map(|(i, user)| {
            let is_selected = i == app.list_selected;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(colors::MAGENTA)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(colors::TEXT)
            };

            let role_color = match user.role {
                crate::models::Role::Admin => colors::YELLOW,
                crate::models::Role::User => colors::GREEN,
            };

            let content = Line::from(vec![
                Span::styled(
                    format!("{:20}", user.display_name()),
                    style,
                ),
                Span::styled(" │ ", Style::default().fg(colors::BORDER_DIM)),
                Span::styled(
                    format!("{:20}", user.login.as_deref().unwrap_or("-")),
                    style.fg(if is_selected { Color::Black } else { colors::TEXT_DIM }),
                ),
                Span::styled(" │ ", Style::default().fg(colors::BORDER_DIM)),
                Span::styled(
                    format!("{:10}", user.role),
                    style.fg(if is_selected { Color::Black } else { role_color }),
                ),
            ]);

            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Users ")
                .title_style(Style::default().fg(colors::MAGENTA).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors::BORDER))
                .style(Style::default().bg(colors::BG_DARK)),
        )
        .style(Style::default());

    frame.render_widget(list, area);

    // Render empty state
    if app.users.is_empty() {
        render_empty_state(frame, area, "No users found", app.is_loading);
    }
}

/// Render the log area
fn render_logs(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .logs
        .iter()
        .rev()
        .take(area.height.saturating_sub(2) as usize)
        .map(|entry| {
            let (prefix, color) = match entry.level {
                LogLevel::Info => ("ℹ", colors::CYAN),
                LogLevel::Success => ("✓", colors::GREEN),
                LogLevel::Warning => ("⚠", colors::YELLOW),
                LogLevel::Error => ("✗", colors::RED),
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", prefix), Style::default().fg(color)),
                Span::styled(&entry.message, Style::default().fg(colors::TEXT_DIM)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" System Log ")
                .title_style(Style::default().fg(colors::GREEN))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors::BORDER_DIM))
                .style(Style::default().bg(colors::BG_DARK)),
        );

    frame.render_widget(list, area);
}

/// Render empty state message
fn render_empty_state(frame: &mut Frame, area: Rect, message: &str, is_loading: bool) {
    let text = if is_loading {
        "Loading..."
    } else {
        message
    };

    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(colors::TEXT_DIM))
        .alignment(ratatui::layout::Alignment::Center);

    // Center the message
    let inner = Block::default().borders(Borders::ALL).inner(area);
    let y = inner.y + inner.height / 2;
    let centered = Rect::new(inner.x, y, inner.width, 1);

    frame.render_widget(paragraph, centered);
}

/// Render error popup
fn render_error_popup(frame: &mut Frame, app: &App, area: Rect) {
    let popup = app.error_popup.as_ref().unwrap();

    let popup_width = (area.width * 60 / 100).min(60).max(30);
    let popup_height = 7;

    let popup_area = centered_rect(popup_width, popup_height, area);

    // Clear the area behind the popup
    frame.render_widget(Clear, popup_area);

    // Render the popup
    let block = Block::default()
        .title(format!(" {} ", popup.title))
        .title_style(
            Style::default()
                .fg(Color::White)
                .bg(colors::RED)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::RED))
        .style(Style::default().bg(Color::Rgb(40, 10, 10)));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let text = Paragraph::new(popup.message.as_str())
        .style(Style::default().fg(colors::TEXT))
        .wrap(Wrap { trim: true });

    frame.render_widget(text, inner);

    // Dismiss hint
    let hint = Paragraph::new("Press ESC or ENTER to dismiss")
        .style(Style::default().fg(colors::TEXT_DIM))
        .alignment(ratatui::layout::Alignment::Center);

    let hint_area = Rect::new(
        popup_area.x,
        popup_area.y + popup_area.height - 1,
        popup_area.width,
        1,
    );
    frame.render_widget(hint, hint_area);
}

/// Render help overlay
fn render_help_overlay(frame: &mut Frame, area: Rect) {
    let popup_width = 50;
    let popup_height = 18;
    let popup_area = centered_rect(popup_width, popup_height, area);

    frame.render_widget(Clear, popup_area);

    let help_text = vec![
        Line::from(Span::styled(
            "Keyboard Shortcuts",
            Style::default()
                .fg(colors::CYAN)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Navigation", Style::default().fg(colors::MAGENTA).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  Tab/Shift+Tab ", Style::default().fg(colors::CYAN)),
            Span::raw("Switch tabs"),
        ]),
        Line::from(vec![
            Span::styled("  j/k or ↑/↓    ", Style::default().fg(colors::CYAN)),
            Span::raw("Move up/down"),
        ]),
        Line::from(vec![
            Span::styled("  h/l or ←/→    ", Style::default().fg(colors::CYAN)),
            Span::raw("Scroll timeline"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Timeline", Style::default().fg(colors::MAGENTA).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  +/-           ", Style::default().fg(colors::CYAN)),
            Span::raw("Zoom in/out"),
        ]),
        Line::from(vec![
            Span::styled("  t             ", Style::default().fg(colors::CYAN)),
            Span::raw("Center on today"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("General", Style::default().fg(colors::MAGENTA).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  r             ", Style::default().fg(colors::CYAN)),
            Span::raw("Refresh data"),
        ]),
        Line::from(vec![
            Span::styled("  p             ", Style::default().fg(colors::CYAN)),
            Span::raw("Toggle particles"),
        ]),
        Line::from(vec![
            Span::styled("  q/Ctrl+C      ", Style::default().fg(colors::CYAN)),
            Span::raw("Quit"),
        ]),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .title(" Help ")
                .title_style(Style::default().fg(colors::GREEN).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors::BORDER))
                .style(Style::default().bg(colors::BG_MEDIUM)),
        )
        .style(Style::default().fg(colors::TEXT));

    frame.render_widget(paragraph, popup_area);
}

/// Helper to create a centered rectangle
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}

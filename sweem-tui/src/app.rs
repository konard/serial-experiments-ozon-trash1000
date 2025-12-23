//! Application state and event handling.
//!
//! This module implements the Elm Architecture pattern for state management,
//! with a centralized App struct holding all application state.

use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::api::{ApiCommand, ApiMessage};
use crate::models::{ClientDto, ProjectDto, UserDto};
use crate::particles::{ParticleMode, ParticleSystem};
use crate::timeline::TimelineState;

/// Active tab in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    /// Clients list view
    Clients,
    /// Project timeline view (default)
    #[default]
    Timeline,
    /// Users list view
    Users,
}

impl Tab {
    /// Move to the next tab
    pub fn next(&self) -> Self {
        match self {
            Tab::Clients => Tab::Timeline,
            Tab::Timeline => Tab::Users,
            Tab::Users => Tab::Clients,
        }
    }

    /// Move to the previous tab
    pub fn previous(&self) -> Self {
        match self {
            Tab::Clients => Tab::Users,
            Tab::Timeline => Tab::Clients,
            Tab::Users => Tab::Timeline,
        }
    }

    /// Get the display name of the tab
    pub fn name(&self) -> &'static str {
        match self {
            Tab::Clients => "Clients",
            Tab::Timeline => "Timeline",
            Tab::Users => "Users",
        }
    }
}

/// Error popup state
#[derive(Debug, Clone)]
pub struct ErrorPopup {
    /// Error title
    pub title: String,
    /// Error message
    pub message: String,
    /// When the error was shown
    pub shown_at: Instant,
    /// Auto-dismiss duration (None for manual dismiss)
    pub auto_dismiss: Option<Duration>,
}

impl ErrorPopup {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            shown_at: Instant::now(),
            auto_dismiss: Some(Duration::from_secs(5)),
        }
    }

    pub fn should_dismiss(&self) -> bool {
        if let Some(duration) = self.auto_dismiss {
            self.shown_at.elapsed() > duration
        } else {
            false
        }
    }
}

/// Log entry for the message area
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: Instant,
    pub message: String,
    pub level: LogLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl LogEntry {
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            timestamp: Instant::now(),
            message: message.into(),
            level: LogLevel::Info,
        }
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self {
            timestamp: Instant::now(),
            message: message.into(),
            level: LogLevel::Success,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            timestamp: Instant::now(),
            message: message.into(),
            level: LogLevel::Warning,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            timestamp: Instant::now(),
            message: message.into(),
            level: LogLevel::Error,
        }
    }
}

/// Main application state
#[derive(Debug)]
pub struct App {
    /// Whether the application should quit
    pub should_quit: bool,

    /// Currently active tab
    pub active_tab: Tab,

    /// Projects data
    pub projects: Vec<ProjectDto>,

    /// Clients data
    pub clients: Vec<ClientDto>,

    /// Users data
    pub users: Vec<UserDto>,

    /// Timeline widget state
    pub timeline_state: TimelineState,

    /// Particle system for background animation
    pub particle_system: ParticleSystem,

    /// Current error popup (if any)
    pub error_popup: Option<ErrorPopup>,

    /// Log messages
    pub logs: Vec<LogEntry>,
    /// Maximum number of log entries to keep
    max_logs: usize,

    /// Selected index in lists (clients/users views)
    pub list_selected: usize,

    /// API connection status
    pub api_connected: bool,

    /// Last data refresh time
    pub last_refresh: Option<Instant>,

    /// Whether data is currently loading
    pub is_loading: bool,

    /// Frame counter for animations
    pub frame_count: u64,

    /// Show help overlay
    pub show_help: bool,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Create a new application instance
    pub fn new() -> Self {
        let mut app = Self {
            should_quit: false,
            active_tab: Tab::Timeline,
            projects: Vec::new(),
            clients: Vec::new(),
            users: Vec::new(),
            timeline_state: TimelineState::default(),
            particle_system: ParticleSystem::new(ParticleMode::DigitalRain, 80),
            error_popup: None,
            logs: Vec::new(),
            max_logs: 100,
            list_selected: 0,
            api_connected: false,
            last_refresh: None,
            is_loading: true,
            frame_count: 0,
            show_help: false,
        };

        app.log(LogEntry::info("SWEeM TUI initialized"));
        app.log(LogEntry::info("Connecting to API..."));
        app
    }

    /// Add a log entry
    pub fn log(&mut self, entry: LogEntry) {
        self.logs.push(entry);
        if self.logs.len() > self.max_logs {
            self.logs.remove(0);
        }
    }

    /// Show an error popup
    pub fn show_error(&mut self, title: impl Into<String>, message: impl Into<String>) {
        let title = title.into();
        let message = message.into();
        self.log(LogEntry::error(format!("{}: {}", title, message)));
        self.error_popup = Some(ErrorPopup::new(title, message));
    }

    /// Dismiss the current error popup
    pub fn dismiss_error(&mut self) {
        self.error_popup = None;
    }

    /// Handle API messages
    pub fn handle_api_message(&mut self, message: ApiMessage) {
        match message {
            ApiMessage::ProjectsLoaded(projects) => {
                let count = projects.len();
                self.projects = projects;
                self.is_loading = false;
                self.last_refresh = Some(Instant::now());
                self.log(LogEntry::success(format!("Loaded {} projects", count)));
            }
            ApiMessage::ClientsLoaded(clients) => {
                let count = clients.len();
                self.clients = clients;
                self.log(LogEntry::success(format!("Loaded {} clients", count)));
            }
            ApiMessage::UsersLoaded(users) => {
                let count = users.len();
                self.users = users;
                self.log(LogEntry::success(format!("Loaded {} users", count)));
            }
            ApiMessage::Error(error) => {
                self.is_loading = false;
                self.show_error("API Error", error);
            }
            ApiMessage::ConnectionStatus(connected) => {
                let was_connected = self.api_connected;
                self.api_connected = connected;

                if connected && !was_connected {
                    self.log(LogEntry::success("Connected to API"));
                } else if !connected && was_connected {
                    self.log(LogEntry::warning("Disconnected from API"));
                }
            }
        }
    }

    /// Handle key events and return optional API command
    pub fn handle_key(&mut self, key: KeyEvent) -> Option<ApiCommand> {
        // Handle error popup dismissal
        if self.error_popup.is_some() {
            if matches!(key.code, KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ')) {
                self.dismiss_error();
            }
            return None;
        }

        // Handle help overlay
        if self.show_help {
            if matches!(key.code, KeyCode::Esc | KeyCode::Char('?') | KeyCode::Enter) {
                self.show_help = false;
            }
            return None;
        }

        // Global shortcuts
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
                return Some(ApiCommand::Shutdown);
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
                return Some(ApiCommand::Shutdown);
            }
            KeyCode::Char('?') => {
                self.show_help = true;
                return None;
            }
            KeyCode::Char('p') => {
                self.particle_system.toggle_mode();
                let mode = match self.particle_system.mode() {
                    ParticleMode::DigitalRain => "Digital Rain",
                    ParticleMode::Starfield => "Starfield",
                    ParticleMode::None => "None",
                };
                self.log(LogEntry::info(format!("Particle mode: {}", mode)));
                return None;
            }
            KeyCode::Char('r') => {
                self.is_loading = true;
                self.log(LogEntry::info("Refreshing data..."));
                return Some(ApiCommand::RefreshAll);
            }
            KeyCode::Tab => {
                self.active_tab = self.active_tab.next();
                self.list_selected = 0;
                return None;
            }
            KeyCode::BackTab => {
                self.active_tab = self.active_tab.previous();
                self.list_selected = 0;
                return None;
            }
            _ => {}
        }

        // Tab-specific shortcuts
        match self.active_tab {
            Tab::Timeline => self.handle_timeline_key(key),
            Tab::Clients => self.handle_list_key(key, self.clients.len()),
            Tab::Users => self.handle_list_key(key, self.users.len()),
        }

        None
    }

    /// Handle timeline-specific key events
    fn handle_timeline_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('h') | KeyCode::Left => {
                let amount = if key.modifiers.contains(KeyModifiers::SHIFT) { 7 } else { 1 };
                self.timeline_state.scroll_left(amount);
            }
            KeyCode::Char('l') | KeyCode::Right => {
                let amount = if key.modifiers.contains(KeyModifiers::SHIFT) { 7 } else { 1 };
                self.timeline_state.scroll_right(amount);
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.timeline_state.select_next(self.projects.len());
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.timeline_state.select_previous(self.projects.len());
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.timeline_state.zoom_in();
            }
            KeyCode::Char('-') => {
                self.timeline_state.zoom_out();
            }
            KeyCode::Char('t') => {
                self.timeline_state.center_on_today(80); // Approximate width
            }
            KeyCode::Home => {
                self.timeline_state.scroll_offset = 0;
            }
            _ => {}
        }
    }

    /// Handle list view key events
    fn handle_list_key(&mut self, key: KeyEvent, total: usize) {
        if total == 0 {
            return;
        }

        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.list_selected = (self.list_selected + 1) % total;
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.list_selected = self.list_selected.checked_sub(1).unwrap_or(total - 1);
            }
            KeyCode::Char('g') => {
                self.list_selected = 0;
            }
            KeyCode::Char('G') => {
                self.list_selected = total.saturating_sub(1);
            }
            _ => {}
        }
    }

    /// Update animations (called every frame)
    pub fn tick(&mut self, width: u16, height: u16) {
        self.frame_count = self.frame_count.wrapping_add(1);

        // Update particles
        self.particle_system.update(width, height);

        // Auto-dismiss error popup
        if let Some(ref popup) = self.error_popup {
            if popup.should_dismiss() {
                self.error_popup = None;
            }
        }
    }

    /// Get the status bar text
    pub fn status_text(&self) -> String {
        let connection = if self.api_connected {
            "● Connected"
        } else {
            "○ Disconnected"
        };

        let loading = if self.is_loading { " [Loading...]" } else { "" };

        let last_refresh = self
            .last_refresh
            .map(|t| {
                let secs = t.elapsed().as_secs();
                if secs < 60 {
                    format!(" ({}s ago)", secs)
                } else {
                    format!(" ({}m ago)", secs / 60)
                }
            })
            .unwrap_or_default();

        format!(
            "{}{}{} | Tab: {} | ?: Help | q: Quit",
            connection,
            loading,
            last_refresh,
            self.active_tab.name()
        )
    }
}

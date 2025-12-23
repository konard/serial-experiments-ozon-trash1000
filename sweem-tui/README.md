# SWEeM TUI - Cyber Command Center

A high-performance, aesthetically stunning TUI (Text User Interface) for the SWEeM REST API, built with Rust.

## Features

- **Project Timeline Flux**: A horizontal Gantt chart visualization with time-based scrolling
- **Animated Background**: Digital rain or starfield particle effects
- **Vim-like Navigation**: Intuitive j/k/h/l key bindings
- **Real-time Updates**: Async API communication with non-blocking UI
- **Neon Cyber Aesthetic**: Dark theme with cyan/magenta/green accents

## Requirements

- Rust 1.70 or newer
- The SWEeM API running at `http://localhost:5094` (or specify custom URL)

## Building

```bash
cd sweem-tui
cargo build --release
```

## Running

```bash
# Use default API URL (http://localhost:5094)
cargo run --release

# Or specify a custom API URL
cargo run --release -- http://your-api-host:port
```

## Keyboard Shortcuts

### Navigation
- `Tab` / `Shift+Tab` - Switch between tabs (Clients, Timeline, Users)
- `j` / `k` or `Down` / `Up` - Move up/down in lists
- `h` / `l` or `Left` / `Right` - Scroll timeline horizontally
- `Shift+h` / `Shift+l` - Scroll timeline by week

### Timeline
- `+` / `-` - Zoom in/out
- `t` - Center on today
- `Home` - Jump to timeline start

### General
- `r` - Refresh data from API
- `p` - Toggle particle animation (Digital Rain / Starfield / None)
- `?` - Show help overlay
- `q` or `Ctrl+C` - Quit

## Architecture

The application follows the Elm Architecture pattern:

- **Model**: Centralized `App` struct holding all state
- **View**: Render functions in `ui.rs`
- **Update**: Event handlers in `app.rs`

Communication with the API is handled asynchronously via Tokio channels, ensuring the UI never blocks.

## Project Structure

```
sweem-tui/
├── Cargo.toml       # Dependencies and project metadata
└── src/
    ├── main.rs      # Entry point and event loop
    ├── api.rs       # API client and async communication
    ├── app.rs       # Application state and event handling
    ├── models.rs    # Domain models (Client, Project, User)
    ├── particles.rs # Background animation system
    ├── timeline.rs  # Gantt chart widget
    └── ui.rs        # UI rendering
```

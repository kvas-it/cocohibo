# Cocohibo

A terminal-based browser for Claude Code project history, built in Rust using Ratatui.

## Overview

Cocohibo allows you to explore and navigate through your Claude Code projects and chat histories in an intuitive terminal interface. It provides a hierarchical view of your projects, chats within those projects, and individual messages within each chat conversation.

## Features

- Browse Claude Code projects stored in `~/.claude/projects`
- Navigate through chats within each project
- View and search through message history
- Terminal-based interface with keyboard navigation
- Built using the ELM architecture pattern for clean state management

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Building from source

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd cocohibo
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Install the binary:
   ```bash
   cargo install --path .
   ```

## Usage

### Basic Usage

Run Cocohibo using either command:
```bash
cocohibo
# or the shorthand:
cch
```

### Custom Projects Directory

By default, Cocohibo looks for Claude Code projects in `~/.claude/projects`. You can specify a different directory using the environment variable:

```bash
COCOHIBO_PROJECTS_DIR=/path/to/your/projects cocohibo
```

### Navigation

- Use arrow keys or vim-like keys (h/j/k/l) to navigate
- Enter to select a project, chat, or view messages
- ESC or q to go back or quit

## Architecture

Cocohibo follows the ELM architecture pattern:

- **Model**: Contains application state including current screen, projects list, selected project, chats list, and messages
- **Update**: Handles user input and updates the model accordingly  
- **View**: Renders the current state as a terminal UI

The application supports three main screens:
1. **Projects**: List of available Claude Code projects
2. **Chats**: List of chats within the selected project
3. **Messages**: Message history for the selected chat

## Dependencies

- `ratatui`: Terminal UI framework
- `crossterm`: Cross-platform terminal manipulation
- `serde`: Serialization framework
- `serde-jsonlines`: JSONL file handling
- `chrono`: Date and time handling
- `dirs`: Directory path utilities

## Development

To run in development mode:
```bash
cargo run
```

To run tests:
```bash
cargo test
```

## License

MIT License.

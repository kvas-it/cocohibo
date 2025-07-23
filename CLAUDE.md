# Instructions for Claude Code

Cocohibo is a browser of Claude Code history (that normally lives in
`~/.claude/projects`) written in Rust using Ratatui. It allows you to view the
list of projects, list of chats in a project and to browse and search the
history of a chat.

## Architecture

- Cocohibo uses Ratatui with Crossterm backend.
- We follow ELM architecture (see https://ratatui.rs/concepts/application-patterns/the-elm-architecture/).
  - The model consists of:
    - current screen (projects, chats, messages),
    - list of projects (loaded from `~/.claude/projects`),
    - selected project (set once selected by user),
    - list of chats in the selected project (populated when a project is selected),
    - selected chat (set once selected by user),
    - list of messages in the selected chat (populated when a chat is selected).
  - The model updates in reponse to user input.
  - We render the current screen (see model) as a list.
- The main entry point is the `cocohibo` command (shorthand: `cch`).
- Use `serde_jsonlines` to read and write JSONL files.
- Use `chrono` for date and time handling.

## Test data

There is a test data in `tests` directory. It contains a couple of generated
projects with some chats and messages. You can use it to test the application.

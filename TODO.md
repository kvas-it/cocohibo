# Cocohibo

This is a browser of Claude Code history (that normally lives in
`~/.claude/projects`) written in Rust using Ratatui. It allows you to view the
list of projects, view the chats of one project and then to browse and search
the history of a chat.

## Tasks

- Adjust navigation between views:
  - Add `l` to enter the view (synonim for `Enter`).
  - Add `h` to go back to the previous view (synonim for `Esc`).
  - Make `q` quit the application (synonim for `Ctrl+C`).
- Implement a search feature:
  - In all views pressing / switches to search mode.
  - In search mode all letters, numbers, and symbols are added to the current
    search query. Backspace removes the last character from the search query.
  - The search query is immediately applied to the current view. Only keep
    items that contain the search query in the title (title, chat name, etc.).
  - `Esc` exits search mode and clears the search.
  - `Enter` in search mode exits search mode but keeps the filter applied.
  - Going to another view (e.g. from projects to chats) clears the search.

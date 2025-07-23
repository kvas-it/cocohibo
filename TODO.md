# Cocohibo

This is a browser of Claude Code history (that normally lives in
`~/.claude/projects`) written in Rust using Ratatui. It allows you to view the
list of projects, view the chats of one project and then to browse and search
the history of a chat.

## Tasks

- Implement a search feature:
  - Add search query to the model. This is a filter that is applied to the
    current view (projects, chats, messages). Only items that contain the
    search query in the title (title, chat name, etc.) as a substring should be
    displayed.
  - Think how to adjust the model to support search. Perhaps we need to have
    two lists of items: one for the full list and another for the filtered
    list. If you can think of a better way, propose it.
  - In all views pressing `/` activates the search mode.
    - When entering search mode, any previous search query is cleared.
    - In search mode all typed letters, numbers, and symbols are added to the
      current search query. Backspace removes the last character from the
      search query.
    - The search query is displayed in the status bar after the `/` character.
    - The search query is immediately applied to the current view. 
    - `Esc` exits search mode and clears the search query.
    - `Enter` in search mode exits search mode and keeps the search query (so
      the items in the list stay filtered).
  - Going to another view (e.g. from projects to chats) clears the search
    query.

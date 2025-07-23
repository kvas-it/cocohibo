# Cocohibo

This is a browser of Claude Code history (that normally lives in
`~/.claude/projects`) written in Rust using Ratatui. It allows you to view the
list of projects, view the chats of one project and then to browse and search
the history of a chat.

## Tasks

- Change the meaning of z/t/b keys in the message view:
  - `t` should scroll the view so that selected item is at the top of the
    view (or as close as possible)
  - `z` should scroll the view so that selected item is at the center of the
    view (or as close as possible)
  - `b` should scroll the view so that selected item is at the bottom of the
    view (or as close as possible)

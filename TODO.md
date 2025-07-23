# Cocohibo

This is a browser of Claude Code history (that normally lives in
`~/.claude/projects`) written in Rust using Ratatui. It allows you to view the
list of projects, view the chats of one project and then to browse and search
the history of a chat.

## Tasks

- Make list search fuzzy:
  - Instead of exact substring match, use fuzzy matching, similar to how
    `fzf` works.

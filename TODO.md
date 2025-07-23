# Cocohibo

This is a browser of Claude Code history (that normally lives in
`~/.claude/projects`) written in Rust using Ratatui. It allows you to view the
list of projects, view the chats of one project and then to browse and search
the history of a chat.

## Tasks

- Customize the directory from where the projects are loaded:
  - Default to `~/.claude/projects`.
  - Allow specifying a different directory using the `--projects-dir` command line
    argument.
  - Allow specifying a different directory using the `COCOHIBO_PROJECTS_DIR`
    environment variable.
  - If the directory does not exist, show an error message and exit.

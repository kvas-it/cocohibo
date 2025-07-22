# Cocohibo

This is a browser of Claude Code history (that normally lives in
`~/.claude/projects`) written in Rust using Ratatui. It allows you to view the
list of projects, view the chats of one project and then to browse and search
the history of a chat.

## Tasks

- Display message details:
  - Messages have additional fields, some of which we would like to display in
    a richer way.
  - Look at existing messages in some chats to figure out the data model. Make
    a proposal for the data model and how to display it.
- Status line:
  - Show the current project (when available) and current chat (when available)
    in the status line at the bottom of the screen.
  - In the project list show "Project list" instead.
- Customize the directory from where the projects are loaded:
  - Default to `~/.claude/projects`.
  - Allow specifying a different directory using the `--projects-dir` command line
    argument.
  - Allow specifying a different directory using the `COCOHIBO_PROJECTS_DIR`
    environment variable.
  - If the directory does not exist, show an error message and exit.
- The user can exit the application with `Ctrl+C` from anywhere.
- Adjust the column widths in the lists of projects and chats to use space more
  efficiently:
  - Projects list: show project name, last modified date, number of chats.
  - Chats list: show chat name, last modified date, number of messages.

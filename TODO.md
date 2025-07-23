# Cocohibo

This is a browser of Claude Code history (that normally lives in
`~/.claude/projects`) written in Rust using Ratatui. It allows you to view the
list of projects, view the chats of one project and then to browse and search
the history of a chat.

## Tasks

- Projects list improvements:
  - Use all of the available width in the terminal, don't trim the project
    names as much.
  - When trimming is necessary, trim the beginning of the project name instead
    of the end -- the final part carries more information.
- Chats list improvements:
  - Use all of the available width in the terminal, don't trim the chat names
    as much.
- Messages view improvements:
  - Replace [ASST] and [USER] with just one letter: A and U to leave more space
    for the message text.
  - Adjust message text trimming to use all of the available space.
- Detail view improvements:
  - Don't trim the working dir. Wrap it if necessary.
- Customize the directory from where the projects are loaded:
  - Default to `~/.claude/projects`.
  - Allow specifying a different directory using the `--projects-dir` command line
    argument.
  - Allow specifying a different directory using the `COCOHIBO_PROJECTS_DIR`
    environment variable.
  - If the directory does not exist, show an error message and exit.
- Status line:
  - Show the current project (when available) and current chat (when available)
    in the status line at the bottom of the screen.
  - In the project list show "Project list" instead.
- The user can exit the application with `Ctrl+C` from anywhere.
- Adjust the column widths in the lists of projects and chats to use space more
  efficiently:
  - Projects list: show project name, last modified date, number of chats.
  - Chats list: show chat name, last modified date, number of messages.

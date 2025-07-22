# Cocohibo

This is a browser of Claude Code history (that normally lives in
`~/.claude/projects`) written in Rust using Ratatui. It allows you to view the
list of projects, view the chats of one project and then to browse and search
the history of a chat.

## Tasks

- Implement project selection, chats screen:
  - Show the currently selected project in the list (highlight it). Start with
    the first project in the list.
  - Allow moving selection up and down using arrow keys and `j`/`k`.
  - If the project list doesn't fit the screen, allow scrolling, when the user
    gets to the bottom of the screen (and scrolling up when the user is at the
    top of the screen).
  - Allow opening selected project with `Enter`.
  - When the project is opened, show the chats screen:
    - It contains list of chats in the project.
    - The chats are JSONL files in the project folder.
    - For each chat, show:
      - Name of the chat (file name without extension).
      - Last modified date of the chat.
      - Number of messages in the chat (count lines in the file).
    - Sort the list by last modified date.
  - The user can exit the chats screen with `Esc` or `q`.
  - Use the List widget from Ratatui to display the list of projects.
- Implement chat selection in the chats screen, start messages screen:
  - Show the currently selected chat in the list (highlight it). Start with
    the first chat in the list.
  - Allow moving selection up and down using arrow keys and `j`/`k`.
  - If the chat list doesn't fit the screen, allow scrolling, when the user
    gets to the bottom of the screen (and scrolling up when the user is at the
    top of the screen).
  - Allow opening selected chat with `Enter`.
  - When the chat is opened, show the messages screen:
    - It contains list of messages in the chat.
    - For each message, show:
      - Timestamp of the message.
      - Type of the message (e.g. "user" or "assistant").
- Implement message browsing in the messages screen:
  - Show the currently selected message in the list (highlight it). Start with
    the first message in the list.
  - Allow scrolling through messages using arrow keys and `j`/`k`.
  - If the message list doesn't fit the screen, allow scrolling, when the user
    gets to the bottom of the screen (and scrolling up when the user is at the
    top of the screen).
  - The user can exit the messages screen with `Esc` or `q`.
- Message hierarchy:
  - Messages can be nested (each message has `uid` and `parentUid` keys to
    represent the hierarchy).
  - Show the hierarchy of messages in the messages screen (submessages are
    indented under their parent message). One indentation level is 2 spaces.
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

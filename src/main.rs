use clap::Parser;
use cocohibo::{app::App, events, ui};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, layout::Rect, Terminal};
use std::{env, io, path::PathBuf};

#[derive(Parser)]
#[command(name = "cocohibo")]
#[command(about = "A browser for Claude Code history")]
struct Cli {
    #[arg(long, help = "Directory containing Claude Code projects")]
    projects_dir: Option<PathBuf>,
    #[arg(
        long,
        help = "Use vertical split (up/down) instead of horizontal split"
    )]
    vertical_split: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let projects_dir = get_projects_dir(cli.projects_dir);

    let mut app = App::new(projects_dir, cli.vertical_split);

    if let Err(e) = app.load_projects() {
        eprintln!("Error loading projects: {}", e);
        return Err(e);
    }

    setup_terminal()?;
    let result = run_app(&mut app);
    restore_terminal()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        return Err(Box::new(err));
    }

    Ok(())
}

fn get_projects_dir(cli_projects_dir: Option<PathBuf>) -> PathBuf {
    if let Some(projects_dir) = cli_projects_dir {
        return projects_dir;
    }

    if let Ok(custom_dir) = env::var("COCOHIBO_PROJECTS_DIR") {
        return PathBuf::from(custom_dir);
    }

    if let Some(home) = dirs::home_dir() {
        home.join(".claude").join("projects")
    } else {
        PathBuf::from(".claude/projects")
    }
}

fn setup_terminal() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    Ok(())
}

fn restore_terminal() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn run_app(app: &mut App) -> io::Result<()> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    loop {
        let terminal_size = terminal.size()?;
        let terminal_area = Rect::new(0, 0, terminal_size.width, terminal_size.height);
        terminal.draw(|f| ui::render(f, app))?;

        events::handle_events(app, terminal_area)?;

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

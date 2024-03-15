use clap::Parser;
use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use hex_patch::{app::App, args};
use ratatui::backend::CrosstermBackend;

fn main() {

    let args = args::Args::parse();

    enable_raw_mode().expect("Failed to enable raw mode");
    let mut stdout = std::io::stdout();
    execute!(stdout,
        EnterAlternateScreen, 
        EnableMouseCapture
    ).expect("Failed to execute setup commands");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend).expect("Failed to create terminal");

    terminal.clear().expect("Failed to clear terminal");
    let mut app = App::new(args.file, &mut terminal).expect("Failed to create app");
    let res = app.run(&mut terminal);
    terminal.clear().expect("Failed to clear terminal");
    
    disable_raw_mode().expect("Failed to disable raw mode");

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).expect("Failed to execute teardown commands");
    terminal.show_cursor().expect("Failed to show cursor");

    if let Err(err) = res {
        println!("{:?}", err)
    }
}

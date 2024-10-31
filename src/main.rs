use std::path::{Path, PathBuf};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem},
    layout::{Layout, Constraint, Direction},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType},
};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    //println!("Hello, world!");
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // get current directory
    // match env::current_dir(){
        // Ok(current_dir) => println!("Current directory: {}", current_dir.display()),
        // Err(e) => eprintln!("Error: unable to retrieve directory: {}", e),
    // }
    
    let curr_dir = std::env::current_dir()?;
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    loop {
        
    
        terminal.draw(|f| {
            let files = peek_dir(&curr_dir);
            let items: Vec<ListItem> = files
                .iter()
                .map(|file| ListItem::new(file.display().to_string()))
                .collect();

            let list = List::new(items)
                .block(Block::default().title("Dired-like File Manager").borders(Borders::ALL));

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            f.render_widget(list, layout[0]);
        })?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break, // Quit
                //KeyCode::Char('d') => delete_file(&current_dir)?,
                //KeyCode::Char('c') => copy_file(&current_dir)?,
                //KeyCode::Enter => enter_directory(&mut current_dir)?,
                _ => {}
            }
        }
    }
    
    //println!("Current directory: {}", dirt_path.display());
    Ok(())
}

fn list_dir(path: &str){
    println!("\nListing contents of: {}", path);
    match std::fs::read_dir(path){
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  {}", entry.path().display());
                }
            }
        }
        Err(e) => eprintln!("Error: reading dir {}", e),
    }
}

fn peek_dir(path: &Path) -> Vec<PathBuf>{
    std::fs::read_dir(path)
    .unwrap()
    .filter_map(Result::ok)
    .map(|entry| entry.path())
    .collect()
}

// fn del_file(path: &Path) -> Result<(), std::io::Error> {
    // return Result<(), Error>;
// }


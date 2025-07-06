use std::{
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

pub fn run(root_path: &str) {
    let mut target_folders: Vec<PathBuf> = Vec::new();
    find_target_paths(root_path, &mut target_folders);

    if confirm_deletion() {
        println!("\nDeleting...");
        delete_folders(&target_folders);
    } else {
        println!("\nExiting without deletion.");
    };
}

pub fn run_tui() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let root = select_root(&mut terminal)?;
    let mut targets: Vec<PathBuf> = Vec::new();
    find_target_paths(&root, &mut targets);

    let chosen = select_targets(&mut terminal, &targets)?;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    if chosen.is_empty() {
        println!("No folders selected. Exiting without deletion.");
    } else if confirm_deletion() {
        println!("\nDeleting...");
        delete_folders(&chosen);
    } else {
        println!("\nExiting without deletion.");
    }
    Ok(())
}

fn confirm_deletion() -> bool {
    println!("Do you want to delete all these folders?\nTo confirm type [y]:");
    let mut user_response = String::new();
    std::io::stdin()
        .read_line(&mut user_response)
        .expect("Invalid input");
    user_response = user_response.trim().to_owned();

    user_response == "y" || user_response == "Y"
}

fn delete_folders(target_folders: &Vec<PathBuf>) {
    for path in target_folders {
        fs::remove_dir_all(path).expect("Cannot remove folders");
    }
}

fn find_target_paths<P: AsRef<Path>>(path: P, paths: &mut Vec<PathBuf>) {
    for folder in sub_folders(path) {
        if folder.file_name() == "target" {
            let target_path = folder.path().to_owned();
            println!("{}", &target_path.display());
            paths.push(target_path);
        } else {
            find_target_paths(folder.path(), paths);
        }
    }
}

fn sub_folders<P: AsRef<Path>>(path: P) -> Vec<DirEntry> {
    fs::read_dir(path)
        .expect("Invalid path.")
        .flatten()
        .filter(|entry| {
            // let is_dir = entry.file_type().unwrap().is_dir();
            // let is_read_only = entry.metadata().unwrap().permissions().readonly();
            // is_dir && !is_read_only
            entry.file_type().unwrap().is_dir()
        })
        .collect()
}

fn select_root(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<PathBuf> {
    let mut folders = sub_folders(".");
    folders.sort_by_key(|f| f.file_name());
    let mut state = ListState::default();
    if !folders.is_empty() {
        state.select(Some(0));
    }

    loop {
        terminal.draw(|f| {
            let items: Vec<ListItem> = folders
                .iter()
                .map(|e| ListItem::new(e.file_name().to_string_lossy().to_string()))
                .collect();
            let list = List::new(items)
                .block(
                    Block::default()
                        .title("Select root folder")
                        .borders(Borders::ALL),
                )
                .highlight_symbol("-> ");
            f.render_stateful_widget(list, f.size(), &mut state);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        if let Some(selected) = state.selected() {
                            let new = if selected > 0 {
                                selected - 1
                            } else {
                                folders.len() - 1
                            };
                            state.select(Some(new));
                        }
                    }
                    KeyCode::Down => {
                        if let Some(selected) = state.selected() {
                            let new = if selected + 1 < folders.len() {
                                selected + 1
                            } else {
                                0
                            };
                            state.select(Some(new));
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(selected) = state.selected() {
                            return Ok(folders[selected].path());
                        }
                    }
                    KeyCode::Char('q') => return Ok(PathBuf::from(".")),
                    _ => {}
                }
            }
        }
    }
}

fn select_targets(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    targets: &Vec<PathBuf>,
) -> io::Result<Vec<PathBuf>> {
    let mut state = ListState::default();
    if !targets.is_empty() {
        state.select(Some(0));
    }
    let mut selected: Vec<bool> = vec![false; targets.len()];

    loop {
        terminal.draw(|f| {
            let items: Vec<ListItem> = targets
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let mark = if selected[i] { "[x]" } else { "[ ]" };
                    ListItem::new(format!("{mark} {}", p.display()))
                })
                .collect();
            let list = List::new(items)
                .block(
                    Block::default()
                        .title("Select targets - d to delete, q to quit")
                        .borders(Borders::ALL),
                )
                .highlight_symbol("-> ");
            f.render_stateful_widget(list, f.size(), &mut state);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        if let Some(sel) = state.selected() {
                            let new = if sel > 0 { sel - 1 } else { targets.len() - 1 };
                            state.select(Some(new));
                        }
                    }
                    KeyCode::Down => {
                        if let Some(sel) = state.selected() {
                            let new = if sel + 1 < targets.len() { sel + 1 } else { 0 };
                            state.select(Some(new));
                        }
                    }
                    KeyCode::Char(' ') => {
                        if let Some(sel) = state.selected() {
                            selected[sel] = !selected[sel];
                        }
                    }
                    KeyCode::Char('d') => {
                        let chosen: Vec<PathBuf> = targets
                            .iter()
                            .cloned()
                            .enumerate()
                            .filter_map(|(i, p)| if selected[i] { Some(p) } else { None })
                            .collect();
                        return Ok(chosen);
                    }
                    KeyCode::Char('q') => return Ok(Vec::new()),
                    _ => {}
                }
            }
        }
    }
}

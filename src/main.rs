use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use std::env;

use std::fs;

fn read(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
          
    let mut input = String::new();
    let mut cursor_pos = 0; 
    
    let mut exit = false; // Must be mut to change it!
    enable_raw_mode().unwrap();

    // Loop runs while exit is NOT true
    while !exit {
        if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Char(c) => {
                    // Insert at cursor position so we can type in the middle
                    input.insert(cursor_pos, c);
                    
                    // Visuals: Print the rest of the string from this point
                    print!("{}", &input[cursor_pos..]);
                    cursor_pos += 1;
                    
                    // Move cursor back to where it should be if we typed in the middle
                    let back = input.len() - cursor_pos;
                    if back > 0 {
                        print!("\x1b[{}D", back);
                    }
                }
                KeyCode::Backspace => {
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                        input.remove(cursor_pos);
                        
                        // \x1b[D = move left 1
                        // \x1b[s = save cursor position
                        // \x1b[K = clear everything to the right
                        print!("\x1b[D\x1b[s\x1b[K");
                        
                        // Print the shifted text from the new cursor pos
                        print!("{}", &input[cursor_pos..]);
                        
                        // \x1b[u = jump back to the saved spot
                        print!("\x1b[u");
                    }
                }
                KeyCode::Left => {
                    if cursor_pos > 0 {
                        print!("\x1b[D");
                        cursor_pos -= 1;
                    }
                }
                KeyCode::Right => {
                    if cursor_pos < input.len() {
                        print!("\x1b[C");
                        cursor_pos += 1;
                    }
                }
                KeyCode::Enter => {
                    println!("\r");
                    exit = true; // This now breaks the 'while' loop
                }
                _ => {}
            }
            let _ = io::stdout().flush();
        }
    }

    disable_raw_mode().unwrap();
    input
}

fn main() {
    enable_raw_mode().unwrap();

    print!("\x1B[2J\x1B[H");

    loop {
        let input = read("\x1b[48;5;208m -> \x1b[0m ");
        let trimmed = input.trim();
        
        if trimmed == "exit" || trimmed == "quit" {
            break;
        } 

        let mut parts = trimmed.split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        match command {
            "ls" => {
                for entry in fs::read_dir(".").unwrap() {
                    let entry = entry.unwrap();
                    println!("{}", entry.path().display());
                }
            }
            "clear" => {
                print!("\x1B[2J\x1B[H");
            }

            "cd" => {
                let path = args.get(0).unwrap_or(&"/");
                if let Err(e) = env::set_current_dir(path) {
                    eprintln!("cd: {}", e);
                }
            }
            "echo" => {
                println!("{}", args.join(" "));
            }
            _ => {
                if !trimmed.is_empty() {
                    // This is where you would normally execute external processes
                }
            }
        }
        
        let _ = io::stdout().flush();
    }

    disable_raw_mode().unwrap();
}

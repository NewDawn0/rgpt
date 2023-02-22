/* Imports & modules*/
use crate::common::*;
use std::io::{stdout, Write};
use crossterm::{event, terminal};

/* fn read_stdin: reads a line from stdin
 * @RVAL: String */
pub fn read_stdin() -> String {
    let mut input = String::new();
    let mut stdout = stdout();

    // Enable raw mode to capture special keys like arrow keys
    terminal::enable_raw_mode().expect("Could not enable raw mode");
    loop {
        // Read next event from user input
        let event = event::read().expect("Could not read event");
        match event {
            event::Event::Key(event) => {
                match event.code {
                    event::KeyCode::Backspace => {
                        input.pop();
                        print!("\x08 \x08");
                        stdout.flush().expect("Could not flush stdout");
                    },
                    event::KeyCode::Enter => {
                        print!("\r\n");
                        stdout.flush().expect("Could not flush stdout");
                        break;
                    },
                    event::KeyCode::Char(c) => {
                        input.push(c);
                        print!("{}", c);
                        stdout.flush().expect("Could not flush stdout");
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }
    // Disable raw mode before returning the input
    terminal::disable_raw_mode().expect("Could not disable raw mode");
    input
}

/* fn confirm: get confirmation from user
 * @RVAL: bool*/
pub fn confirm() -> bool {
    loop {
        print!("{}>{} Confirm run: [y/N]: ", COLOURS.red, COLOURS.reset);
        match read_stdin().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Invalid option => aborting");
                return false 
            }
        }
    }
}

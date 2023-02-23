/*                  _
 *  _ __ __ _ _ __ | |_  A cli chat gpt client 
 * | '__/ _` | '_ \| __| Author: NewDawn0
 * | | | (_| | |_) | |_  License: MIT
 * |_|  \__, | .__/ \__| Copyright: ©NewDawn0 2023
 *      |___/|_| https://github.com/NewDawn0/rgpt
 *
 *  file: io.rs
 *  desc: All the things requring io from stdin
 *  date: 22.02.2023
 *  lang: rust
*/

/* Imports & modules*/
use crate::common::*;
use std::io::{stdout, Write};
use crossterm::{event, terminal};

/* fn read_stdin: reads a line from stdin
 * @RVAL: String */
pub fn read_stdin() -> String {
    let mut input = String::new();
    let mut stdout = stdout();
    let mut del_size: usize = 0;

    // Enable raw mode to capture special keys like arrow keys
    terminal::enable_raw_mode().expect("Could not enable raw mode");
    loop {
        // Read next event from user input
        let event = event::read().expect("Could not read event");
        match event {
            event::Event::Key(event) => {
                match event.code {
                    event::KeyCode::Backspace => {
                        if del_size > 0 {
                            input.pop();
                            print!("\x08 \x08");
                            stdout.flush().expect("Could not flush stdout");
                            del_size -= 1;
                        }
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
                        del_size += 1;
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
        std::io::stdout().flush().expect("Could not flush stdout"); // flush
        match read_stdin().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Invalid option:");
            }
        }
    }
}

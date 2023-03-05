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
use crate::util::set_mode;
use std::io::{stdout, Write};
use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers, KeyEvent, KeyEventKind, KeyEventState},
    terminal
};

/* fn read_stdin: reads a line from stdin
 * @RVAL: String */
pub fn read_stdin(hist: &mut Vec<String>) -> String {
    let mut hist_index = hist.len();
    let mut input = String::new();
    let mut stdout = stdout();
    let mut del_size: usize = 0;

    // Enable raw mode to capture special keys like arrow keys
    terminal::enable_raw_mode().expect("Could not enable raw mode");
    loop {
        // Read next event from user input
        let event = read().expect("Could not read event");
        match event {
            /* Exit using Ctrl-c or Ctrl-q
             * ctrl-c */
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            // Ctrl - q
            }) | Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            }) => {
                input.clear();
                print!("\r");
                stdout.flush().expect("Could not flush stdout");
                break;
            },
            /* Delete using Backspace */
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            }) => {
                if del_size > 0 {
                    input.pop();
                    print!("\x08 \x08");
                    stdout.flush().expect("Could not flush stdout");
                    del_size -= 1;
                }
            },
            /* Submit using Enter*/
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            }) => {
                print!("\r\n");
                stdout.flush().expect("Could not flush stdout");
                break;
            },
            /* Write using any other key
             * Lowercase keys */
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            }) => {
                input.push(c);
                print!("{}", c);
                stdout.flush().expect("Could not flush stdout");
                del_size += 1;
            },
            /* Uppercase keys*/
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            }) => {
                input.push(c);
                print!("{}", c);
                stdout.flush().expect("Could not flush stdout");
                del_size += 1;
            },
            /* Get previous command history */
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            }) => {
                if hist_index > 0 && hist_index <= hist.len() {
                    hist_index -= 1;
                    input = hist[hist_index].clone();
                    del_size = hist[hist_index].len();
                    print!("\r\x1b[2K{}>{} {}", COLOURS.red, COLOURS.reset, input);
                    stdout.flush().expect("Could not flush stdout")
                }
            },
            /* Forward command history */
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE
            }) => {
                if hist_index < hist.len()-1 {
                    input = hist[hist_index].clone();
                    del_size = hist[hist_index].len();
                    print!("\r\x1b[2K{}>{} {}", COLOURS.red, COLOURS.reset, input);
                    stdout.flush().expect("Could not flush stdout");
                    hist_index += 1
                } else {
                    input = String::new();
                    del_size = 0;
                    print!("\r\x1b[2K{}>{} {}", COLOURS.red, COLOURS.reset, input);
                }
            },
            _ => {}
        }
    }
    if !input.is_empty() {
        hist.push(input.clone());
    }
    // Disable raw mode before returning the input
    terminal::disable_raw_mode().expect("Could not disable raw mode");
    input
}

/* fn confirm: get confirmation from user
 * @RVAL: bool*/
pub fn confirm() -> bool {
    // create a temporary history
    let mut tmp_hist = Vec::<String>::new();
    loop {
        print!("{}>{} Confirm run: [y/N]: ", COLOURS.red, COLOURS.reset);
        std::io::stdout().flush().expect("Could not flush stdout"); // flush
        match read_stdin(&mut tmp_hist).to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Invalid option:");
            }
        }
    }
}

/* fn parse_io: parses a string for arguments
 * @PARAM params: crate::common::Params
 * @PARAM settings: crate::common::Settings
 * @RVAL: String */
pub fn parse_io(params:&mut Params, settings: &mut Settings, hist: &mut Vec<String>) -> String {
    let mut word_vec = vec![];
    let binding = read_stdin(hist);
    for token in binding.split_whitespace() {
        match token {
            "-c" | "--code" => set_mode(Modes::Code, params, settings),
            "-r" | "--roast" => set_mode(Modes::Shell, params, settings),
            "-s" | "--shell" => set_mode(Modes::Shell, params, settings),
            "-e" | "--execute" => params.execute = true,
            str => word_vec.push(str)
        }
    }
    word_vec.join(" ")
}

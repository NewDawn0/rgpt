/*                  _
 *  _ __ __ _ _ __ | |_  A cli chat gpt client 
 * | '__/ _` | '_ \| __| Author: NewDawn0
 * | | | (_| | |_) | |_  License: MIT
 * |_|  \__, | .__/ \__| Copyright: ©NewDawn0 2023
 *      |___/|_| https://github.com/NewDawn0/rgpt
 *
 *  file: common.rs
 *  desc: shared global variables
 *  date: 22.02.2023
 *  lang: rust
*/

/* Imports */
use const_format::formatcp;
use serde::{Serialize, Deserialize};
use std::{
    error::Error,
    fmt::{self, Display, Formatter, Debug}
};

use crate::net;

/* Errors */
/* RgptError err type */
#[derive(Debug)]
pub struct RgptError {
    error: ErrorType
}
impl RgptError {
    pub fn new(r#type: ErrorType) -> RgptError {
        RgptError {
            error: r#type
        }
    }
}
impl Error for RgptError {}
impl Display for RgptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

/* the type of errors which can occur */
#[derive(Debug)]
pub enum ErrorType {
    NetParsingErr(String),
    GetEnvErr(String)
}
impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::GetEnvErr(key) => write!(f, "Environment variable '{}' not set", key),
            ErrorType::NetParsingErr(msg) => {
                match net::parse_error(msg.as_str()) {
                    Some(e) => write!(f, "{}", e),
                    None => write!(f, "Could not parse respnse from data")
                }
            }
        }
    }
}

/* Constants 
 * The help shorthand */
pub const HELP: &'static str = formatcp!("Try '$ {}{}rgpt {}--help{}'", COLOURS.bold, COLOURS.purple, COLOURS.blue, COLOURS.reset);

/* Colours to be used */
pub const COLOURS: Colours = Colours {
    purple: "\x1b[35m",
    bold: "\x1b[1m",
    blue: "\x1b[34m",
    reset: "\x1b[0m",
    red: "\x1b[31;1m"
};

/* Helper Structs & Enums
 * Struct storing the colours */
pub struct Colours {
    pub purple: &'static str,
    pub bold: &'static str,
    pub reset: &'static str,
    pub blue: &'static str,
    pub red: &'static str
}

/* Struct storing the model settings */
#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub model: String,
    pub max_tokens: i32,
    pub temperature: f32,
    pub top_p: f32
}
impl Settings {
    pub fn new() -> Settings {
        Settings {
            model: String::from("text-davinci-003"),
            max_tokens: 1024,
            temperature: 0.2,
            top_p: 0.9
        }
    }
}

/* Flags which can be set */
#[derive(Clone, Copy)]
pub struct Params {
    pub code: bool,
    pub shell: bool,
    pub execute: bool,
    pub interactive: bool,
}
impl Params {
    pub fn new() -> Params {
        Params {
            code: false,
            shell: false,
            execute: false,
            interactive: false
        }
    }
}

/* type of float for the parse float function */
pub enum ParseFloatType {
    Temperature,
    Accuracy
}
impl Display for ParseFloatType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Accuracy => write!(f, "Accuracy"),
            Self::Temperature => write!(f, "Temperature")
        }
    }
}


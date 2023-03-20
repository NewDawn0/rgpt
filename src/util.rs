/*                  _
 *  _ __ __ _ _ __ | |_  A cli chat gpt client 
 * | '__/ _` | '_ \| __| Author: NewDawn0
 * | | | (_| | |_) | |_  License: MIT
 * |_|  \__, | .__/ \__| Copyright: ©NewDawn0 2023
 *      |___/|_| https://github.com/NewDawn0/rgpt
 *
 *  file: util.rs
 *  desc: Random useful functions
 *  date: 22.02.2023
 *  lang: rust
*/

/* Imports */
use crate::common::*;
use std::{
    env,
    process::{exit, Command},
};

/* macro error_exit: exits with a formatted error message and exit code 1 
 * @PARAM: formatting */
#[macro_export]
macro_rules! error_exit {
    ($($arg:tt)*) => ({
        eprintln!("{}Error{} :: {}", COLOURS.red, COLOURS.reset, format!($($arg)*));
        println!("{}", crate::common::HELP);
        exit(1);
    })
}

/* fn get_env: Gets an environment variable
 * @PARAM env_var: String
 * @PARAM env_var: crate::common::Param
 * @RVAL: String */
pub fn get_env(env_var: &str, params: &Params) -> Result<String, RgptError>{
    match &params.key {
        Some(key) => Ok(key.to_string()),
        None => {
            match env::var(env_var) {
                Ok(var) => Ok(var),
                Err(_) => Err(RgptError::new(ErrorType::GetEnvErr(env_var.to_string())))
            }
        }
    }
}


/* fn run_command: runs a shell command and prints the output
 * @Param cmd: &str */
pub fn run_command(cmd: &str) {
    let command = if cmd.starts_with("$ ") {
        cmd[2..].to_string()
    } else {
        cmd.to_string()
    };
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    if let Ok(e) = String::from_utf8(output.stderr) {
        eprintln!("{}",e)
    }
    match String::from_utf8(output.stdout) {
        Ok(out) => println!("{}", out),
        Err(e) => println!("{}Error{} :: {}", COLOURS.red, COLOURS.reset, e)
    }
}

/* fn parse_float: parse strs for floats and set return a single digit precision f32
 * @PARAM float_string: &str
 * @RVAL: f32 */
pub fn parse_float(float_string: &str, max: f32, typef: ParseFloatType) -> Option<f32> {
    match float_string.parse::<f32>() {
        Ok(float) => {
            if float >= 0.0 && float <= max {
                Some((float * 10.0).round() / 10.0)
            } else {
                error_exit!("{} not in range of 0.0 and {}", typef, max);
            }
        },
        Err(e) => {
            error_exit!("{}", e);
        }
    }
}

/* fn set_mode: sets settings and params for a given mode
 * @PARAM mode: crate:common::Modes
 * @PARAM params: crate::common::Params
 * @PARAM settings: crate::common::Settings */
pub fn set_mode(mode: Modes, params: &mut Params, settings: &mut Settings) {
    match mode {
        Modes::Shell => {
            params.shell = true;
            settings.temperature = 0.2;
            settings.top_p = 0.9;
        },
        Modes::Code => {
            params.code = true;
            settings.temperature = 0.8;
            settings.top_p = 0.2;
        },
        Modes::Roast => {
            params.roast = true;
            settings.temperature = 0.8;
            settings.top_p = 0.2;
        }
    }
}

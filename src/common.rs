/*                  _
 *  _ __ __ _ _ __ | |_  A cli chat gpt client 
 * | '__/ _` | '_ \| __| Author: NewDawn0
 * | | | (_| | |_) | |_  License: MIT
 * |_|  \__, | .__/ \__| Copyright: ©NewDawn0 2023
 *      |___/|_| https://github.com/NewDawn0/rgpt
 *
 *  file: common.rs
 *  lang: rust
*/

/* Constants
 * Colours to be used */
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

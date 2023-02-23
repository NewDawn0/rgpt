/*                  _
 *  _ __ __ _ _ __ | |_  A cli chat gpt client 
 * | '__/ _` | '_ \| __| Author: NewDawn0
 * | | | (_| | |_) | |_  License: MIT
 * |_|  \__, | .__/ \__| Copyright: ©NewDawn0 2023
 *      |___/|_| https://github.com/NewDawn0/rgpt
 *
 *  file: highlight.rs
 *  desc: all the things code highliting
 *  date: 22.02.2023
 *  lang: rust
*/

use syntect::{
    highlighting::ThemeSet,
    parsing::SyntaxSet,
    easy::HighlightLines,
    util::as_24_bit_terminal_escaped
};

/* fn do_highlight: set highlighting for provided String
 * @PARAM code: &str,
 * @PARAM lang: &str,
 * @RVAL: &str*/
pub fn highlight(code: &str, lang: &str) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let detected_syntax = match syntax_set.find_syntax_by_extension(lang) {
        Some(syntx) => syntx,
        None => syntax_set.find_syntax_plain_text()
    };
    let theme = &theme_set.themes["base16-eighties.dark"];
    let mut h = HighlightLines::new(detected_syntax, theme);
    let highlighted = h.highlight(&code, &syntax_set);
    
    // Convert and return
    as_24_bit_terminal_escaped(&highlighted[..], false)
}

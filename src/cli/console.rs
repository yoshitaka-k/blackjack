use crossterm::style::{Stylize};

/// 区切り線（-----）
pub fn print_single_separator() {
    print_hr('-', 50);
}

/// 区切り線（======）
pub fn print_double_separator() {
    print_hr('=', 50);
}

/// 区切り線
pub fn print_hr(symbol: char, length: usize) {
    println!("{}", symbol.to_string().repeat(length));
}

pub fn print_br() {
    println!("");
}

/// 主にシステム向けな表示
pub fn system(prompt: &str) {
    println!("{}", prompt.green());
}

/// 主にシステム向けな表示（太字）
pub fn system_bold(prompt: &str) {
    println!("{}", prompt.green().bold());
}

/// 主にお知らせ向けな表示（太字）
pub fn info(prompt: &str) {
    println!("{}", prompt.cyan());
}

/// 主にエラー向けな表示
pub fn error(prompt: &str) {
    println!("{}", prompt.red());
}

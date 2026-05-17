use std::io::{self};
use regex::Regex;

use crossterm::{
    style::{Stylize},
};
use rustyline::{
    completion::Completer,
    config::{CompletionType, Config},
    error::ReadlineError,
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
    history::DefaultHistory,
    Context, Editor, Helper,
};

use crate::cli::console::{system_bold, error};

/// 入力処理（正規表現判定）
/// 正規表現の値が入力されるまで表示
pub fn input_match_read_line(prompt: &str, pattern: &str) -> String {
    input_match_read_line_with_words(prompt, pattern, &[])
}

/// 正規表現判定 + Tab 補完（候補の先頭一致）
pub fn input_match_read_line_with_words(
    prompt: &str,
    pattern: &str,
    words: &[&str],
) -> String {
    let re = Regex::new(pattern).unwrap();
    loop {
        match read_line(prompt, words) {
            Ok(line) if re.is_match(&line) => {
                break line;
            }
            Ok(_) => error("The input is not a pattern match."),
            Err(_) => error("The input empty is error."),
        }
    }
}

/// 入力処理（数値に変換）
///指定範囲内の値が入力されるまで表示
pub fn input_isize_read_line(input_msg: &str,
                        default_num: isize,
                        min_num: isize,
                        max_num: isize) -> isize {
    loop {
        match read_isize_line(
            input_msg,
            default_num,
        ) {
            Ok(num) if (min_num..=max_num).contains(&num) => {
                break num;
            }
            Ok(_) => error(&format!("The input is not a number {}-{}.", min_num, max_num)),
            Err(_) => error("The input is not a number."),
        }
    }
}

/// 入力処理（数値に変換）
///指定範囲内の値が入力されるまで表示
pub fn input_usize_read_line(input_msg: &str,
                        default_num: usize,
                        min_num: usize,
                        max_num: usize) -> usize {
    loop {
        match read_usize_line(
            input_msg,
            default_num,
        ) {
            Ok(num) if (min_num..=max_num).contains(&num) => {
                break num;
            }
            Ok(_) => error(&format!("The input is not a number {}-{}.", min_num, max_num)),
            Err(_) => error("The input is not a number."),
        }
    }
}

/// 単語リストから Tab 補完する Helper
struct WordCompleter {
    words: Vec<String>,
}

impl WordCompleter {
    fn new(words: &[&str]) -> Self {
        Self {
            words: words.iter().map(|w| (*w).to_string()).collect(),
        }
    }
}

impl Completer for WordCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<String>)> {
        let start = line[..pos]
            .char_indices()
            .rfind(|(_, c)| c.is_whitespace())
            .map(|(i, c)| i + c.len_utf8())
            .unwrap_or(0);
        let prefix = &line[start..pos];
        let matches: Vec<String> = self
            .words
            .iter()
            .filter(|w| w.starts_with(prefix))
            .cloned()
            .collect();
        Ok((start, matches))
    }
}

impl Helper for WordCompleter {}
impl Highlighter for WordCompleter {}
impl Hinter for WordCompleter {
    type Hint = String;
}
impl Validator for WordCompleter {}

/// 入力処理（ベース）
/// `words` が空なら補完なし、非空なら Tab で候補を表示
fn read_line(prompt: &str, words: &[&str]) -> rustyline::Result<String> {
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();
    let input_tag = format!("{}", "[INPUT]".yellow());
    let full_prompt = format!("{} {} >> ", input_tag, prompt);

    if words.is_empty() {
        let mut rl = rustyline::DefaultEditor::with_config(config)?;
        readline_inner(&mut rl, &full_prompt)
    } else {
        let mut rl = Editor::<WordCompleter, DefaultHistory>::with_config(config)?;
        rl.set_helper(Some(WordCompleter::new(words)));
        readline_inner(&mut rl, &full_prompt)
    }
}

fn readline_inner<H: Helper>(
    rl: &mut Editor<H, DefaultHistory>,
    prompt: &str,
) -> rustyline::Result<String> {
    match rl.readline(prompt) {
        Ok(line) => Ok(line.trim().to_string()),
        Err(ReadlineError::Interrupted) => {
            system_bold("Pressing Ctrl+C. Ends the Game.");
            std::process::exit(0);
        }
        Err(ReadlineError::Eof) => {
            system_bold("Pressing Ctrl+D. Ends the Game.");
            std::process::exit(0);
        }
        Err(e) => Err(e),
    }
}

/// 入力したのを数値に変換
pub fn read_isize_line(prompt: &str, default: isize) -> rustyline::Result<isize> {
    let line = read_line(prompt, &[])?;

    if line.is_empty() {
        Ok(default)
    } else {
        Ok(line.parse::<isize>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?)
    }
}

/// 入力したのを数値に変換
pub fn read_usize_line(prompt: &str, default: usize) -> rustyline::Result<usize> {
    let line = read_line(prompt, &[])?;

    if line.is_empty() {
        Ok(default)
    } else {
        Ok(line.parse::<usize>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?)
    }
}

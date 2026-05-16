use std::io::{self};
use crossterm::{
    style::{Stylize},
};
use rustyline::error::ReadlineError;

use crate::cli::console::{system_bold, error};

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

/// 入力処理（ベース）
fn read_line(prompt: &str) -> rustyline::Result<String> {
    let mut rl = rustyline::DefaultEditor::new()?;
    let input_tag = format!("{}", "[INPUT]".yellow());
    let readline = rl.readline(&format!("{} {} >> ", input_tag, prompt));

    match readline {
        Ok(line) => Ok(line.trim().to_string()),
        Err(ReadlineError::Interrupted) => {
            system_bold("Pressing Ctrl+C. Ends the Game.");
            std::process::exit(0);
        },
        Err(ReadlineError::Eof) => {
            system_bold("Pressing Ctrl+D. Ends the Game.");
            std::process::exit(0);
        },
        Err(e) => Err(e),
    }
}

/// 入力したのを数値に変換
pub fn read_isize_line(prompt: &str, default: isize) -> rustyline::Result<isize> {
    let line = read_line(prompt)?;

    if line.is_empty() {
        Ok(default)
    } else {
        Ok(line.parse::<isize>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?)
    }
}

/// 入力したのを数値に変換
pub fn read_usize_line(prompt: &str, default: usize) -> rustyline::Result<usize> {
    let line = read_line(prompt)?;

    if line.is_empty() {
        Ok(default)
    } else {
        Ok(line.parse::<usize>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?)
    }
}

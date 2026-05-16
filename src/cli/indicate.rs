use std::time::Duration;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

/// 待ち中表示（スピナー）
pub fn execute_with_spinner<T, F>(set_message: &str, finish_message: &str, f: F) -> T
    where
        F: FnOnce() -> T {
    let mult = MultiProgress::new();
    let pb = mult.add(ProgressBar::new_spinner());

    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["|", "/", "-", "\\"])
    );
    pb.set_message(set_message.to_string());

    let result = f();

    if finish_message.trim().len() > 0 {
        // 終わったらメッセージを変える
        pb.finish_with_message(finish_message.to_string());
    } else {
        // 終わったらバーを消す
        pb.finish_and_clear();
    }

    result
}

/// 待ち中表示（プログレスバー）
pub fn execute_with_progress<T, F>(total: u64, set_message: &str, finish_message: &str, f: F) -> T
    where
        F: FnOnce(&ProgressBar) -> T {
    let mult = MultiProgress::new();
    let pb = mult.add(ProgressBar::new(total));

    pb.set_style(
        ProgressStyle::with_template("{msg} [{wide_bar:.green}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    pb.set_message(set_message.to_string());

    let result = f(&pb);

    if finish_message.trim().len() > 0 {
        // 終わったらメッセージを変える
        pb.finish_with_message(finish_message.to_string());
    } else {
        // 終わったらバーを消す
        pb.finish_and_clear();
    }

    result
}

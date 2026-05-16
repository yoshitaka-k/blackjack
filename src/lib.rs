/// 設定値とか
pub mod constants;
/// ターミナルに表示するもの
pub mod cli;
/// 処理
pub mod logic;
/// トランプカード、プレイヤー、フィールド情報
pub mod trump;
/// ゲーム進行の処理
pub mod game;

pub use cli::console::{error};
pub use cli::input::{input_usize_read_line};
pub use trump::{Card, Deck, Field, Player};

use rand::RngExt;

/// 表示させた後少し待機
pub fn wait_for_dramatic_pause() {
    std::thread::sleep(std::time::Duration::from_millis(200));
}

/// 表示させた後少し待機
pub fn wait_for_long_dramatic_pause() {
    std::thread::sleep(std::time::Duration::from_millis(3000));
}

/// 真ん中あたりの位置を取得（少しだけランダム）
pub fn get_center_position(cards_len: usize) -> usize {
    if cards_len == 0 {
        return 0;
    }
    let mut rng = rand::rng();
    let base = cards_len / 2;
    let jitter = (cards_len / 10).max(1);
    (base as isize + rng.random_range(-(jitter as i64)..=(jitter as i64)) as isize)
        .clamp(0, cards_len as isize - 1) as usize
}

/// 頭文字だけ大文字にする簡単な関数っ
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// 麻雀のサイ振り的な処理
pub fn dice_role() -> usize {
    let dice1 = rand::rng().random_range(1..=6);
    let dice2 = rand::rng().random_range(1..=6);

    dice1 + dice2
}

/// 起家指定
pub fn init_current_player(current: usize, players_count: usize) -> usize {
    let dice = dice_role();
    println!("{}", format!("  ->Dice Result: {}", dice));

    (current + (dice - 1)) % players_count
}

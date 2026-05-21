use std::error::Error;

use crate::constants::{
    START_CHIP,
    MIN_CHIP,
    DEFAULT_CHIP,
    CALL_HIT,
    CALL_STAND,
    CALL_WORDS,
};
use crate::cli::{
    input::{
        input_isize_read_line,
        input_match_read_line_with_words,
    },
    print_display::{hand_display_one},
};
use crate::trump::{Player};

/// ユーザー処理
pub struct Human();
impl Human {
    /// 賭けチップ入力
    pub fn bet(player: &Player) -> Result<isize, Box<dyn Error>> {
        let mut max = player.get_chip();
        if max <= 0 {
            max = START_CHIP;
        }

        let bet = input_isize_read_line(
            &format!(
                "Input: {}-{} (Default: {})",
                MIN_CHIP,
                max,
                DEFAULT_CHIP
            ),
            DEFAULT_CHIP,
            MIN_CHIP,
            max,
        );

        Ok(bet)
    }

    /// コール入力
    pub fn call(player: &Player) -> Result<String, Box<dyn Error>> {
        hand_display_one(player, true);

        let call = input_match_read_line_with_words(
            &format!("{} or {}? [Tab]", CALL_HIT, CALL_STAND),
            &format!(r"^({})$", CALL_WORDS.join("|")),
            CALL_WORDS,
        );

        Ok(call)
    }
}

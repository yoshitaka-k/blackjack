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
    pub fn bet(player: &Player) -> isize {
        let mut max = player.get_chip();
        if max <= 0 {
            max = START_CHIP;
        }

        input_isize_read_line(
            &format!(
                "Input: {}-{} (Default: {})",
                MIN_CHIP,
                max,
                DEFAULT_CHIP
            ),
            DEFAULT_CHIP,
            MIN_CHIP,
            max,
        )
    }

    /// コール入力
    pub fn call(player: &Player) -> String {
        hand_display_one(player, true);

        input_match_read_line_with_words(
            &format!("{} or {}? [Tab]", CALL_HIT, CALL_STAND),
            &format!(r"^({})$", CALL_WORDS.join("|")),
            CALL_WORDS,
        )
    }
}

use rand::RngExt;

use crate::constants::{
    START_CHIP,
    MIN_CHIP,
    CALL_HIT,
    CALL_STAND,
    // CALL_WORDS,
    CPU_STAND_LINE,
};
use crate::trump::{Player};

/// CPU処理
pub struct Cpu();
impl Cpu {
    /// 賭けチップ入力
    pub fn bet(player: &Player) -> isize {
        let bet: isize;

        if player.get_chip() > 1 {
            // 1から所持数の半分まで、小数点以下は切り捨て
            let max = player.get_chip().div_euclid(2) as i32;
            bet = rand::rng().random_range(MIN_CHIP as i32..=max) as isize;
        } else  if player.get_chip() < 1 {
            bet = rand::rng().random_range(MIN_CHIP as i32..=START_CHIP as i32) as isize;
        } else {
            bet = MIN_CHIP;
        }

        bet
    }

    /// コール入力
    pub fn call(player: &Player) -> String {
        let input: String;

        if player.rank_sum() > CPU_STAND_LINE {
            input = CALL_STAND.to_string();
        } else {
            input = CALL_HIT.to_string();
        }

        input
    }
}

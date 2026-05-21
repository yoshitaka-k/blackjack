use std::error::Error;

use rand::RngExt;

use crate::constants::{
    START_CHIP,
    MIN_CHIP,
    CALL_HIT,
    CALL_STAND,
};
use crate::logic::cpulib::strategy::CpuStrategy;
use crate::trump::{Player};

const CPU_STAND_LINE: usize = 18;

/// 強さ指定なし
pub struct BeginnerStrategy;
impl CpuStrategy for BeginnerStrategy {
    /// ベット掛け方
    fn bet(&self, player: &mut Player) -> Result<isize, Box<dyn Error>> {
        let bet: isize;

        if player.get_chip() > 1 {
            // 1から所持数の80%分まで、小数点以下は切り捨て
            let max = (player.get_chip() as f64 * 0.8).floor() as i32;
            bet = rand::rng().random_range(MIN_CHIP as i32..=max) as isize;
        } else  if player.get_chip() < 1 {
            bet = rand::rng().random_range(MIN_CHIP as i32..=START_CHIP as i32) as isize;
        } else {
            bet = MIN_CHIP;
        }

        Ok(bet)
    }

    /// 相手のカードを引く場所
    fn call(&self, player: &mut Player) -> Result<String, Box<dyn Error>> {
        let input: String;

        if player.rank_sum() > CPU_STAND_LINE {
            input = CALL_STAND.to_string();
        } else {
            input = CALL_HIT.to_string();
        }

        Ok(input)
    }
}

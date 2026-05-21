use std::error::Error;

use crate::Player;

/// CPUの強さ
pub trait CpuStrategy {
    /// ベットの掛け方
    fn bet(&self, player: &mut Player) -> Result<isize, Box<dyn Error>>;

    /// コールの仕方
    fn call(&self, player: &mut Player) -> Result<String, Box<dyn Error>>;
}

use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::logic::cpulib::{
    strategy::CpuStrategy,
    default::DefaultStrategy,
};
use crate::trump::{Player, PlayerType};

/// CPU強さ
#[derive(Clone, Deserialize, Serialize)]
pub enum CpuLevel {
    None,
}

impl CpuLevel {
    const PREFIX: &'static str = "Cpu:";

    pub fn as_str(&self) -> &'static str {
        match self {
            CpuLevel::None => "None",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "None" => Some(CpuLevel::None),
            _ => None,
        }
    }

    pub fn encode_player_type(level: &CpuLevel) -> String {
        format!("{}{}", Self::PREFIX, level.as_str())
    }

    pub fn decode_player_type(value: &str) -> Option<CpuLevel> {
        value.strip_prefix(Self::PREFIX).and_then(Self::from_str)
    }
}

/// CPU処理
pub struct Cpu();
impl Cpu {
    /// 強さ選択
    fn get_strategy(player_type: &PlayerType) -> Box<dyn CpuStrategy> {
        match player_type {
            PlayerType::Human => Box::new(DefaultStrategy),
            PlayerType::Cpu(level) => {
                match level {
                    CpuLevel::None => Box::new(DefaultStrategy),
                }
            }
        }
    }

    /// 賭けチップ入力
    pub fn bet(player: &mut Player) -> Result<isize, Box<dyn Error>> {
        let player_type = player.get_player_type();
        let strategy = Self::get_strategy(player_type);
        strategy.bet(player)
    }

    /// コール入力
    pub fn call(player: &mut Player) -> Result<String, Box<dyn Error>> {
        let player_type = player.get_player_type();
        let strategy = Self::get_strategy(player_type);
        strategy.call(player)
    }
}

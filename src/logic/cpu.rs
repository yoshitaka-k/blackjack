use std::error::Error;
use rand::prelude::IndexedRandom;

use serde::{Deserialize, Serialize};

use crate::logic::cpulib::{
    strategy::CpuStrategy,
    default::DefaultStrategy,
    beginner::BeginnerStrategy,
};
use crate::trump::{Player, PlayerType};

/// CPU強さグループ
#[derive(Copy, Clone)]
pub enum CpuLevelGroup {
    None,
    Beginner,
}

/// CPU強さ
#[derive(Clone, Deserialize, Serialize)]
pub enum CpuLevel {
    None,
    Beginner,
}

impl CpuLevel {
    const PREFIX: &'static str = "Cpu:";

    pub fn as_str(&self) -> &'static str {
        match self {
            CpuLevel::None => "None",
            CpuLevel::Beginner => "Beginner",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "None" => Some(CpuLevel::None),
            "Beginner" => Some(CpuLevel::Beginner),
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
    /// CPUの強さグループ
    /// 6分の1でCPUの強さを決める
    fn level_choices(level_group: &CpuLevelGroup) -> [CpuLevel; 2] {
        match level_group {
            CpuLevelGroup::None => [
                CpuLevel::None, CpuLevel::None,
            ],
            CpuLevelGroup::Beginner => [
                CpuLevel::None, CpuLevel::Beginner,
            ],
        }
    }

    /// CPUの強さ設定
    /// 強さグループから6分の1でCPUの強さを決める
    pub fn new_level(level_group: &CpuLevelGroup) -> CpuLevel {
        let choices = Self::level_choices(&level_group);
        choices.choose(&mut rand::rng()).unwrap().clone()
    }

    /// 強さ選択
    fn get_strategy(player_type: &PlayerType) -> Box<dyn CpuStrategy> {
        match player_type {
            PlayerType::Human => Box::new(DefaultStrategy),
            PlayerType::Cpu(level) => {
                match level {
                    CpuLevel::None => Box::new(DefaultStrategy),
                    CpuLevel::Beginner => Box::new(BeginnerStrategy),
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

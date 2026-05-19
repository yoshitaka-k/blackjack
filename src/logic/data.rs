use std::{
    path::Path,
    fs::File,
    error::Error,
};
use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};

use crate::constants::CSV_PATH;
use crate::trump::{Player, PlayerType, PlayerRole};

#[derive(Clone, Deserialize, Serialize)]
pub struct Record {
    name: String,
    player_type: PlayerType,
    role: PlayerRole,
    chip: isize,
}
impl Record {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn player_type(&self) -> &PlayerType {
        &self.player_type
    }
    pub fn role(&self) -> &PlayerRole {
        &self.role
    }
    pub fn chip(&self) -> &isize {
        &self.chip
    }
}

/// 外部ファイルに読み書き
pub struct Data();
impl Data {
    /// 空ファイル作成
    fn crate_empty_csv() -> Result<(), Box<dyn Error>> {
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .from_path(CSV_PATH)?;

        wtr.write_record(&["name", "chip"])?;
        wtr.flush()?;

        Ok(())
    }

    /// 読み込み
    pub fn load() -> Result<Vec<Record>, Box<dyn Error>> {
        if !Path::new(CSV_PATH).exists() {
            Self::crate_empty_csv()?;
            return Ok(Vec::new());
        }

        let file = File::open(CSV_PATH)?;
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut records= Vec::new();
        for result in rdr.deserialize() {
            records.push(result?);
        }

        Ok(records)
    }

    /// 書き込み
    pub fn save(players: &Vec<Player>) -> Result<(), Box<dyn Error>> {
        let mut wtr = WriterBuilder::new().from_path(CSV_PATH)?;

        for player in players {
            wtr.serialize(Record {
                name: player.get_name().to_string(),
                player_type: player.get_player_type().clone(),
                role: player.get_player_role().clone(),
                chip: player.get_chip(),
            })?;
        }

        wtr.flush()?;

        Ok(())
    }
}

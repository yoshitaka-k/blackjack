use serde::{Deserialize, Serialize};

use crate::constants::{
    START_CHIP,
    ACE_FROM_RANK,
    ACE_TO_RANK,

    CALL_HIT,
    CALL_STAND,
};

use crate::trump::{Card};
use crate::logic::{Record};

/// プレイヤー、CPU判断
#[derive(Clone, Deserialize, Serialize)]
pub enum PlayerType {
    Human,
    Cpu,
}

/// ディーラー、プレイヤー判断
#[derive(Clone, Deserialize, Serialize)]
pub enum PlayerRole {
    Dealer,
    Player,
}

/// プレイヤー状態
pub enum PlayerStatus {
    None,
    Hit,
    Stand,
    Burst,
}

/// 手札
pub struct CardSet(Vec<Card>);
impl CardSet {
    fn add(&mut self, card: Card) {
        self.0.push(card);
    }

    fn get(&self) -> &Vec<Card> {
        &self.0
    }

    fn rank_sum(&self) -> usize {
        let mut ace = 0;
        let mut total = 0;

        for card in &self.0 {
            if ACE_FROM_RANK == card.get_calc_rank() {
                ace = card.get_calc_rank();
            }
            total = total + card.get_calc_rank();
        }

        if ace > 0 && total - ace < 11 {
            total = total - ace + ACE_TO_RANK;
        }

        total
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

/// プレイヤー
pub struct Player {
    name: String,
    player_type: PlayerType,
    role: PlayerRole,
    hand: CardSet,
    chip: isize,
    bet: usize,
    status: PlayerStatus,
}

impl Player {
    /// プレイヤー構造体作成
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            player_type: PlayerType::Human,
            role: PlayerRole::Player,
            hand: CardSet(vec![]),
            chip: START_CHIP,
            bet: 0,
            status: PlayerStatus::None,
        }
    }

    /// プレイヤー構造体読み込み
    pub fn load(data: &Record) -> Self {
        Self {
            name: data.name().clone(),
            player_type: data.player_type().clone(),
            role: data.role().clone(),
            hand: CardSet(vec![]),
            chip: *data.chip(),
            bet: 0,
            status: PlayerStatus::None,
        }
    }

    /// プレイヤー名取得
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// 役割設定
    pub fn set_player_role(&mut self, role: PlayerRole) {
        self.role = role;
    }

    /// 役割取得
    pub fn get_player_role(&self) -> &PlayerRole {
        &self.role
    }

    /// 種類設定
    pub fn set_player_type(&mut self, player_type: PlayerType) {
        self.player_type = player_type;
    }

    /// 種類取得
    pub fn get_player_type(&self) -> &PlayerType {
        &self.player_type
    }

    /// ユーザー操作？
    pub fn is_human(&self) -> bool {
        match self.player_type {
            PlayerType::Human => true,
            PlayerType::Cpu => false,
        }
    }

    /// 手札にカードを1枚追加
    pub fn add_hand(&mut self, card: Card) {
        self.hand.add(card);
    }

    /// 手札参照
    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand.get()
    }

    /// 手札ランク合計
    pub fn rank_sum(&self) -> usize {
        self.hand.rank_sum()
    }

    /// 所持チップ参照
    pub fn get_chip(&self) -> isize {
        self.chip
    }

    /// ベット保持
    pub fn set_bet(&mut self, bet: usize) {
        self.bet = bet;
    }

    /// ベット初期値
    pub fn clear_bet(&mut self) {
        self.bet = 0;
    }

    /// 所持チップ変動
    pub fn update_chip(&mut self, is_win: bool) {
        if self.bet == 0 {
            return;
        }

        if is_win {
            self.chip = self.chip + self.bet as isize;
        } else {
            self.chip = self.chip + (self.bet as isize * -1);
        }
        self.bet = 0;
    }

    /// 状態保持
    pub fn set_call(&mut self, call: &str) {
        match call {
            CALL_HIT => {
                self.status = PlayerStatus::Hit;
            },
            CALL_STAND => {
                self.status = PlayerStatus::Stand;
            },
            _ => {
                self.status = PlayerStatus::None;
            }
        }
    }

    /// ヒット状態
    pub fn hit(&mut self) {
        self.status = PlayerStatus::Hit;
    }

    /// スタンド状態
    pub fn stand(&mut self) {
        self.status = PlayerStatus::Stand;
    }

    /// バースト状態
    pub fn burst(&mut self) {
        self.status = PlayerStatus::Burst;
    }

    /// 状態参照
    pub fn get_status(&self) -> &PlayerStatus {
        &self.status
    }

    /// 初期化
    pub fn clear(&mut self) {
        self.hand.clear();
        self.bet = 0;
        self.status = PlayerStatus::None;
    }
}

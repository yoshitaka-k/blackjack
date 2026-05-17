use std::{
    thread,
    time::Duration,
};
use rand::RngExt;

use crossterm::{
    style::{Stylize},
};

use crate::constants::{
    MIN_CHIP,
    DEFAULT_CHIP,
};
use crate::cli::{
    indicate::{execute_with_spinner},
    input::{
        input_isize_read_line,
        input_match_read_line_with_words,
    },
};
use crate::trump::shuffle::{
    double_cut,
    hindu_shuffle,
    riffle_shuffle,
    deal_shuffle,
    HinduParams,
    RiffleParams,
    DealParams,
};
use crate::{
    init_current_player,
    wait_for_dramatic_pause,
};

use crate::trump::{Card, Deck, Player, PlayerType, PlayerRole};

/// ゲームフロー毎の処理
pub struct GameSession();
impl GameSession {
    pub fn new() -> Self { Self() }

    /// 山札を切る
    pub fn shuffle(&self, cards: &mut Vec<Card>) {
        // Deck Shuffle.
        execute_with_spinner(
            &format!("Deck setup and shuffle..."),
            &format!("Deck setup and shuffle end."),
        || {
            hindu_shuffle(cards, &HinduParams::default());
            riffle_shuffle(cards, &RiffleParams::default());
            deal_shuffle(cards, &DealParams::default());
            double_cut(cards);
        });
    }

    /// ディーラー
    pub fn dealer_setup(&self, ) -> Player{
        print!("{} ", "/".green());
        println!("Dealer setup.");

        let mut dealer = Player::new("Dealer");

        dealer.set_player_type(PlayerType::Cpu);
        dealer.set_player_role(PlayerRole::Dealer);

        dealer
    }

    /// 参加プレイヤー
    pub fn players_setup(&self, cpu_count: usize) -> Vec<Player> {
        print!("{} ", "/".green());
        println!("Players setup.");

        let mut players: Vec<Player> = Vec::new();

        players.push(Player::new("Player"));

        for i in 1..=cpu_count {
            let mut player = Player::new(&format!("CPU {}", i));
            player.set_player_type(PlayerType::Cpu);

            players.push(player);
        }

        players
    }

    /// 起家決め
    pub fn starting_setup(&self, players: &Vec<Player>) -> usize {
        println!("Starting Player");

        let players_count = players.len();
        let temp_current = rand::rng().random_range(0..players_count);

        wait_for_dramatic_pause();

        println!("Pre-Roller: {}", format!("{}", players[temp_current].get_name()).green());
        let dice_current = init_current_player(temp_current, players_count);

        wait_for_dramatic_pause();

        println!("Deciding Roller: {}", format!("{}", players[dice_current].get_name()).green());
        let current = init_current_player(dice_current, players_count);

        wait_for_dramatic_pause();

        println!("Starting Player: {}", format!("{}", players[current].get_name()).green());

        current
    }

    /// 賭けチップ入力
    pub fn input_chip(&self, player: &mut Player) {
        let chip = input_isize_read_line(
            &format!(
                "Input: {}-{} (Default: {})",
                MIN_CHIP,
                player.get_chip(),
                DEFAULT_CHIP
            ),
            DEFAULT_CHIP,
            MIN_CHIP,
            player.get_chip(),
        );

        println!("  >> Bet {}", chip);
        player.set_bet(chip);
    }

    /// 1人に手札配り
    fn deal_one(&self, deck: &mut Deck, target: &mut Player) {
        if let Some(card) = deck.draw() {
            target.add_hand(card);
        }
        thread::sleep(Duration::from_millis(rand::rng().random_range(10..=100)));
    }

    /// 手札配り
    pub fn deal_setup(&self, deck: &mut Deck, current: usize, players: &mut Vec<Player>, dealer: &mut Player) {
        let n = players.len();

        execute_with_spinner(
            "Deal the cards.",
            "Deal the cards end.",
        || {
            for _round in 0..2 {
                for i in 0..n {
                    let idx = (current + i) % n;
                    self.deal_one(deck, &mut players[idx]);
                }
                self.deal_one(deck, dealer);
            }
        });
    }

    /// コール入力
    pub fn input_call(&self, current: usize, deck: &mut Deck, players: &mut Vec<Player>) {
        let input = input_match_read_line_with_words(
            "hit or stand? [Tab]",
            r"^(hit|stand)$",
            &["hit", "stand"],
        );
        println!("{}", input);
    }
}

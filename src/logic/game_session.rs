use rand::RngExt;

use crossterm::{
    style::{Stylize},
};

use crate::constants::{
    CPU_COUNT,
    TWENTY_ONE_NUM,
};
use crate::cli::{
    indicate::{execute_with_spinner},
    print_display::{hand_display_one},
};
use crate::logic::{Record, Human, Cpu};
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

use crate::trump::{Card, Deck, Player, PlayerType, PlayerRole, PlayerStatus};


/// ゲームフロー毎の処理
pub struct GameSession();
impl GameSession {
    pub fn new() -> Self { Self() }

    /// 山札を切る
    pub fn shuffle(&self, cards: &mut Vec<Card>) {
        // Deck Shuffle.
        execute_with_spinner(
            &format!("Deck prepare and shuffle..."),
            &format!("Deck prepare and shuffle end."),
        || {
            hindu_shuffle(cards, &HinduParams::default());
            riffle_shuffle(cards, &RiffleParams::default());
            deal_shuffle(cards, &DealParams::default());
            double_cut(cards);
        });
    }

    /// ディーラー
    pub fn dealer_setup(&self) -> Player{
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

    /// プレイヤー情報読み込み
    pub fn players_data_load(&self, load_data: &Vec<Record>, cpu_count: usize) -> Vec<Player> {
        print!("{} ", "/".green());
        println!("Players load data.");

        let mut players: Vec<Player> = Vec::new();

        if load_data.len() != CPU_COUNT + 1 {
            return self.players_setup(cpu_count);
        }

        for data in load_data {
            let player = Player::load(&data);
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
    pub fn input_bet(&self, player: &mut Player) {
        println!("Input Bet: {} ({}). ", player.get_name(), player.get_chip());

        let bet: isize;

        if player.is_human() {
            bet = Human::bet(player);
        } else {
            bet = Cpu::bet(player);
        }

        println!("  >> Bet {}", bet);
        player.set_bet(bet as usize);
    }

    /// 1人に手札配り
    fn deal_one(&self, deck: &mut Deck, target: &mut Player) {
        if let Some(card) = deck.draw() {
            target.add_hand(card);
        }
    }

    /// 手札配り
    pub fn deal_setup(&self, current: usize, deck: &mut Deck, players: &mut Vec<Player>, dealer: &mut Player) {
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

    // プレイヤーがスタンドかバーストじゃなければtrue
    pub fn has_player_call(&self, player: &Player) -> bool {
        match player.get_status() {
            PlayerStatus::None => return true,
            PlayerStatus::Hit => return true,
            _ => {},
        }
        false
    }

    /// 全員スタンドかバーストかしない限りtrue
    pub fn has_players_call(&self, players: &Vec<Player>) -> bool {
        for player in players {
            match player.get_status() {
                PlayerStatus::None => return true,
                PlayerStatus::Hit => return true,
                _ => {},
            }
        }

        false
    }

    /// コール入力
    pub fn input_call(&self, player: &mut Player) {
        println!("Input Call: {}. ", player.get_name());

        let input: String;

        if player.is_human() {
            input = Human::call(player);
        } else {
            input = Cpu::call(player);
        }

        println!("  >> Call: {}", input);
        player.set_call(&input);
    }

    /// コール処理
    pub fn call_process(&self, deck: &mut Deck, player: &mut Player) {
        match player.get_status() {
            PlayerStatus::Hit => {
                player.hit();
                self.call_hit(deck, player);
            },
            PlayerStatus::Stand => {
                player.stand();
            },
            _ => {},
        }

        if player.rank_sum() > TWENTY_ONE_NUM {
            player.burst();
        }
    }

    /// ヒット処理（手札追加）
    fn call_hit(&self, deck: &mut Deck, player: &mut Player) {
        if let Some(card) = deck.draw() {
            player.add_hand(card);
        }
    }

    /// 判定
    pub fn end_fase(&self, dealer: &Player, players: &mut Vec<Player>) {
        hand_display_one(dealer, true);
        if dealer.rank_sum() > TWENTY_ONE_NUM {
            println!("      >> {}", "Burst.".red());
        }

        for player in players {
            hand_display_one(player, true);

            self.result_judg(dealer, player);
        }
    }

    /// リザルト判定
    fn result_judg(&self, dealer: &Player, player: &mut Player) {
        match player.get_status() {
            PlayerStatus::Burst => {
                println!("      >> {}", "Burst.".red());
                player.update_chip(false);
            },
            _ => {
                if dealer.rank_sum() > TWENTY_ONE_NUM {
                    println!("      >> {}", "Win.".green());
                    player.update_chip(true);

                } else if player.rank_sum() > dealer.rank_sum() {
                    println!("      >> {}", "Win.".green());
                    player.update_chip(true);

                } else if player.rank_sum() == dealer.rank_sum() {
                    println!("      >> {}", "Push.".green());
                    player.clear_bet();

                } else {
                    println!("      >> {}", "Lose.".red());
                    player.update_chip(false);
                }
            }
        }
    }
}

use crossterm::{
    style::{Stylize},
};

use crate::constants::{CPU_COUNT};
use crate::cli::{
    console::{print_single_separator},
    print_display::{players_hand_display},
};
use crate::{wait_for_dramatic_pause};

use crate::logic::{GameSession};
use crate::trump::{Deck};

/// ゲームフロー処理
pub fn app() -> std::io::Result<()> {
    let game = GameSession::new();

    // ディーラー設定
    let mut dealer = game.dealer_setup();
    wait_for_dramatic_pause();

    print_single_separator();

    // 山札設定
    let mut deck = Deck::new();
    game.shuffle(deck.get_cards());

    print_single_separator();

    // プレイヤー設定
    let mut players = game.players_setup(CPU_COUNT);
    let players_count = players.len();
    wait_for_dramatic_pause();

    print_single_separator();

    // 起家設定
    let current = game.starting_setup(&players);
    wait_for_dramatic_pause();

    print_single_separator();

    // ベット入力
    println!("{}", "Todo: CPU Bet processing.".red());
    for i in 0..players_count {
        let idx = (current + i) % players_count;
        game.input_bet(&mut players[idx]);
    }
    wait_for_dramatic_pause();

    print_single_separator();

    // 手札配り
    game.deal_setup(current, &mut deck, &mut players, &mut dealer);
    wait_for_dramatic_pause();

    print_single_separator();

    // 手札表示
    players_hand_display(&dealer, &players, false);

    print_single_separator();

    // プレイヤー操作
    // ヒット or スタンド
    println!("{}", "Todo: Input Hit or Stand.".red());
    game.input_call(current, &mut deck, &mut players);
    wait_for_dramatic_pause();

    print_single_separator();

    Ok(())
}

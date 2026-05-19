use crossterm::{
    style::{Stylize},
};

use crate::constants::{NUM_DECKS, CPU_COUNT};
use crate::cli::{
    console::{
        print_br,
        print_single_separator,
        print_double_separator,
    },
    print_display::{
        hand_display_one,
        players_hand_display,
        players_chip_display,
    },
};
use crate::{wait_for_dramatic_pause};

use crate::logic::{GameSession};
use crate::trump::{Deck, PlayerStatus};

/// ゲームフロー処理
pub fn app() -> std::io::Result<()> {
    let game = GameSession::new();

    // ディーラー設定
    let mut dealer = game.dealer_setup();
    wait_for_dramatic_pause();

    print_single_separator();

    // 山札設定
    println!("  Use Deck num: {}", NUM_DECKS);
    let mut deck = Deck::new(NUM_DECKS);
    game.shuffle(deck.get_cards());

    print_single_separator();

    // プレイヤー設定
    let mut players = game.players_setup(CPU_COUNT);
    let players_count = players.len();
    wait_for_dramatic_pause();

    print_double_separator();

    // 起家設定
    let current = game.starting_setup(&players);
    wait_for_dramatic_pause();

    print_single_separator();

    // ベット入力
    for i in 0..players_count {
        let idx = (current + i) % players_count;
        game.input_bet(&mut players[idx]);
    }
    wait_for_dramatic_pause();

    print_double_separator();

    // 手札配り
    game.deal_setup(current, &mut deck, &mut players, &mut dealer);
    wait_for_dramatic_pause();

    print_double_separator();

    // 手札表示
    players_hand_display(&dealer, &players, false);

    print_double_separator();

    print_br();

    println!("=================== {} ==================", "Player Turn".green());

    // プレイヤー操作
    let mut need_separator = false;
    while game.has_players_call(&players) {
        // ヒット or スタンド
        for i in 0..players_count {
            let idx = (current + i) % players_count;

            match players[idx].get_status() {
                PlayerStatus::Stand => continue,
                PlayerStatus::Burst => continue,
                _ => {},
            }

            if need_separator {
                print_single_separator();
            }
            need_separator = true;

            // コール入力
            game.input_call(&mut players[idx]);
            wait_for_dramatic_pause();

            // コール処理
            game.call_process(&mut deck, &mut players[idx]);
            hand_display_one(&players[idx], true);
            wait_for_dramatic_pause();
        }
    }

    print_double_separator();

    print_br();

    println!("=================== {} ==================", "Dealer Turn".magenta());

    // ディーラー操作
    need_separator = false;
    while game.has_player_call(&dealer) {
        if need_separator {
            print_single_separator();
        }
        need_separator = true;

        game.input_call(&mut dealer);
        wait_for_dramatic_pause();

        // コール処理
        game.call_process(&mut deck, &mut dealer);
        hand_display_one(&dealer, true);
        wait_for_dramatic_pause();
    }

    print_double_separator();

    print_br();

    println!("=================== {} ==================", "Game Result".yellow());

    // 判定
    println!("{}", "Todo: Natural Blackjack processing.".red());
    game.end_fase(&dealer, &mut players);
    wait_for_dramatic_pause();

    print_double_separator();

    print_br();

    println!("=================== {} ==================", "Chip Result".yellow());

    players_chip_display(&players);

    Ok(())
}

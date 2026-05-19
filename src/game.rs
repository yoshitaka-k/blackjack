use std::{
    io::stdout,
    error::Error,
};

use crossterm::{
    execute,
    cursor::{MoveTo},
    terminal::{Clear, ClearType},
};
use crossterm::{
    style::{Stylize},
};

use crate::constants::{NUM_DECKS, DEFAULT_ROUND_COUNT, CPU_COUNT};
use crate::cli::{
    console::{
        print_br,
        print_single_separator,
        print_double_separator,
    },
    indicate::{
        execute_with_spinner,
    },
    print_display::{
        title_display,
        hand_display_one,
        players_hand_display,
        players_chip_display,
    },
};
use crate::{
    wait_for_dramatic_pause,
    wait_for_long_dramatic_pause,
};

use crate::logic::{GameSession, Data};
use crate::trump::{Deck, Player, PlayerStatus};

/// ゲームフロー処理
pub fn app() -> Result<(), Box<dyn Error>> {
    let mut stroke = stdout();
    let game = GameSession::new();
    let round_count = DEFAULT_ROUND_COUNT;

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
    let load_data = Data::load()?;
    let mut players = if load_data.is_empty() {
        game.players_setup(CPU_COUNT)
    } else {
        game.players_data_load(&load_data, CPU_COUNT)
    };

    let players_count = players.len();
    wait_for_dramatic_pause();

    print_double_separator();

    // 起家設定
    let mut current = game.starting_setup(&players);
    wait_for_dramatic_pause();

    print_double_separator();

    print_br();

    execute_with_spinner(
        "Next Round ...",
        "",
    || {
        wait_for_long_dramatic_pause();
    });


    for round in 0..round_count {
        execute!(
            stroke,
            Clear(ClearType::All),
            MoveTo(0, 0)
        )?;

        title_display();

        print_br();

        println!("Round: {}/{}", round + 1, round_count);

        print_br();

        print_double_separator();

        game_run(current, &mut deck, &mut dealer, &mut players);

        println!("=================== {} ==================", "Game Result".yellow());

        // 判定
        println!("{}", "Todo: Natural Blackjack processing.".red());
        game.end_fase(&dealer, &mut players);
        wait_for_dramatic_pause();

        print_double_separator();

        print_br();

        println!("=================== {} ==================", "Chip Result".yellow());

        players_chip_display(&players);

        print_double_separator();

        print_br();

        wait_for_dramatic_pause();

        // 初期化
        dealer.clear();
        for player in &mut players {
            player.clear();
        }

        current = (current + 1) % players_count;

        execute_with_spinner(
            "Saving...",
            "Saved.",
        || {
            // プレイヤー情報書き込み
            let _ = Data::save(&players);
            wait_for_long_dramatic_pause();
        });

        if round < round_count - 1 {
            execute_with_spinner(
                "Next Round ...",
                "",
            || {
                wait_for_long_dramatic_pause();
            });
        }
    }

    print_br();

    Ok(())
}

fn game_run(current: usize, deck: &mut Deck, dealer: &mut Player, players: &mut Vec<Player>) {
    let game = GameSession::new();
    let players_count = players.len();

    // ベット入力
    for i in 0..players_count {
        let idx = (current + i) % players_count;
        game.input_bet(&mut players[idx]);
        wait_for_dramatic_pause();
    }

    print_double_separator();

    print_br();

    print_double_separator();

    // 手札配り
    game.deal_setup(current, deck, players, dealer);
    wait_for_dramatic_pause();

    print_single_separator();

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
            game.call_process(deck, &mut players[idx]);
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

        game.input_call(dealer);
        wait_for_dramatic_pause();

        // コール処理
        game.call_process(deck, dealer);
        hand_display_one(&dealer, true);
        wait_for_dramatic_pause();
    }

    print_double_separator();

    print_br();

    wait_for_dramatic_pause();
}

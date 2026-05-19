use figlet_rs::FIGlet;
use crossterm::{
    style::{Stylize},
};

use crate::constants::{TWENTY_ONE_NUM};
use crate::cli::{
    console::{print_single_separator},
};
use crate::trump::{Player};
use crate::{capitalize, wait_for_dramatic_pause};

/// タイトルとか表示
pub fn title_display() {
    let standard_font = FIGlet::standard().unwrap();
    let title = &format!("{}", standard_font.convert(&capitalize(env!("CARGO_PKG_NAME"))).unwrap());

    print!("{}", title.clone().magenta().bold());

    print_single_separator();

    println!("  Version: {}  |  License: {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_LICENSE"));
    println!("  Starting {} Game Engine... 🚀", env!("CARGO_PKG_NAME"));

    print_single_separator();

    println!("{}", "  Key of Game Force quit. (Ctrl+C or Ctrl+D).".yellow());
}

/// 1人手札表示
pub fn hand_display_one(player: &Player, open: bool) {
    let mut msg: String = String::new();

    print!("  {:>6} ", player.get_name());

    if open {
        let hand = format!("{:>2}", player.rank_sum());

        if player.rank_sum() > TWENTY_ONE_NUM {
            print!("Hand ({:>2}): ", hand.red());
        } else if player.rank_sum() == TWENTY_ONE_NUM {
            print!("Hand ({:>2}): ", hand.green().bold());
        } else {
            print!("Hand ({:>2}): ", hand.green());
        }
    } else {
        print!("Hand (??): ");
    }

    for (i, card) in player.get_hand().clone().iter().enumerate() {
        if !open && i > 0 {
            msg = format!("{}??, ", msg);
        } else {
            msg = format!("{}{:>3}, ", msg, card.get_name());
        }
    }

    if let Some(m) = msg.to_string().strip_suffix(", ") {
        println!("{}", m);
    }
}

/// 手札表示
pub fn players_hand_display(dealer: &Player, players: &Vec<Player>, open: bool) {
    println!("Dealer Hand");

    hand_display_one(dealer, open);

    wait_for_dramatic_pause();

    print_single_separator();

    println!("Players Hand");

    for player in players {
        hand_display_one(player, true);

        wait_for_dramatic_pause();
    }
}

/// 所持チップ表示
pub fn players_chip_display(players: &Vec<Player>) {
    for player in players {
        println!("  {:>6}: {:>3}", player.get_name(), player.get_chip());
    }
}

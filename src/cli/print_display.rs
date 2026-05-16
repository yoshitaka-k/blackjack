use figlet_rs::FIGlet;
use crossterm::{
    style::{Stylize},
};

use crate::constants::{TWENTY_ONE_NUM};
use crate::cli::{
    console::{print_single_separator},
};
use crate::trump::{Player};
use crate::{capitalize};

/// タイトルとか表示
pub fn title_display() {
    let standard_font = FIGlet::standard().unwrap();
    let title = &format!("{}", standard_font.convert(&capitalize(env!("CARGO_PKG_NAME"))).unwrap());

    print!("{}", title.clone().magenta().bold());

    print_single_separator();

    println!("  Version: {}  |  License: {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_LICENSE"));
    println!("  Starting {} Game Engine... 🚀", env!("CARGO_PKG_NAME"));
}

/// 1人手札表示
fn hand_display_one(player: &Player, open: bool) {
    let mut msg: String = String::new();

    print!("  {:>6} ", player.get_name());

    if open {
        let hand = format!("{:>2}", player.rank_sum());

        if player.rank_sum() > TWENTY_ONE_NUM {
            print!("Hand ({:>2}): ", hand.red());
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

    msg = msg.trim().to_string();
    if let Some(m) = msg.strip_suffix(",") {
        println!("{}", m);
    }
}

/// 手札表示
pub fn players_hand_display(dealer: &Player, players: &Vec<Player>, open: bool) {
    println!("Dealer Hand");

    hand_display_one(dealer, open);

    print_single_separator();

    println!("Players Hand");

    for player in players {
        hand_display_one(player, true);
    }
}

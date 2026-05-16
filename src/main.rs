use std::io::{stdout};
use crossterm::{
    execute,
    cursor::{MoveTo},
    terminal::{Clear, ClearType},
};

use blackjack::cli::{
    console::{
        print_br,
        print_double_separator
    },
    print_display::{title_display}
};
use blackjack::game::{app};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stroke = stdout();

    execute!(
        stroke,
        Clear(ClearType::All),
        MoveTo(0, 0)
    )?;

    print_double_separator();

    title_display();

    print_double_separator();

    let _ = app();

    print_double_separator();

    print_br();

    Ok(())
}

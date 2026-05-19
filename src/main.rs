use std::io::{stdout};
use crossterm::{
    execute,
    cursor::{MoveTo},
    terminal::{Clear, ClearType},
};

use blackjack::cli::{
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

    title_display();

    let _ = app();

    Ok(())
}

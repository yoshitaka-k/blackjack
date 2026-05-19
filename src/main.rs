use std::{
    io::stdout,
    error::Error,
};
use crossterm::{
    execute,
    cursor::{MoveTo},
    terminal::{Clear, ClearType},
};

use blackjack::{
    cli::print_display::title_display,
    game::app,
};

fn main() -> Result<(), Box<dyn Error>> {
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

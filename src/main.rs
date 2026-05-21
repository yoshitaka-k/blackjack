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
    cli::{
        console::error,
        print_display::title_display,
    },
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

    if let Err(e) = app() {
        error(&format!("Game aborted: {}", e));
        std::process::exit(1);
    }

    Ok(())
}

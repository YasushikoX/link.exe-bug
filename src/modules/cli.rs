use crossterm::{
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType}
};
use std::io::{self, stdout};

pub fn display_menu() -> i32 {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print("╔══════════════════════════════════════╗\n"),
        Print("║      Welcome to Super Mailer         ║\n"),
        Print("╚══════════════════════════════════════╝\n\n"),
        Print("Please choose an option from below:\n\n"),
        Print("  1.  Create new mailing task\n"),
        Print("  2.  Configure emails\n"),
        Print("  3.  Configure AI settings\n"),
        Print("  4.  Log In\n"),
        Print("  5.  Test\n\n"),
    ).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap_or(0)
}

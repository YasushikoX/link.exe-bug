mod modules {
    pub mod oauth;
    pub mod cli;
    pub mod mail;
}

use modules::oauth::{login};
use modules::cli::display_menu;
use modules::mail::send_mail;

fn main() {
    let choice = display_menu();

    match choice {
        1 => println!("Creating new mailing task..."),
        2 => println!("Configuring emails..."),
        3 => println!("Configuring AI settings..."),
        4 => {
            println!("Beginning OAuth2 login...");
                login();
            }
        5 => {
            println!("Running tests...");
            send_mail();
        },
        _ => {
            println!("Invalid choice. Please select a number between 1 and 5.");
        }
    }
}

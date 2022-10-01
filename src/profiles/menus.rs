use crate::general_libraries;
use crate::profiles::functions::Player;
use crate::profiles;
use std::io::Write;

pub fn create_user_menu(v: &mut Vec <Player>) -> i32{ 
    general_libraries::repeater('╔','═','╗',74); 
    println!("\n║\t\t    Welcome to character creation!!!    \t\t║");
    println!("║\t\t\t\t\t\t\t\t\t║");
    println!("║ Jk, this isn't an RPG, just pick a nickname.\t\t\t\t║");
    general_libraries::repeater('╚','═','╝',74);
    print!("\n ->> ");
    std::io::stdout().flush().expect("Failed to flush");

    profiles::functions::create_user(v);

    println!("\nThere you have it, now you can go ahead and play!");
    
    return 0;
}
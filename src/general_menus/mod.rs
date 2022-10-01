pub use std::io::Write;
pub use crate::general_libraries;

pub fn main_menu() -> i32 {
    std::io::stdout().flush().expect("Failed to flush");
    print!("\n");
    general_libraries::repeater('╔','═','╗',74); 
    println!("\n║\t\t\t Welcome to \"Rusty Games\"\t\t\t║");
    println!("║\t\t       What would you like to play?\t\t\t║\n║\t\t\t\t\t\t\t\t\t║");
    println!("║ 1. I want to play \"Guess The Rusty Number\"\t\t\t\t║");
    println!("║ 2. I want to see \"Guess The Rusty Number\" ranks.\t\t\t║");
    println!("║ 11. I want to change my username.\t\t\t\t\t║");
    println!("║ 12. I want to create a new user.\t\t\t\t\t║");
    println!("║ 13. I want to exit the game.\t\t\t\t\t\t║");
    general_libraries::repeater('╚','═','╝',74);
    print!("\n--> ");
    general_libraries::atoi()
}

pub fn goodbye(){
    print!("\n");
    general_libraries::repeater('╔','═','╗',34); 
    println!("\n║\t    Goodbye!!!\t\t║\n║\tHave a nice day!\t║");
    general_libraries::repeater('╚','═','╝',34);
    println!("\n");
}
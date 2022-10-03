use crate::games::rusty_numbers::diffs::Difficulty;
use rand::Rng;
use std::cmp::Ordering;
use super::menu;

pub fn guess_the_rusty_number(diff: &Difficulty) -> i32{

    let mut guess;
    let num = rand::thread_rng().gen_range(diff.minimum ..=diff.maximum);
    println!("[DEBUG] Diff = {:#?}", diff);
    println!("[DEBUG] num = {num}");

    menu::rusty_numbers_explain();

    loop{
        guess = menu::input_guess();
        match guess.cmp(&num){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                menu::winner_menu();
                break;
            },
        }
    }
    return -1;
}
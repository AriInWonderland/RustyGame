use crate::games::rusty_numbers::diffs::Difficulty;
use rand::Rng;
use std::cmp::Ordering;
use crate::general_libraries;

pub fn guess_the_rusty_number(diff: &Difficulty) -> i32{

    let mut guess;
    let num = rand::thread_rng().gen_range(diff.minimum ..=diff.maximum);
    println!("Diff = {:#?}", diff);
    println!("num = {num}");

    loop{
        println!("Input your guess...");
        guess = general_libraries::atoi();
        match guess.cmp(&num){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too small"),
            Ordering::Equal => {
                println!("You won!");
                break;
            },
        }
    }

    return -1;
}
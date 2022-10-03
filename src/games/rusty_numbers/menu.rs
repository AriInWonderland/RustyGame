use crate::general_libraries;
use std::io::Write;
use super::diffs;
use super::random_game;

pub fn rusty_numbers_menu() -> i32{
    let mut check:i32 = -1;
    let mut actual_index:usize = 1;
    
    let mut difficulties:Vec <diffs::Difficulty> = Vec::new();
    diffs::default_diffs_create(&mut difficulties);

    let mut actual_diff = &difficulties[actual_index];
    
    //print the user data at the right of the menu
    loop{
        if check != -1 {
            check = general_libraries::atoi();
        }

        match check{

            -3 => {
                diffs::show_diff_array(&difficulties);
                continue;
            },

            -2 => return -2,

            -1 => {
                rusty_welcome(actual_diff);
                check = -10;
            },

            0 => continue,

            1 => check = random_game::guess_the_rusty_number(actual_diff),

            2 => {
                actual_index = change_diff_menu(&difficulties, actual_index);
                actual_diff = &difficulties[actual_index];
                check = -1;
                continue;
            }

            //3 => 

            //4 => change_user_name

            //5 => show game ranks

            6 => return 12,

            7 => return 0,

            8 => return 13,

            _other => {
                println!("\n --> Please input a valid number <--\n");
                continue;
            },
        };
    }
}

pub fn change_diff_menu(v:&Vec <diffs::Difficulty>, actual_diff:usize) -> usize{
    let mut new_diff:usize;
    let mut j:usize = 1;

    general_libraries::repeater('╔','═','╗',74);       
    println!("\n║\t\t  Welcome to the difficulty change menu!\t\t║");
    println!("║\t\tHow hard would do you want the game to be?\t\t║\n║\t\t\t\t\t\t\t\t\t║");

    for i in v{
        println!("║ {}. {}\t\t\t\t\t\t\t\t║",j, i.name);
        j+=1;
    }

    general_libraries::repeater('╚','═','╝',74);
    print!("\n--> ");

    new_diff = general_libraries::atoi() as usize;

    if new_diff > 0{
        new_diff-=1;
    }

    if new_diff >= j {
        println!("\n --> Please input a number from the list <--\n");
        change_diff_menu(v, actual_diff);
        return actual_diff;
        //change_diff_menu(v, actual_diff);
    } else if new_diff == 0{
        new_diff = actual_diff;
    }

    return new_diff;
}

pub fn rusty_welcome(actual_diff: & diffs::Difficulty){
        std::io::stdout().flush().expect("Failed to flush");
        general_libraries::repeater('╔','═','╗',74);       
        print!("\n");
        println!("║\t\t  Welcome to \"Guess the rusty number\"!!\t\t\t║");
        println!("║\t\t\tWhat would you like to do?\t\t\t║");
        println!("║\t\t\t\t\t\t\t\t\t║");
        println!("║ -3. Show difficulties\t\t\t\t\t\t\t║");
        println!("║ -2. Show profiles\t\t\t\t\t\t\t║");
        println!("║ -1. IDLE\t\t\t\t\t\t\t\t║");
        println!("║  1. I just want to play the game!\t\t\t\t\t║");
        println!("║  2. I want to change the current difficulty ({})\t\t\t║", actual_diff.name);
        println!("║  3. I want to switch my profile.\t\t\t\t\t║");
        println!("║  4. I want to change my username!\t\t\t\t\t║");
        println!("║  5. I want to see the ranks.\t\t\t\t\t\t║");
        println!("║  6. I want to create a new profile!\t\t\t\t\t║");
        println!("║  7. I want to go back to the main menu!!!\t\t\t\t║");
        println!("║  8. I want exit this damn thing!!!\t\t\t\t\t║");
        general_libraries::repeater('╚','═','╝',74);
        print!("\n--> "); 
}
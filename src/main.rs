//Hacer easter eggs con banderas
use game::general_menus;
use game::games::rusty_numbers;
use game::profiles::functions;
use game::profiles;

//the difficulty is managed on the game which it belongs to
fn main() {
    //A pointer to the difficulty structs

    let mut check:i32 = -1;

    let mut players:Vec <functions::Player> = Vec::new();

    loop{
        
        if check == 13{
            general_menus::goodbye();
            break;
        }

        if check == -1{
            check = general_menus::main_menu();
        }

        match check{
            //Negative numbers are debug stuff, but -1 is "idle"
           //This one goes to the rusty menu
           /*-2 => {
                continue;
           }*/
           -2 => check = profiles::functions::db_show_profiles(&players),

            0 => {
                check = -1;
                continue;
            }

            1 => check = rusty_numbers::menu::rusty_numbers_menu(),

            //11 => change username
            12 => {
                check = profiles::menus::create_user_menu(&mut players);
                continue;
            }

            13 => {
                check = 13;
                continue;
            }

            _ => {
                println!("\n--> Please input a valid number <--\n");
                check = -1;
                continue;
            },
        };
    }

}
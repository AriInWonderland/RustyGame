//Los users estan en un array, hacer en los menus principales una parte que sea de rankig,
//con un asterisco marca el actual en el ranking, una funcion en la que podes ver bien tu perfil
use crate::games::rusty_numbers::player::GuessGame;

#[derive(Debug)]
pub struct Player{
    pub guess_player : GuessGame,
}

pub fn create_user(v: &mut Vec <Player>) /*-> Player*/{
    let mut s = String::new();

    std::io::stdin().read_line(&mut s).expect("Failed to parse line");
    
    s = s.to_string();
    
    v.push({Player{
        guess_player : GuessGame{
            name : s.trim().to_string(),
            games_played : 0,
            points : 0,
            rank : 0,
        }
    }});
}

pub fn db_show_profiles(v: &Vec <Player>) -> i32{
    println!("\n[DEBUG]");
    for i in v{
        println!("{:#?}", i);
    }

    return 0;
}

//l = left character c = center characters r = right character
//w = total width of the string.
pub fn repeater(l:char, c:char, r:char, w:i32){
    let mut i: i32 = 1;

    print!("{l}");
    loop{
        if i == w-2{
            print!("{r}");
            break;
        }
        else{
            print!("{c}");
            i += 1;
        }
    }
}

use std::io::Write;
pub fn atoi() -> i32{
    let mut a = String::new();

    std::io::stdout().flush().expect("Failed to flush");
    std::io::stdin().read_line(&mut a).expect("Failed to read line");
    let a:i32 = a.trim().parse().expect("Failed to parse line");

    return a;
}
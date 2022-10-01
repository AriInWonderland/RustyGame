#[derive(Debug)]
pub struct Difficulty{
    pub minimum : i32,
    pub maximum : i32,
    pub name    : String,
    pub mode    : String,
}

    impl Difficulty{
        pub fn pass_values(&mut self, min:i32, max:i32, name:String, mode:String){
            self.minimum  = min;
            self.maximum  = max;
            self.name     = name;
            self.mode     = mode;
        }
    }

pub use crate::general_libraries;
pub fn show_diff_array(v:&Vec <Difficulty>){
    general_libraries::repeater('╔', '═','═', 10);
    print!("[DEBUG]");
    general_libraries::repeater('═', '═','╗', 10);
    print!("\n");
        for i in v{
            println!("  {:#?} ", i);
        }
    print!("\n");
    general_libraries::repeater('╚','═','╝',27);
    print!("\n");
}

//later on this should read a file to asign the diffs
pub fn default_diffs_create(v:&mut Vec<Difficulty>){
    let s = ["Very Easy", "Easy", "Medium", "Hard", "Very Hard", "Insane", "Really?"];
    let maximums = [50, 100, 250, 300, 400, 600, 1000];

    let mut j:usize = 0;
    
    for i in s{
        v.push({Difficulty{
            minimum : 0,
            maximum : maximums[j],
            name : i.to_string(),
            mode : String::from("Default"),
        }});
        j+=1;
    }
}

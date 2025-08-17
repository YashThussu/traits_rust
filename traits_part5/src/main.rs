
#[derive(Debug)]
enum Musician{
    SingerSongwriter(String),
    Band(u32)
}

impl PartialEq for Musician {

    fn eq(&self, other: &Self) -> bool {
        
        match self {
            SingerSongwriter(name)=> match other {
                SingerSongwriter(other_name)=> name == other_name,
                Band(_)=> false,
            },
            Band(num)=> match other {
                SingerSongwriter(_)=> false,
                Band(other_num)=> num == other_num,
            }
        }
    }
}

use Musician::*;

fn main() {

    let rustin_bieber=SingerSongwriter(String::from("Rustin"));
    let rustin_timberlake=SingerSongwriter(String::from("Rustin"));
    let holly=SingerSongwriter(String::from("Holly"));

    let rust_no_one=Band(5);
    let rustworthy=Band(4);
    let rust_for_vengeance=Band(5);
    

    println!("{}",rustin_bieber == rustin_timberlake);
    println!("{}",rustin_timberlake == rustin_bieber );
    println!("{}",rustin_bieber == holly);
}

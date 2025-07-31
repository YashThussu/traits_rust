use traits_part1::lodging::{Hotel, Airbnb, Accommodation, Description};

use traits_part1::utils;

fn main() {
    
    let mut hotel=Hotel::new("The Luxe");   
    println!("{}",hotel.summarize());
    hotel.book("Dana",5);
    
    let mut airbnb=Airbnb::new("Parker");
    println!("{}",airbnb.get_description());
    utils::book_for_one_night(&mut airbnb, "Dan");

    utils::mix_and_match(&mut hotel, &mut airbnb, "Alibaba");

    // let stays:Vec<&dyn Description>=vec![&hotel, &Airbnb];
    // println!("{}",stays[0].get_description());
    // println!("{}",stays[1].get_description());

    // let mut stays2: Vec<&mut dyn Accommodation> = vec![&mut hotel, &mut Airbnb];
    // stays2[0].book("Alice", 2);
    // stays2[1].book("Bob", 3);

    // println!("{:#?}",hotel);
    // println!("{:#?}",Airbnb);


}

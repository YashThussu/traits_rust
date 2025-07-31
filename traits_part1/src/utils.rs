use crate::lodging::{Accommodation, Description,Hotel,Airbnb};

pub fn book_for_one_night<T: Accommodation>(entity: &mut T, guestname: &str) {
    
    entity.book(guestname,1);
}

// fn mix_and_match<T: Accommodation, U: Accommodation>(entity1: &mut T, entity2: &mut U) {
    
//     entity1.book("Alice", 2);
//     entity2.book("Bob", 3);
// }


pub fn mix_and_match<T,U>(entity1: &mut T, entity2: &mut U, guest:&str)
where T: Accommodation+Description,
      U: Accommodation {
    
    entity1.book(guest, 1);
    println!("{}",entity1.get_description());

    entity2.book(guest, 3);
}

pub fn choose_best_place_to_stay()->impl Accommodation + Description + std::fmt::Debug {
    

   Hotel::new("The Luxe")
}
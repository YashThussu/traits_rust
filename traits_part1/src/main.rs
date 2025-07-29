use std::collections::HashMap;
use std::fmt::Display;


trait Accommodation {

    fn book(&mut self,name:&str,nights:u32);
}

trait Description {

    //default implementation
    fn get_description(&self)->String{
        String::from("A wonderful place to stay")
    }
}

#[derive(Debug)]
struct Hotel<T>{
    name:T,
    reservation:HashMap<String,u32>
}

impl<T> Hotel<T>{

        fn new(name:T)->Self{
        Self{
            name,
            reservation:HashMap::new()
        }
    }
}

impl<T:Display> Hotel<T>{
    
    fn summarize(&self)->String{

        format!("{}:{}",self.name,self.get_description())
    }
}

impl <T> Accommodation for Hotel<T>{

    fn book(&mut self,name:&str,nights:u32) {
        
        self.reservation.insert(name.to_string(), nights);
    }
}

// this takes the default implementation from the trait
impl<T> Description for Hotel<T>{}


#[derive(Debug)]
struct Airbnb{
    host:String,
    guests:Vec<(String,u32)>
}

impl Airbnb{

    fn new(host:&str)->Self{

        Self { 
            host: host.to_string(), 
            guests: vec![] 
        }
    }
}

impl Accommodation for Airbnb{

    fn book(&mut self,name:&str,nights:u32) {
        
        self.guests.push((name.to_string(),nights));
    }
    
}

impl Description for Airbnb {

    // types own implementation of the trait
    // this will override the default implementation

    fn get_description(&self) -> String {
        format!("A cozy Airbnb hosted by {}", self.host)
    }
}


fn book_for_one_night<T: Accommodation>(entity: &mut T, guestname: &str) {
    
    entity.book(guestname,1);
}

// fn mix_and_match<T: Accommodation, U: Accommodation>(entity1: &mut T, entity2: &mut U) {
    
//     entity1.book("Alice", 2);
//     entity2.book("Bob", 3);
// }

// the below and above function are equivalent

fn mix_and_match(entity1: &mut (impl Accommodation + Description), entity2: &mut impl Accommodation) {
    
    entity1.book("Alice", 2);
    println!("{}",entity1.get_description());

    entity2.book("Bob", 3);
    
}

fn mix_and_match2<T,U>(entity1: &mut T, entity2: &mut U)
where T: Accommodation+Description,
      U: Accommodation {
    
    entity1.book("Alice", 2);
    println!("{}",entity1.get_description());

    entity2.book("Bob", 3);
}

fn choose_best_place_to_stay()->impl Accommodation + Description + std::fmt::Debug {
    

   Hotel::new("The Luxe")
}

fn main() {
    
    let hotel1=Hotel::new(String::from("The Luxe"));
    println!("{}", hotel1.summarize());

    let hotel2=Hotel::new("The Grand Palace");
    println!("{}",hotel2.summarize());

    let hotel3=Hotel::new(vec!["The Cozy Inn", "The Grand Hotel"]);
    println!("{}", hotel3.summarize());
}

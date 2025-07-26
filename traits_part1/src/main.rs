use std::collections::HashMap;


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
struct Hotel{
    name:String,
    reservation:HashMap<String,u32>
}

impl Hotel{
    
    fn new(name:&str)->Self{
        Self{
            name:name.to_string(),
            reservation:HashMap::new()
        }
    }

    fn summarize(&self)->String{

        format!("{}:{}",self.name,self.get_description())
    }
}

impl Accommodation for Hotel{

    fn book(&mut self,name:&str,nights:u32) {
        
        self.reservation.insert(name.to_string(), nights);
    }
}

// this takes the default implementation from the trait
impl Description for Hotel{}


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

fn mix_and_match2<T:Accommodation+Description,U:Accommodation>(entity1: &mut T, entity2: &mut U){
    
    entity1.book("Alice", 2);
    println!("{}",entity1.get_description());

    entity2.book("Bob", 3);
}

fn main() {
    
    let mut hotel=Hotel::new("The Lux");
    book_for_one_night( &mut hotel,"piers");
    println!("Hotel: {:?}", hotel);
    let mut airbnb=Airbnb::new("Peter");
    book_for_one_night( &mut airbnb,"amanda");
    println!("Airbnb: {:?}", airbnb);

    mix_and_match(&mut hotel, &mut airbnb);
    mix_and_match2(&mut hotel, &mut airbnb);
}

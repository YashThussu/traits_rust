use std::collections::HashMap;


trait Accommodation {
    
    // fn get_description(&self)->String; 

    // default implementation
    fn get_description(&self)->String{
        String::from("A wonderful place to stay")
    }

    fn book(&mut self,name:&str,nights:u32);
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

    // fn get_description(&self)->String {
        
    //     format!("{} is the pinnacle of luxury",self.name)
    // }

    fn book(&mut self,name:&str,nights:u32) {
        
        self.reservation.insert(name.to_string(), nights);
    }
}


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
    
    fn get_description(&self)->String {
        format!("Please enjoy {}'s Apartment",self.host)    
    }

    fn book(&mut self,name:&str,nights:u32) {
        
        self.guests.push((name.to_string(),nights));
    }
    
}

fn book_for_one_night(entity: &mut impl Accommodation, guestname: &str) {
    
    entity.book(guestname,1);
}

fn main() {
    
    let mut hotel=Hotel::new("The Lux");
    book_for_one_night( &mut hotel,"piers");
    println!("Hotel: {:?}", hotel);
    let mut airbnb=Airbnb::new("Peter");
    book_for_one_night( &mut airbnb,"amanda");
    println!("Airbnb: {:?}", airbnb);
}

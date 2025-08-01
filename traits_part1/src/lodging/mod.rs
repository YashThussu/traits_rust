use std::collections::HashMap;
use std::fmt::Display;


pub trait Accommodation {

    fn book(&mut self,name:&str,nights:u32);
}

pub trait Description {

    //default implementation
    fn get_description(&self)->String{
        String::from("A wonderful place to stay")
    }
}

#[derive(Debug)]
pub struct Hotel<T>{
    name:T,
    reservation:HashMap<String,u32>
}

impl<T> Hotel<T>{

        pub fn new(name:T)->Self{
        Self{
            name,
            reservation:HashMap::new()
        }
    }
}

impl<T:Display> Hotel<T>{
    
    pub fn summarize(&self)->String{

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
pub struct Airbnb{
    host:String,
    guests:Vec<(String,u32)>
}

impl Airbnb{

    pub fn new(host:&str)->Self{

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



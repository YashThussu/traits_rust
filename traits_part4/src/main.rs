struct Flight{
    origin:String,
    destination:String,
    duration:u32
}

impl Flight{

    fn new(origin:String, destination:String, duration:u32)->Self{

        Self{
            origin,
            destination,
            duration
        }
    }
}

impl PartialEq for Flight {

    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}


fn main() {
    
    let flight1= Flight::new("New York".to_string(), "New Delhi".to_string(), 30);
    let flight2= Flight::new("New York".to_string(), "New Delhi".to_string(), 60);
    let flight3= Flight::new("New York".to_string(), "Bangalore".to_string(), 30);

    println!("Flight 1 and Flight 2 are equal: {}", flight1 == flight2);
    println!("Fligh 1 and Flight 3 are equal:{}", flight1.eq(&flight3));
}

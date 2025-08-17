
// we can implement the PartialEq trait for our own types to compare them but it will compare all the fields inside the struct for them to be equal
// if we want to compare only some fields, we can implement the PartialEq trait for our own types and override the eq method

#[derive(Debug,PartialEq)]
struct BusTrip{
    origin:String,
    destination:String,
    duration:u32
}

impl BusTrip{

    fn new(origin:String, destination:String, duration:u32)->Self{

        Self{
            origin,
            destination,
            duration
        }
    }
}

impl PartialEq<Flight> for BusTrip{
    fn eq(&self, other: &Flight) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}


#[derive(Debug)]
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

impl PartialEq<BusTrip> for Flight {

    fn eq(&self, other: &BusTrip) -> bool {
        
        self.origin==other.origin && self.destination==other.destination
    }
}


fn main() {
    
    let flight1= Flight::new("New York".to_string(), "New Delhi".to_string(), 30);
    let flight2= Flight::new("New York".to_string(), "New Delhi".to_string(), 60);
    let flight3= Flight::new("New York".to_string(), "Bangalore".to_string(), 30);
    let flight4= Flight::new("New York".to_string(), "California".to_string(), 30);

    let bustrip1=BusTrip::new("New York".to_string(), "California".to_string(), 48);

    println!("Flight 1 and Flight 2 are equal: {}", flight1 == flight2);
    println!("Flight 1 and Flight 3 are equal:{}", flight1.eq(&flight3));
    println!("Flight 4 and Bus Trip 1 are equal: {}",flight4==bustrip1);
    println!("Bus Trip 1 and Flight 4 are equal: {}",bustrip1==flight4);

    println!("{:?}",bustrip1)
}

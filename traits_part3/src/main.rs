use std::fmt::{Debug,Display,Formatter,Result};
use std::ops::{Drop};
use std::fs;

enum AppleType {
    GrannySmith,
    Fuji,
    Honeycrisp
}

impl Display for AppleType {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
         write!(f,"{}",match self {
             AppleType::GrannySmith=>"Granny Smith",
             AppleType::Fuji=>"Fuji",
             AppleType::Honeycrisp=>"Honeycrisp"
         })
    }
}

impl Debug for AppleType {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"{}", match self {
            AppleType::GrannySmith=>"AppleType::Granny Smith",
            AppleType::Fuji=>"AppleType::Fuji",
            AppleType::Honeycrisp=>"AppleType::Honeycrisp"
        })
    }
}

struct Apple {
    kind:AppleType,
    price:f64
}

impl Apple{
    fn create_file(&self){

        match self.kind {
            AppleType::Fuji=>{
                match fs::File::create(r"C:\Users\YASH\Desktop\coding\Rust 2\Traits\traits_rust\traits_part3\fuji.txt"){
                    Ok(_)=>println!("Fuji.txt created successfully"),
                    Err(e)=>println!("Error creating file; {}", e)
                }
            },
            AppleType::GrannySmith=>{
                match fs::File::create(r"C:\Users\YASH\Desktop\coding\Rust 2\Traits\traits_rust\traits_part3\granny_smith.txt"){
                    Ok(_)=>println!("Granny Smith.txt created successfully"),
                    Err(e)=>println!("Error creating file; {}", e)
                }
            },
            AppleType::Honeycrisp=>{
                match fs::File::create(r"C:\Users\YASH\Desktop\coding\Rust 2\Traits\traits_rust\traits_part3\honeycrisp.txt"){
                    Ok(_)=>println!("Honeycrisp.txt created successfully"),
                    Err(e)=>println!("Error creating file; {}", e)
        }
    }
        };
            }
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"{} apple costs ${:.2}", self.kind, self.price)
    }
}

impl Debug for Apple {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"{:?}=> [{}]",self.kind,self.price)
    }
} 

impl Drop for Apple {
    
    fn drop(&mut self) {

        println!("Dropping an apple of type: {}",self.kind);

        match self.kind {
            AppleType::Fuji=>{
                match fs::remove_file(r"C:\Users\YASH\Desktop\coding\Rust 2\Traits\traits_rust\traits_part3\fuji.txt") {
                    Ok(_)=>println!("Fuji.txt dropped"),
                    Err(e)=>println!("Error dropping fuji.txt: {}", e)
                }
            },
            AppleType::GrannySmith=>{
                match fs::remove_file(r"C:\Users\YASH\Desktop\coding\Rust 2\Traits\traits_rust\traits_part3\granny_smith.txt") {
                    Ok(_)=>println!("Granny Smith.txt dropped"),
                    Err(e)=>println!("Error dropping granny_smith.txt: {}", e)
                }
            },
            AppleType::Honeycrisp=>{
                match fs::remove_file(r"C:\Users\YASH\Desktop\coding\Rust 2\Traits\traits_rust\traits_part3\honeycrisp.txt") {
                    Ok(_)=>println!("Honeycrisp.txt dropped"),
                    Err(e)=>println!("Error dropping honeycrisp.txt: {}", e)
                }
            }
            
        };
}
}
    

fn main() {

    let lunch_snack= Apple {
        kind: AppleType::GrannySmith,
        price: 1.04
    };

    lunch_snack.create_file();

    let dinner= Apple{
        kind: AppleType::Fuji,
        price: 1.25
    };

    dinner.create_file();

    println!("{:?}",lunch_snack);
    println!("{:?}",dinner);
}

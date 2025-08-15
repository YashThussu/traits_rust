use std::fmt::{Display,Formatter,Result};

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

struct Apple {
    kind:AppleType,
    price:f64
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"{} apple costs ${:.2}", self.kind, self.price)
    }
}

fn main() {

    let lunch_snack= Apple {
        kind: AppleType::GrannySmith,
        price: 1.04
    };

    let dinner= Apple{
        kind: AppleType::Fuji,
        price: 1.25
    };

    println!("{}",lunch_snack);
    println!("{}",dinner);
}

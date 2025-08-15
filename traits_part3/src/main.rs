use std::fmt::{Display,Formatter,Result};

struct Apple {
    kind:String,
    price:f64
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"{} apple costs ${:.2}", self.kind, self.price)
    }
}

fn main() {

    let lunch_snack= Apple {
        kind: String::from("Granny Smith"),
        price: 1.04
    };

    println!("{}",lunch_snack);
}

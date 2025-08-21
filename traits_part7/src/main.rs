use std::ops::Add;


#[derive(Debug)]
struct Lunch{
    cost:f64
}

impl Add for Lunch {
    type Output = f64;

    fn add(self, rhs: Self) -> Self::Output {
        
        self.cost + rhs.cost
    }
}

fn main() {
    
    let lunch1 = Lunch { cost: 12.50 };
    let lunch2 = Lunch { cost: 15.75 };

    // let total_cost:f64 = lunch1+ lunch2;

    println!("Total cost of lunches: {:.2}", lunch1 + lunch2);
}

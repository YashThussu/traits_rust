use std::{ops::Add, result};


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

fn add_two_numbers<T:Add<Output=T>>(a:T,b:T)->T{
    a+b
}

fn multiply_two_numbers<T,U>(a:T,b:U)->f64
where
T:Into<f64>,
U:Into<f64> {
    a.into()*b.into()
}


fn main() {
    
    let lunch1 = Lunch { cost: 12.50 };
    let lunch2 = Lunch { cost: 15.75 };

    // let total_cost:f64 = lunch1+ lunch2;

    println!("Total cost of lunches: {:.2}", lunch1 + lunch2);
    println!("{}",add_two_numbers(10,20));
    println!("{}",add_two_numbers(10.131,20.12));

    println!("{}",multiply_two_numbers(10,20));
    println!("{}",multiply_two_numbers(10.131,20.12));
}

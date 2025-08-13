trait Taxable {

    const TAX_RATE: f64 = 0.25;

    fn set_amount(&mut self, new_amount:f64); // setter method

    fn double_amount(&mut self){
        self.set_amount(self.amount() * 2.0);
    } // 

    fn amount(&self)->f64; // setter method

    fn tax_bill(&self)->f64{
        self.amount()*Self::TAX_RATE
    }
    
}

#[derive(Debug)]
struct Income{
    amount: f64,
}

impl Taxable for Income{

    fn amount(&self)->f64{
        self.amount
    }

    fn set_amount(&mut self, new_amount:f64) {
        self.amount = new_amount;
    }
}

#[derive(Debug)]
struct Bonus{
    value: f64,
}

impl Taxable for Bonus{

    const TAX_RATE:f64=0.30;

    fn set_amount(&mut self, new_amount:f64) {
        self.value = new_amount;
    }
    
    fn amount(&self)->f64{
        self.value
    }
}

fn main() {
    
    let mut income=Income{amount: 1000.0};
    println!("Total tax owned: {:.2}", income.tax_bill());
    println!("Tax rate for income: {:.2}", Income::TAX_RATE);
    income.double_amount();
    println!("New amount after doubling: {:.2}", income.amount());

    let mut bonus=Bonus{value: 150000.0};
    println!("Total tax owned: {:.2}", bonus.tax_bill());
    println!("Tax rate for bonus: {:.2}", Bonus::TAX_RATE);
    bonus.double_amount();
    println!("New amount after doubling: {:.2}", bonus.amount());
}
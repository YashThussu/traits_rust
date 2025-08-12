trait Taxable {

    const TAX_RATE: f64 = 0.25;

    fn amount(&self)->f64;

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
}

#[derive(Debug)]
struct Bonus{
    amount: f64,
}

impl Taxable for Bonus{

    const TAX_RATE:f64=0.30;

    fn amount(&self)->f64{
        self.amount
    }
}

fn main() {
    
    let income=Income{amount: 1000.0};
    println!("Total tax owned: {:.2}", income.tax_bill());
    println!("Tax rate for income: {:.2}", Income::TAX_RATE);

    let bonus=Bonus{amount: 150000.0};
    println!("Total tax owned: {:.2}", bonus.tax_bill());
    println!("Tax rate for bonus: {:.2}", Bonus::TAX_RATE);
}
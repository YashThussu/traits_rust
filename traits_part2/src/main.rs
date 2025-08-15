trait Investment<T> {

     fn set_amount(&mut self, new_amount:T); // setter method

    fn amount(&self)->T; // setter method

    fn double_amount(&mut self);

}

//taxable traits inherit from Investement trait
// this allows us to use the methods defined in Investement trait
trait Taxable: Investment <f64> {

    const TAX_RATE: f64 = 0.25;


    fn tax_bill(&self)->f64{
        self.amount()*Self::TAX_RATE
    }
    
}

#[derive(Debug)]
struct Income{
    amount: f64,
}

impl Investment<f64> for Income {
        fn amount(&self)->f64{
        self.amount
    }

    fn set_amount(&mut self, new_amount:f64) {
        self.amount = new_amount;
    }

    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }
}

impl Taxable for Income{}

#[derive(Debug)]
struct Bonus{
    value: f64,
}

impl Investment<f64> for Bonus {

    fn set_amount(&mut self, new_amount:f64) {
        self.value = new_amount;
    }
    
    fn amount(&self)->f64{
        self.value
    }

    fn double_amount(&mut self){
        self.value *= 2.0;
    }
}

impl Taxable for Bonus{

    const TAX_RATE:f64=0.30;
}

#[derive(Debug)]
struct QualityTime {
    minutes:u32,
}

impl Investment<u32> for QualityTime{

    fn amount(&self)->u32{
        self.minutes
    }

    fn set_amount(&mut self, new_amount:u32){
        self.minutes = new_amount;
    }

    fn double_amount(&mut self) {
        self.minutes *= 2;
    }
}

fn main() {
    
    // let mut income=Income{amount: 1000.0};
    // println!("Total tax owned: {:.2}", income.tax_bill());
    // println!("Tax rate for income: {:.2}", Income::TAX_RATE);
    // income.double_amount();
    // println!("New amount after doubling: {:.2}", income.amount());

    // let mut bonus=Bonus{value: 150000.0};
    // println!("Total tax owned: {:.2}", bonus.tax_bill());
    // println!("Tax rate for bonus: {:.2}", Bonus::TAX_RATE);
    // bonus.double_amount();
    // println!("New amount after doubling: {:.2}", bonus.amount());

    // let mut quality_time=QualityTime{minutes: 120.0};
    // println!("Relaxation time :{:.2}", quality_time.amount());
    // println!("Doubling relaxation time...");
    // quality_time.double_amount();
    // println!("New relaxation time: {:.2}", quality_time.amount());


    let mut income=Income{amount:50000.50};
    let mut bonus=Bonus{value: 20000.75};
    let mut rust_programming_time=QualityTime{minutes: 180};

    income.double_amount();
    bonus.double_amount();
    rust_programming_time.double_amount();

    println!("quality doubled amount: {}",rust_programming_time.amount());
}
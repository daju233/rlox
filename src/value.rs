pub type Value = f64;

pub struct ValueArray{
   pub values:Vec<Value>,
}

impl ValueArray{
    pub fn new() -> Self{
        Self{ values:Vec::<Value>::new()}
    }
    pub fn print_value(&mut self, value:Value){
        print!("{}",value);
    }
}
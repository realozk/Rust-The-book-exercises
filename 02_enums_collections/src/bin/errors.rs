use std::path::MAIN_SEPARATOR;

struct Product {
    name: String,
    quantity: i32,
}

impl Product {

    fn new(name : String, quantity : i32) -> Product {
        if quantity < 0 {
            panic!("ya 7mar la tkon salb")
        }
            Product { name: name, quantity: quantity }
    }

    fn sell(&mut self,count: i32)-> Result <i32,String> {

        if count > self.quantity {
            return Err(String::from("ufwhiugfq"));
        }

        self.quantity -= count;
        Ok(self.quantity)
    }

    fn restock(&mut self,add: i32) -> Result <i32,String>{
    if add > self.quantity {
    return Err(String::from("ufwhiugfq"));
    }
    self.quantity += add;
    Ok(self.quantity)
    }

}

fn main(){

    let mut laptop = Product::new("laptop".to_string(), 10);

    match laptop.sell(5) {
        Ok(remin) => println!(" good , in the stock{}", remin),
        Err(e) => println!("Error: {}" , e)
    }

    match laptop.restock(5) {
        Ok(remin) => println!(" good , in the stock{}", remin),
        Err(e) => println!("Error: {}" , e)
    }
}


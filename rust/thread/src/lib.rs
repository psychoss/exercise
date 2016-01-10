use std::collections::HashMap;

#[derive(Clone,Debug)]
struct Store {
    name:String,
    prices:HashMap<String,f32>,
}
impl Store {
    fn add_item(&mut self,name:String,price:f32){
        self.prices.insert(name,price);
    }
    fn price(&self,item_name:&str)->f32{
        self.prices[item_name]
    }
    fn new(name:String)->Store{
        Store{
            name:name:
            prices:HashMap::new(),
        }
    }
}

fn build_stores()->Vec<Store>{
    let mut stores=vec![];
    let mut store=Store::new(format!("R-mart"));
    store.add_item(format!("chocolate",5.0));
    store.add_item(format!("doll",22.0));
    store.add_item(format!("bike",150.0));
    stores.push(store);
    
    stores 
    
}
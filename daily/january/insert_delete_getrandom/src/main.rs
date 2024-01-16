fn main() {
    let mut obj = RandomizedSet::new();
    let ret_1: bool = obj.insert(0);
    let ret_2: bool = obj.insert(1);
    let ret_3: bool = obj.remove(0);
    let ret_4: bool = obj.insert(2);
    let ret_5: bool = obj.remove(1);
    let ret_6: i32 = obj.get_random();
    

    println!("ret_1: {}", ret_1);
    println!("ret_2: {}", ret_2);
    println!("ret_3: {}", ret_3);
    println!("ret_4: {}", ret_4);
    println!("ret_5: {}", ret_5);
    println!("ret_6: {}", ret_6);
}

use rand::prelude::*;

struct RandomizedSet {
    table: std::collections::HashMap<i32, i32>,
    values: Vec<i32>,
}
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            table: std::collections::HashMap::new(),
            values: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.table.contains_key(&val) {
            return false;
        }

        self.table.insert(val, self.values.len() as i32);
        self.values.push(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.table.contains_key(&val) {
            return false;
        }
        let index = self.table.remove(&val).unwrap();
        let last_val = *self.values.last().unwrap();
        self.values.swap_remove(index as usize);

        if val != last_val {
            *self.table.get_mut(&last_val).unwrap() = index;
        }


        true        
    }

    fn get_random(&self) -> i32 {
        // rand::choose
        *self.values.choose(&mut rand::thread_rng()).unwrap()
        
    }
}
fn main() {
    let obj = RandomizedSet::new();
    let ret_1: bool = obj.insert(1);
    let ret_2: bool = obj.remove(2);
    let ret_3: i32 = obj.get_random();
    

    println!("ret_1: {}, ret_2: {}, ret_3: {}", ret_1, ret_2, ret_3);
}

struct RandomizedSet {
    values: std::collections::HashMap<i32, i32>,
}
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            values: std::collections::HashMap::new(),
        }
    }

    fn insert(&self, val: i32) -> bool {
        self.values.insert(val, val)
    }

    fn remove(&self, val: i32) -> bool {
        self.values.remove(&val)
    }

    fn get_random(&self) -> i32 {
        0
    }
}
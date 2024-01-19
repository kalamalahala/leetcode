/// Insert, Delete, Get Random O(1)
/// https://leetcode.com/problems/insert-delete-getrandom-o1/
/// 
/// Create a data structure that supports
/// all following operations in average O(1) time.
/// 
/// insert(val): Inserts an item val to the set if not already present.
/// remove(val): Removes an item val from the set if present.
/// getRandom: Returns a random element from current set of elements.

fn main() {
    let mut obj = RandomizedSet::new();
    println!("Op 1: {} (expect: true)", obj.insert(1));
    println!("Op 2: {} (expect: false)", obj.remove(2));
    println!("Op 3: {} (expect: true)", obj.insert(2));
    println!("Op 4: {:?}", obj.get_random());
    println!("Op 5: {} (expect: true)", obj.remove(1));
    println!("Op 6: {} (expect: false)", obj.insert(2));
    println!("Op 7: {:?}", obj.get_random());

    let mut obj = RandomizedSet::new();
    println!("Op 1: {} (expect: true)", obj.insert(0));
    println!("Op 2: {} (expect: true)", obj.insert(1));
    println!("Op 3: {} (expect: true)", obj.remove(0));
    println!("Op 4: {} (expect: true)", obj.insert(2));
    println!("Op 5: {} (expect: true)", obj.remove(1));
    println!("Op 6: {:?} (expect: 2)", obj.get_random());
}

use std::collections::HashSet;
use rand::prelude::*;

struct RandomizedSet {
    values: Vec<i32>,
    indexes: HashSet<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            values: vec![],
            indexes: HashSet::new(),
        }

    }

    fn insert(&mut self, val: i32) -> bool {
        if self.indexes.contains(&val) {
            return false;
        }

        self.values.push(val);
        self.indexes.insert(val);
        true        
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.indexes.contains(&val) {
            return false;
        }

        let index = self.values.iter().position(|&x| x == val).unwrap();
        self.values.remove(index);
        self.indexes.remove(&val);
        true
    }

    fn get_random(&self) -> i32 {
        self.values.choose(&mut thread_rng()).unwrap().clone()
    }
}
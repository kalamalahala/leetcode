use std::cell::RefCell;
use std::rc::Rc;

/// Given a binary tree where nodes are values from 1 to 9, 
/// a path in the tree is said to be psuedo-palindromic if at
/// least one permutation of the values in the path is a palindrome.
/// 
/// Return the number of psuedo-palindromic paths going from the root
/// node to leaf nodes.
fn main() {
    let mut root = Option::Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_mut().unwrap().borrow_mut().left = Option::Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_mut().unwrap().borrow_mut().right = Option::Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_mut().unwrap().borrow_mut().left.as_mut().unwrap().borrow_mut().left = Option::Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_mut().unwrap().borrow_mut().left.as_mut().unwrap().borrow_mut().right = Option::Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_mut().unwrap().borrow_mut().right.as_mut().unwrap().borrow_mut().right = Option::Some(Rc::new(RefCell::new(TreeNode::new(1))));

    println!("expected 2, got {}", Solution::psuedo_palindromic_paths(root));

    let mut root = Option::Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_mut().unwrap().borrow_mut().left = Option::Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_mut().unwrap().borrow_mut().right = Option::Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_mut().unwrap().borrow_mut().left.as_mut().unwrap().borrow_mut().left = Option::Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_mut().unwrap().borrow_mut().left.as_mut().unwrap().borrow_mut().right = Option::Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_mut().unwrap().borrow_mut().left.as_mut().unwrap().borrow_mut().right.as_mut().unwrap().borrow_mut().right = Option::Some(Rc::new(RefCell::new(TreeNode::new(1))));

    println!("expected 1, got {}", Solution::psuedo_palindromic_paths(root));

    let root = Option::Some(Rc::new(RefCell::new(TreeNode::new(9))));
    
    println!("expected 1, got {}", Solution::psuedo_palindromic_paths(root));
}

struct Solution;
impl Solution {
    pub fn psuedo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut num_paths = 0;
        let mut path = Vec::new();
        Solution::dfs(root, &mut path, &mut num_paths);
        num_paths
    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, num_paths: &mut i32) {
        if let Some(node) = root {
            path.push(node.borrow().val);
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                if Solution::is_palindrome(path) {
                    *num_paths += 1;
                }
            } else {
                Solution::dfs(node.borrow().left.clone(), path, num_paths);
                Solution::dfs(node.borrow().right.clone(), path, num_paths);
            }
            path.pop();
        }
    }

    pub fn is_palindrome(path: &Vec<i32>) -> bool {
        let mut counts = [0; 10];
        for i in path {
            counts[*i as usize] += 1;
        }
        let mut odd_count = 0;
        for i in counts.iter() {
            if *i % 2 == 1 {
                odd_count += 1;
            }
        }
        odd_count <= 1
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}
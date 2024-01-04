use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
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

fn main() {
    let obj = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    obj.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    obj.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    check_tree(obj);
}

fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // if root value is equal to sum of left and right, return true
    // else return false
    let mut result = false;
    let mut left = 0;
    let mut right = 0;
    if let Some(node) = root {
        let sum = node.borrow().val;
        if let Some(left_node) = node.borrow().left.clone() {
            left = left_node.borrow().val;
        }
        if let Some(right_node) = node.borrow().right.clone() {
            right = right_node.borrow().val;
        }
        if sum == left + right {
            result = true;
        }
    }

    println!("result: {}", result);
    result
}
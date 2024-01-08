use std::{rc::Rc, cell::RefCell};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode{
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

fn main() {
    let mut root = TreeNode::new(10);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    root.left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    root.right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(18))));
    dbg!(&root);
    let result = Solution::range_sum_bst(Some(Rc::new(RefCell::new(root))), 7, 15);


    println!("result: {}", result);
}

struct Solution {}
impl Solution {
    /**
     * given the root node of a binary search tree and two integers low and high,
     * return the sum of values of all nodes with a value in the inclusive range.
     * 
     * first test: [10,5,15,3,7,null,18], low = 7, high = 15, expected output 32
     * the range is inclusive, so 7 and 15 are included in the range
     * 10  + 15 + 7 = 32, skipping 5, 3, null, 18
     * 
     * this function will pop the root node from the stack, check if it is within the range
     * given, add the value to the total if it has a value, then attempt to
     * push into the vector stack the left and right nodes of the current node,
     * if they exist. then it will attempt to pop one of the pushed nodes, and
     * repeat the process until the stack is empty. the total will be returned
     * 
     */
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut total = 0; // initialize total to 0
        let mut add_stack = vec![root]; // create a stack, add the root node to it
        while let Some(current_node) = add_stack.pop() { // while the stack is not empty (exits if it cannot pop)
            if let Some(node_exists) = current_node { // if it pops a non-null node
                let node = node_exists.borrow(); // borrow the node to avoid moving it
                if node.val >= low && node.val <= high { // if the node's value is within the range
                    total += node.val; // add the node's value to the total
                }
                add_stack.push(node.left.clone()); // push the left node to the stack (if it exists)
                add_stack.push(node.right.clone()); // push the right node to the stack (if it exists)
            }
        }

        total
    }
}
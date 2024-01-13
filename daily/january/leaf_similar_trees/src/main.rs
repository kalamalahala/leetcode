use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut first_tree = TreeNode::new(3);
    first_tree.left = make_node(5);
    first_tree.right = make_node(1);
    first_tree.left.as_ref().unwrap().borrow_mut().left = make_node(6);
    first_tree.left.as_ref().unwrap().borrow_mut().right = make_node(2);
    first_tree
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = make_node(7);
    first_tree
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = make_node(4);
    first_tree.right.as_ref().unwrap().borrow_mut().left = make_node(9);
    first_tree.right.as_ref().unwrap().borrow_mut().right = make_node(8);

    let mut second_tree = TreeNode::new(3);
    second_tree.left = make_node(5);
    second_tree.right = make_node(1);
    second_tree.left.as_ref().unwrap().borrow_mut().left = make_node(6);
    second_tree.left.as_ref().unwrap().borrow_mut().right = make_node(7);
    second_tree.right.as_ref().unwrap().borrow_mut().left = make_node(4);
    second_tree.right.as_ref().unwrap().borrow_mut().right = make_node(2);
    second_tree
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = make_node(9);
    second_tree
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = make_node(8);

    // println!("First tree: {:?}", first_tree);
    // println!("Second tree: {:?}", second_tree);

    println!(
        "Result: {}",
        Solution::leaf_similar(
            Some(Rc::new(RefCell::new(first_tree))),
            Some(Rc::new(RefCell::new(second_tree)))
        )
    );
}

fn make_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

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
            right: None,
        }
    }
}

struct Solution;
impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::get_leafs(root1) == Self::get_leafs(root2)
    }

    fn get_leafs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // place root of tree in stack
        let mut stack = vec![root];
        // create vector to store leafs once found
        let mut leaf_vec: Vec<i32> = Vec::new();

        // while stack is not empty
        while let Some(node) = stack.pop() {
            // if node is not None
            if let Some(node) = node {
                // check if the node has no children
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    // if no children, push value to leaf_vec
                    leaf_vec.push(node.borrow().val);
                } else {
                    // if node has children, push children to stack with take() to remove from node
                    stack.push(node.borrow_mut().left.take());
                    stack.push(node.borrow_mut().right.take());
                }
            }
        }

        // above loop ends with a stack is empty and all nodes have been checked
        leaf_vec
    }
}

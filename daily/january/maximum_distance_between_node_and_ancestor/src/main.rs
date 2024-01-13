use std::{cell::RefCell, rc::Rc};

// leetcode definition of binary tree
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    // #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn main() {
    /*
       The 'maximum difference' is the abs(node a - node b) where a and b are nodes in the tree,
       and a and b are ancestrally related. The 'maximum difference' is the largest of all possible
       differences between nodes in the tree that are ancestrally related.
    */

    // failed case [2,4,3,1,null,0,5,null,6,null,null,null,7] expected output 5
    let root = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    // right, right, right
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    println!(
        "Tree 0 max ancestor diff: {}",
        Solution::max_ancestor_diff(root)
    );

    // tree 1 [8,3,10,1,6,null,14,null,null,4,7,13] expected output 7
    let root = Some(Rc::new(RefCell::new(TreeNode::new(8))));
    root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(10))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(14))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(13))));

    println!(
        "Tree 1 max ancestor diff: {}",
        Solution::max_ancestor_diff(root)
    );

    // tree 2 [1,null,2,null,0,3] expected output 3
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    println!(
        "Tree 2 max ancestor diff: {}",
        Solution::max_ancestor_diff(root)
    );
}

struct Solution;
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // init values
        let mut max_difference = 0;
        let min_init = i32::MAX;
        let max_init = i32::MIN;

        // recurse
        Solution::min_max_dfs(&root, min_init, max_init, &mut max_difference);

        // return
        max_difference
    }

    fn min_max_dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        minimum: i32,
        maximum: i32,
        max_difference: &mut i32,
    ) {
        if let Some(node) = root {

            // get node and value
            let node = node.borrow();
            let node_val = node.val;

            // update max difference if necessary
            if minimum != i32::MAX && maximum != i32::MIN {
                let diff = std::cmp::max((node_val - minimum).abs(), (node_val - maximum).abs());
                *max_difference = std::cmp::max(*max_difference, diff);
            }

            // recurse left and right
            Solution::min_max_dfs(
                &node.left,
                std::cmp::min(node_val, minimum),
                std::cmp::max(node_val, maximum),
                max_difference,
            );
            Solution::min_max_dfs(
                &node.right,
                std::cmp::min(node_val, minimum),
                std::cmp::max(node_val, maximum),
                max_difference,
            );
        }
    }
}

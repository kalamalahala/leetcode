use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // // create tree [1,5,3,null,4,10,6,9,2] root
    // let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    // // layer 1
    // root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    // root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    // // layer 2
    // root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    // root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(10))));
    // root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));

    // // layer 3
    // root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    // root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    // println!("time to infection: {} - expected 4", Solution::amount_of_time(root, 3));

    // let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    // println!("time to infection: {}", Solution::amount_of_time(root, 1));

    // // add failed test case [1, null, 2, null, 3, null, 4, null, 5] with infection at 4

    // let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    // root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    // root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    // root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    // println!("time to infection: {}", Solution::amount_of_time(root, 4));

    // // add failed test case [1, 2, null, 3, null, 4, null, 5] with infection at 1

    // let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    // root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    // root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    // root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    // println!("time to infection: {}", Solution::amount_of_time(root, 1));

    // // add failed test case [1, null, 2, 3, 4, null, 5] infection 4
    // let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    // root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    // root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    // root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    // println!("time to infection: {}", Solution::amount_of_time(root, 4));

    // // add failed test case [1, 2, null, 3, null, 4, null, 5] infection 2 expect 3
    // let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    // root.as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left
    //     .as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    // root.as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left
    //     .as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left
    //     .as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    // root.as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left
    //     .as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left
    //     .as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left
    //     .as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    // println!(
    //     "time to infection: {} - expected 3",
    //     Solution::amount_of_time(root, 2)
    // );

    // // add failed test case [1, 2, null, 3, null, 4, null, 5] infection 4 expect 3
    // let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    // root.as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left
    //     .as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    // root.as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left
    //     .as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left
    //     .as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    // root.as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left
    //     .as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left
    //     .as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left
    //     .as_ref()
    //     .unwrap()
    //     .borrow_mut()
    //     .left = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    // println!(
    //     "time to infection: {} - expected 3",
    //     Solution::amount_of_time(root, 4)
    // );

    // // add failed test case [5, 2, 3, 4, null, null, null, 1]
    // let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    // root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    // root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    // root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    // root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    // println!(
    //     "time to infection: {} - expected 3",
    //     Solution::amount_of_time(root, 4)
    // );

    // add failed test case [1, null, 2, 3, 4, null, 5] infection 4 expect 3
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    println!(
        "time to infection: {} - expected 3",
        Solution::amount_of_time(root, 4)
    );
}

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

/**
 *  You are given the root of a binary tree with *unique* values, and an integer 'start'. At minute '0',
 *  an *infection* starts from the node with the value 'start'. Each minute, a node becomes infected if
 *  - the node is currently uninfected
 *  - the node is adjacent to an infected node
 *
 *  Return the number of minutes needed for the entire tree to be infected.
 */
struct Solution;
impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut left_tree_data: Vec<i32> = vec![0, 0, 0]; // [is_infected, infection_depth, current_layer]
        let mut right_tree_data: Vec<i32> = vec![0, 0, 0]; // [is_infected, infection_depth, current_layer]

        // get left tree data
        let left_tree = root.as_ref().unwrap().borrow().left.clone();
        // get right tree data
        let right_tree = root.as_ref().unwrap().borrow().right.clone();

        // if only root, return zero
        if left_tree.is_none() && right_tree.is_none() {
            return 0;
        }

        let mut left_tree_max = 0;
        let mut right_tree_max = 0;

        if left_tree.is_some() {
            left_tree_max = Solution::dfs_yay(left_tree, start, &mut left_tree_data, 1);
        }

        if right_tree.is_some() {
            right_tree_max = Solution::dfs_yay(right_tree, start, &mut right_tree_data, 1);
        }

        if left_tree_data[0] == 1 {
            // infection is in left tree

            // check if depth of infection is greater than max depth of right tree
            if left_tree_data[1] > right_tree_max {
                // the infection time is the maximum of infection depth and root depth, versus infection depth and left tree depth

                // if infection depth is more than halfway down
                if left_tree_data[1] > left_tree_max / 2 {
                    return left_tree_data[1] + right_tree_max;
                }



                std::cmp::max(
                    (0..left_tree_data[1]).len() as i32,
                    (left_tree_data[1]..left_tree_max).len() as i32)
            } else {
                std::cmp::max(
                    left_tree_max - left_tree_data[1],
                    right_tree_max + left_tree_data[1],
                )
            }
        } else if right_tree_data[0] == 1 {
            if right_tree_data[1] > left_tree_max {

                if right_tree_data[1] > right_tree_max / 2 {
                    return right_tree_data[1] + left_tree_max;
                }

                std::cmp::max(
                    (0..right_tree_data[1]).len() as i32,
                    (right_tree_data[1]..right_tree_max).len() as i32)
            } else {
                std::cmp::max(
                    right_tree_max - right_tree_data[1],
                    left_tree_max + right_tree_data[1],
                )
            }
        } else {
            // infection is in root
            std::cmp::max(left_tree_max, right_tree_max)
        }
    }

    fn dfs_yay(
        node: Option<Rc<RefCell<TreeNode>>>,
        infected_node: i32,
        tree_data: &mut [i32],
        current_layer: i32,
    ) -> i32 {
        tree_data[2] = current_layer;
        // get node value
        let node = node.unwrap();
        let node = node.borrow();
        if node.val == infected_node {
            tree_data[0] = 1; // set is_infected to true
            tree_data[1] = tree_data[2]; // set infection_depth to current_layer
        }

        // get left tree data
        let left_tree = node.left.clone();
        // get right tree data
        let right_tree = node.right.clone();

        // set max_depth tracker to current max_depth in tree_data
        let mut max_depth = tree_data[2];

        if left_tree.is_some() {
            max_depth = std::cmp::max(
                max_depth,
                Solution::dfs_yay(left_tree, infected_node, tree_data, current_layer + 1),
            );
        }

        if right_tree.is_some() {
            max_depth = std::cmp::max(
                max_depth,
                Solution::dfs_yay(right_tree, infected_node, tree_data, current_layer + 1),
            );
        }

        max_depth
    }
}

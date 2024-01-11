use std::cell::RefCell;
use std::rc::Rc;

fn main() {

    // most recent failed case [1,2,null,3,null,4,null,5] start 5, expect 4
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    println!("time to infection: {} - expected 4", Solution::amount_of_time(root, 5));


    // create tree [1,5,3,null,4,10,6,9,2] root
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    // layer 1
    root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    // layer 2
    root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(10))));
    root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));

    // layer 3
    root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    println!("time to infection: {} - expected 4", Solution::amount_of_time(root, 3));

    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    println!("time to infection: {}", Solution::amount_of_time(root, 1));

    // add failed test case [1, null, 2, null, 3, null, 4, null, 5] with infection at 4

    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    println!("time to infection: {}", Solution::amount_of_time(root, 4));

    // add failed test case [1, 2, null, 3, null, 4, null, 5] with infection at 1

    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    println!("time to infection: {}", Solution::amount_of_time(root, 1));

    // add failed test case [1, null, 2, 3, 4, null, 5] infection 4
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    println!("time to infection: {}", Solution::amount_of_time(root, 4));

    // add failed test case [1, 2, null, 3, null, 4, null, 5] infection 2 expect 3
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
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
        .left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
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
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    println!(
        "time to infection: {} - expected 3",
        Solution::amount_of_time(root, 2)
    );

    // add failed test case [1, 2, null, 3, null, 4, null, 5] infection 4 expect 3
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
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
        .left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
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
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    println!(
        "time to infection: {} - expected 3",
        Solution::amount_of_time(root, 4)
    );

    // add failed test case [5, 2, 3, 4, null, null, null, 1]
    let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    println!(
        "time to infection: {} - expected 3",
        Solution::amount_of_time(root, 4)
    );

    // add failed test case [1, null, 2, 3, 4, null, 5] infection 4 expect 3
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
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
        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();

        if left.is_none() && right.is_none() {
            return 0;
        }

        let mut left_data = (0, -1, 0);
        let mut right_data = (0, -1, 0); // (max_depth, depth_of_infection, furthest_node)

        if left.is_some() {
            Solution::new_dfs(left, start, &mut left_data, 1);
        }

        if right.is_some() {
            Solution::new_dfs(right, start, &mut right_data, 1);
        }

        println!("left: {:?}", left_data);
        println!("right: {:?}", right_data);

        // data.0 = max depth of tree
        // data.1 = depth of infection in tree

        // do math
        // if left_data.1 > 0 {
        //     if left_data.1 > left_data.0 {
        //         left_data.1 = left_data.0; // if infection depth is greater than max depth, set infection depth to max depth
        //     }
        //     let arm_length = std::cmp::max(left_data.1, left_data.0 - left_data.1); // which is longer, depth to infection, or depth from infection to max depth
        //     std::cmp::max(arm_length, left_data.1 + right_data.0) // 
        // } else if right_data.1 > 0 {
        //     if right_data.1 > right_data.0 {
        //         right_data.1 = right_data.0;
        //     }
        //     let arm_length = std::cmp::max(right_data.1, right_data.0 - right_data.1);
        //     std::cmp::max(arm_length, right_data.1 + left_data.0)
        // } else {
        //     std::cmp::max(left_data.0, right_data.0)
        // }

        // try this
        if left_data.1 > 0 { // infection is in left tree
            if left_data.1 > left_data.0 { // infection depth is greater than max depth
                left_data.1 = left_data.0; // set infection depth to max depth
            }

            // return the max of furthest node in tree, or the infection depth + other tree max
            std::cmp::max(right_data.0 + left_data.1, left_data.2)            
        } else if right_data.1 > 0 { // infection is in right tree
            if right_data.1 > right_data.0 { // infection depth is greater than max depth
                right_data.1 = right_data.0; // set infection depth to max depth
            }

            // return the max of furthest node in tree, or the infection depth + other tree max
            std::cmp::max(left_data.0 + right_data.1, right_data.2)
        } else { // no infection
            std::cmp::max(left_data.0, right_data.0)
        }


    }

    /**
     * mkayander - leetcode
     * 1. At each recursion, return current height and distance from start node
     * 2. when we find the start node, we know the depth, so return start distance + 1 so we know the distance to start from each parent
     * 3. if we receive a distance to start node, calculate new value
     *  max(left_max_depth + right_start_dist, left_start_dist + right_max_depth, start_depth)
     */
    fn new_dfs(node: Option<Rc<RefCell<TreeNode>>>, infected_value: i32, tree_data: &mut (i32, i32, i32), current_depth: i32) -> (i32, i32, i32) {
        // update current max depth
        if current_depth > tree_data.0 {
            tree_data.0 = current_depth;
        }

        // collect node
        let node = node.unwrap();
        let node = node.borrow();
        let val = node.val;
        let left = node.left.clone();
        let right = node.right.clone();
        // let mut leaf_modifier = 0;

        // if left.is_none() && right.is_none() {
        //     leaf_modifier += 1;
        // }

        if val == infected_value {
            // set depth of infection to current depth
            tree_data.1 = current_depth;
            tree_data.2 = 1;
        }

        // did we just come from the start node?
        if tree_data.2 == 1 {
            // if so, set the furthest node to the current depth
            tree_data.2 = current_depth;
        }

        // otherwise, if we already have a furthest node, check if the current depth is greater
        if tree_data.2 > 0 && current_depth > tree_data.2 {
            // if so, set the furthest node to the current depth
            tree_data.2 = current_depth;
        }


        if left.is_some() {
            Solution::new_dfs(left, infected_value, tree_data, current_depth + 1);
        }

        if right.is_some() {
            Solution::new_dfs(right, infected_value, tree_data, current_depth + 1);
        }


        *tree_data
    }
}

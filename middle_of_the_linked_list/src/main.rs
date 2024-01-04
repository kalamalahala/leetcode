// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn main() {
    let mut list = Some(Box::new(ListNode::new(1)));
    list.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    let second_node = &mut list.as_mut().unwrap().next;
    second_node.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    let third_node = &mut second_node.as_mut().unwrap().next;
    third_node.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    let fourth_node = &mut third_node.as_mut().unwrap().next;
    fourth_node.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));

    dbg!(&list);
    println!("The middle of the linked list is: {:?}", middle_node(list));
}

fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = &head;
    let mut fast = &head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }

    slow.clone()

}

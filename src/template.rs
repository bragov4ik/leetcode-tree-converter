use crate::TreeNode;
use crate::Solution;
// template start
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomTreeNode {
    pub unique_id: u32,
    pub val: i32,
    pub parent: Option<Rc<RefCell<CustomTreeNode>>>,
    pub left: Option<Rc<RefCell<CustomTreeNode>>>,
    pub right: Option<Rc<RefCell<CustomTreeNode>>>,
}

enum ChildSide {
    Left,
    Right,
}
struct InsertChild {
    pub parent: Rc<RefCell<CustomTreeNode>>,
    pub child: Rc<RefCell<TreeNode>>,
    pub side: ChildSide,
}

impl CustomTreeNode {
    fn set_child(&mut self, node: Rc<RefCell<CustomTreeNode>>, side: ChildSide) {
        match side {
            ChildSide::Left => self.left = Some(node),
            ChildSide::Right => self.right = Some(node),
        };
    }

    fn from_bruh(value: Rc<RefCell<TreeNode>>) -> Rc<RefCell<Self>> {
        fn add_children_to_queue(
            queue: &mut Vec<InsertChild>,
            node: Rc<RefCell<TreeNode>>,
            custom_node: Rc<RefCell<CustomTreeNode>>,
        ) {
            if let Some(left) = &node.borrow().left {
                queue.push(InsertChild {
                    parent: custom_node.clone(),
                    child: left.clone(),
                    side: ChildSide::Left,
                });
            };
            if let Some(right) = &node.borrow().right {
                queue.push(InsertChild {
                    parent: custom_node,
                    child: right.clone(),
                    side: ChildSide::Right,
                });
            };
        }

        let mut next_id = 0;
        let root = Rc::new(RefCell::new(CustomTreeNode {
            unique_id: next_id,
            val: value.borrow().val,
            parent: None,
            left: None,
            right: None,
        }));
        next_id += 1;
        let mut children_to_add = vec![];
        add_children_to_queue(&mut children_to_add, value, root.clone());
        while let Some(next_child) = children_to_add.pop() {
            let custom_child = Rc::new(RefCell::new(CustomTreeNode {
                unique_id: next_id,
                val: next_child.child.borrow().val,
                parent: Some(next_child.parent.clone()),
                left: None,
                right: None,
            }));
            next_id += 1;
            next_child
                .parent
                .borrow_mut()
                .set_child(custom_child.clone(), next_child.side);
            add_children_to_queue(&mut children_to_add, next_child.child, custom_child);
        }
        root
    }
}

impl Solution {
    pub fn solution_name() -> i32 {
        0
    }
}

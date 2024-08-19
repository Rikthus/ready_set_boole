use std::{cell::RefCell, iter::Rev, rc::Rc, str::Chars};

type TreeNodeRef = Rc<RefCell<TreeNode>>;
#[derive(Debug, Clone)]
pub struct TreeNode {
    val: char,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

fn build_node(iterator: &mut Rev<Chars<'_>>) -> Option<TreeNodeRef> {
    if let Some(val) = iterator.next() {
        let mut left = Default::default();
        let mut right = Default::default();
        if val == '&' || val == '|' || val == '^' || val == '=' {
            left = build_node(iterator);
            right = build_node(iterator);
        } else if val == '!' || val == '>' {
            left = build_node(iterator);
        }
        let node = TreeNode { val, left, right };
        Some(Rc::new(RefCell::from(node)))
    } else {
        None
    }
}

fn evaluate_node(node: TreeNode) -> bool {
    match node.val {
        '|' => return evaluate_node(node.left) | evaluate_node(node.right),
        '&' => return evaluate_node(node.left) & evaluate_node(node.right),
        '^' => return evaluate_node(node.left) ^ evaluate_node(node.right),
        '=' => return evaluate_node(node.left) == evaluate_node(node.right),
        '!' => return ! evaluate_node(node.left),
        // '>' ?
        '1' => return true,
        '0' => return false,
        _ => return false,
    }
}

fn eval_formula(formula: &str) -> bool {
    let tree = build_node(&mut formula.chars().rev());
    // println!("{tree:?}");
    if tree.is_some() {
        evaluate_node(tree)
    } else {
        false
    }
}

fn main() {
    let result = eval_formula("10|");
    println!("{result}");
}

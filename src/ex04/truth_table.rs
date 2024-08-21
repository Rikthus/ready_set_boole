use std::{iter::Rev, str::Chars};

type TreeNodeRef = Box<TreeNode>;
#[derive(Debug, Clone)]
pub struct TreeNode {
    val: char,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

#[derive(Debug)]
struct UndefinedBehavior;

fn build_node(iterator: &mut Rev<Chars<'_>>) -> Option<TreeNodeRef> {
    if let Some(val) = iterator.next() {
        let mut left = Default::default();
        let mut right = Default::default();
        if val == '&' || val == '|' || val == '^' || val == '=' || val == '>' {
            left = build_node(iterator);
            right = build_node(iterator);
        } else if val == '!' {
            left = build_node(iterator);
        }
        let node = TreeNode { val, left, right };
        Some(Box::from(node))
    } else {
        None
    }
}

fn evaluate_node(node: Option<TreeNodeRef>) -> Result<bool, UndefinedBehavior> {
    if node.is_some() {
        let unwrapped_node = node.unwrap();
        match unwrapped_node.val {
            '|' => return Ok(evaluate_node(unwrapped_node.left)? | evaluate_node(unwrapped_node.right)?),
            '&' => return Ok(evaluate_node(unwrapped_node.left)? & evaluate_node(unwrapped_node.right)?),
            '^' => return Ok(evaluate_node(unwrapped_node.left)? ^ evaluate_node(unwrapped_node.right)?),
            '=' => return Ok(evaluate_node(unwrapped_node.left)? == evaluate_node(unwrapped_node.right)?),
            '>' => return Ok(!evaluate_node(unwrapped_node.left)? | evaluate_node(unwrapped_node.right)?),
            '!' => return Ok(!evaluate_node(unwrapped_node.left)?),
            '1' => return Ok(true),
            '0' => return Ok(false),
            _ => return Err(UndefinedBehavior),
        }
    } else {
        Err(UndefinedBehavior)
    }
}

fn count_nb_variables(formula: &str) -> usize {
    let var_lst: Vec<char> = Vec::new();
    
    for c in formula.chars() {
        if var_lst.contains(c) {

        }
    }

    var_lst.len()
}

fn evaluate_truth_combinations(formula: &str) -> bool {
    // create_gray_num
    // while all values of vec are not 1
    // clone tree
    // replace variables with vec values
    // push new vec of vec comb values and evaluate result in a solutions vec
    let tree_root = build_node(&mut formula.chars().rev());
    let result = evaluate_node(tree_root);
    if result.is_ok() {
        result.unwrap()
    } else {
        println!("Undefined Behavior");
        false
    }
}


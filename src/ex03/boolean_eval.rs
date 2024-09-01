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
            '|' => {
                return Ok(
                    evaluate_node(unwrapped_node.left)? | evaluate_node(unwrapped_node.right)?
                )
            }
            '&' => {
                return Ok(
                    evaluate_node(unwrapped_node.left)? & evaluate_node(unwrapped_node.right)?
                )
            }
            '^' => {
                return Ok(
                    evaluate_node(unwrapped_node.left)? ^ evaluate_node(unwrapped_node.right)?
                )
            }
            '=' => {
                return Ok(
                    evaluate_node(unwrapped_node.left)? == evaluate_node(unwrapped_node.right)?
                )
            }
            '>' => {
                return Ok(
                    !evaluate_node(unwrapped_node.left)? | evaluate_node(unwrapped_node.right)?
                )
            }
            '!' => return Ok(!evaluate_node(unwrapped_node.left)?),
            '1' => return Ok(true),
            '0' => return Ok(false),
            _ => return Err(UndefinedBehavior),
        }
    } else {
        Err(UndefinedBehavior)
    }
}

fn check_characters(formula: &str) -> bool {
    let symbols = ['&', '|', '^', '=', '>', '!', '1', '0'];

    for c in formula.chars() {
        if !symbols.contains(&c) {
            return false;
        }
    }
    true
}

fn check_logic(formula: &str) -> bool {
    let mut nb_values = 0;
    let mut nb_ops = 0;

    for c in formula.chars() {
        if c == '1' || c == '0' {
            nb_values += 1;
        } else if c != '!' {
            nb_ops += 1;
        }
    }
    if nb_values - 1 != nb_ops {
        return false;
    }
    true
}

fn parse_formula(formula: &str) -> bool {
    if !check_characters(formula) {
        return false;
    } else if !check_logic(formula) {
        return false;
    }
    true
}

fn evaluate_formula(formula: &str) -> Result<bool, UndefinedBehavior> {
    if !parse_formula(formula) {
        return Err(UndefinedBehavior);
    }
    let tree_root = build_node(&mut formula.chars().rev());
    let result = evaluate_node(tree_root);
    result
}

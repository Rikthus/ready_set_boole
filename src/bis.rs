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

fn parse_characters(formula: &str) -> bool {
    let symbols = ['&', '|', '^', '=', '>', '!'];

    for c in formula.chars() {
        if !symbols.contains(&c) && (!c.is_alphabetic() || !c.is_uppercase()) {
            return false;
        }
    }
    true
}

fn count_nb_variables(formula: &str) -> usize {
    let mut var_lst: Vec<char> = Vec::new();
    
    for c in formula.chars() {
        if c.is_alphabetic() && !var_lst.contains(&c) {
            var_lst.push(c);
        }
    }
    var_lst.len()
}

fn  gray_code(n: u32) -> u32 {
    if n == 0{
        return 0;
    }

    let a: u32 = n % 2;
    let b: u32 = (n >> 1) % 2;

    return (a ^ b) + (gray_code(n >> 1) << 1);
}

fn  generate_comb_values(curr_comb: u32) -> Vec<bool> {
    let mut comb_values: Vec<bool> = Vec::new();
    let mut g_code = gray_code(curr_comb);

    while g_code != 0 {
        if g_code ^ 1 == 0 {
            
        }
        g_code = g_code << 1;
    }
    comb_values
}

fn generate_truth_combinations(formula: &str) -> Result<Vec<Vec<bool>>, UndefinedBehavior> {
    let mut truth_table: Vec<Vec<bool>> = Vec::new();
    let nb_var = count_nb_variables(formula);
    let max_comb = u32::pow(2, nb_var.try_into().unwrap());
    let mut curr_comb: u32 = 0;

    if !parse_characters(formula) {
        return Err(UndefinedBehavior);
    }
    let tree_root = TreeNode {
        val: '_',
        left: build_node(&mut formula.chars().rev()),
        right: None,
    };
    while curr_comb < max_comb {
        let mut tmp_tree = tree_root.clone();
        let mut comb_values = generate_comb_values(curr_comb);
        // replace variables by comb_values
        let result = evaluate_node(tmp_tree.left);

        if result.is_ok() {
            comb_values.push(result.unwrap());
            truth_table.push(comb_values);
        } else {
            return Err(UndefinedBehavior);
        }
        curr_comb += 1;
    }
    Ok(truth_table)
}

fn print_truth_table(_table: Vec<Vec<bool>>) {
    println!("ok");
}

fn main() {
    let result = generate_truth_combinations("10>");
    if result.is_ok() {
        print_truth_table(result.unwrap());
    } else {
        println!("Undefined Behavior");
    }
}

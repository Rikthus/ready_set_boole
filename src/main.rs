use std::{iter::Rev, str::Chars};

type TreeNodeRef = Box<TreeNode>;
#[derive(Debug, Clone)]
pub struct TreeNode {
    val: char,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

fn check_characters(formula: &str) -> bool {
   let  symbols = ['&', '|', '^', '>', '!', '='];

   for c in formula.chars() {
        if !symbols.contains(&c) && (!c.is_alphabetic() || !c.is_uppercase()) {
            return false;
        }
   }
   true
}

fn check_alpha_order(formula: &str) -> bool {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut alpha_in_streak = true;
    let mut found_alpha: Vec<char> = Vec::new();

    for c in formula.chars() {
        if c.is_alphabetic() && !found_alpha.contains(&c) {
            found_alpha.push(c);
        }
    }
    for c in alphabet.chars() {
        if !found_alpha.contains(&c) && alpha_in_streak{
            alpha_in_streak = false;
        } else if found_alpha.contains(&c) && !alpha_in_streak {
            return false;
        }
    }
    true    
}

fn check_logic(formula: &str) -> bool {
    let mut nb_values = 0;
    let mut nb_ops = 0;

    for c in formula.chars() {
        if c.is_alphabetic() {
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
    } else if !check_alpha_order(formula) {
        return false;
    } else if !check_logic(formula) {
        return false;
    }
    true
}

fn  reduce_negation(formula: String) -> String {
    let mut reduced_form = String::new();
    let mut odd_neg: bool = false;
    let mut eval_neg: bool = false;

    for c in formula.chars().rev() {
        if c == '!'{
            if !eval_neg {
                eval_neg = true;
                odd_neg = true;
            } else {
                odd_neg = !odd_neg;
            }
        } else {
            if eval_neg {
                if odd_neg {
                    reduced_form.insert(0, '!');
                }
                reduced_form.insert(0, c);
                
                eval_neg = false;
                odd_neg = false;
            } else {
                reduced_form.insert(0, c);
            }
        }
    }
    reduced_form
}

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

fn  convert_to_nnf(node: Option<TreeNodeRef>, formula: &mut String, is_neg: bool) {
    let unwrapped_node = node.unwrap();
    let left = unwrapped_node.left;
    let right = unwrapped_node.right;
    let val = unwrapped_node.val;

    match val {
        '|' => {
            if is_neg {
                formula.insert(0, '&');
                convert_to_nnf(left, formula, true);
                convert_to_nnf(right, formula, true);
            } else {
                formula.insert(0, '|');
                convert_to_nnf(left, formula, false);
                convert_to_nnf(right, formula, false);
            }
        }
        '&' => {
            if is_neg {
                formula.insert(0, '|');
                convert_to_nnf(left, formula, true);
                convert_to_nnf(right, formula, true);
            } else {
                formula.insert(0, '&');
                convert_to_nnf(left, formula, false);
                convert_to_nnf(right, formula, false);
            }
        }
        '>' => {
            if is_neg {
                formula.insert(0, '&');
                convert_to_nnf(left, formula, false);
                convert_to_nnf(right, formula, true);
            } else {
                formula.insert(0, '|');
                convert_to_nnf(left, formula, false);
                convert_to_nnf(right, formula, true);
            }
        }
        '=' => {
            if is_neg {
                formula.insert(0, '&');

                formula.insert(0, '|');
                convert_to_nnf(left.clone(), formula, false);
                convert_to_nnf(right.clone(), formula, false);

                formula.insert(0, '|');
                convert_to_nnf(left, formula, true);
                convert_to_nnf(right, formula, true);
            } else {
                formula.insert(0, '|');

                formula.insert(0, '&');
                convert_to_nnf(left.clone(), formula, true);
                convert_to_nnf(right.clone(), formula, true);

                formula.insert(0, '&');
                convert_to_nnf(left, formula, false);
                convert_to_nnf(right, formula, false);                
            }
        }
        // UNSURE
        // (!A & B) | (A & !B)

        // !((!A & B) | (A & !B))
        // !(!A & B) & !(A & !B) DE morgans laws

        // (A | !B) & (!A | B)
        '^' => {
            if is_neg {
                formula.insert(0, '&');

                formula.insert(0, '|');
                convert_to_nnf(left.clone(), formula, true);
                convert_to_nnf(right.clone(), formula, false);
                
                formula.insert(0, '|');
                convert_to_nnf(left, formula, false);
                convert_to_nnf(right, formula, true);                
            } else {
                formula.insert(0, '|');
                
                formula.insert(0, '&');
                convert_to_nnf(left.clone(), formula, false);
                convert_to_nnf(right.clone(), formula, true);
                
                formula.insert(0, '&');
                convert_to_nnf(left, formula, true);
                convert_to_nnf(right, formula, false);
            }
        }
        '!' => {
            if is_neg {
                convert_to_nnf(left, formula, false);
            } else {
                convert_to_nnf(left, formula, true)
            }
        }
        _ => {
            if is_neg {
                formula.insert(0, '!');
            }
            formula.insert(0, val);
        }
    }
}

fn negation_normal_form(formula: &str) -> String {
    if !parse_formula(formula) {
        println!("Invalid formula");
        return "".to_string();
    }

    let pre_nnf = reduce_negation(formula.to_string());
    let tree_root = build_node(&mut pre_nnf.chars().rev());
    let mut nnf = String::new();
    convert_to_nnf(tree_root, &mut nnf, false);

    return nnf;
}
fn main() {
    let nnf = negation_normal_form("AB^!");
    println!("{}", nnf);
}

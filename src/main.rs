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

#[derive(Debug)]
struct InvalidFormula;


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
    // for val in found_alpha.iter() {
    //     println!("{}", val);
    // }
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

fn count_nb_variables(formula: &str) -> usize {
   let  mut var_list: Vec<char> = Vec::new();

   for c in formula.chars() {
        if c.is_alphabetic() && !var_list.contains(&c) {
            var_list.push(c);
        }
   }

   return var_list.len();
}

fn  fill_comb_vec(comb_vec: &mut Vec<bool>, comb: u32, nb_vars: usize) {
    if nb_vars == 0 {
        return;
    }

    fill_comb_vec(comb_vec, comb >> 1, nb_vars - 1);
    comb_vec.push(if comb & 1 == 1 {true} else {false});
}

fn  generate_combinations(formula: &str) -> Vec<Vec<bool>> {
    let nb_vars: usize = count_nb_variables(formula);
    let nb_combs = u32::pow(2, nb_vars.try_into().unwrap());
    let mut curr_comb: u32 = 0;
    let mut comb_list: Vec<Vec<bool>> = Vec::new();

    while curr_comb < nb_combs {
        let mut comb_vec: Vec<bool> = Vec::new();

        fill_comb_vec(&mut comb_vec, curr_comb, nb_vars);
        comb_list.push(comb_vec);
        curr_comb += 1;
    }

    return comb_list;
}

fn  print_combs(combs: Vec<Vec<bool>>) {
    let nb_vars = combs[0].len();

    for (i, alpha) in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().enumerate() {
        if i + 1 == nb_vars {
            break;
        }
        print!("| {} ", alpha);
    }
    println!("| = |");

    for i in 0..nb_vars {
        print!("|---");
    }
    println!("|");
    for comb in combs {
        for val in comb {
            print!("| {} ", val as i32);
        }
        println!("|");
    }
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

fn get_value(c: char, comb: &Vec<bool>) -> char {
    for (i, alpha) in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().enumerate() {
        if c == alpha {
            let mut j = 0;
            for value in comb {
                if j == i {
                    if *value == true {
                        return '1';
                    } else {
                        return '0';
                    }
                }
                j += 1;
            }
        }
    }
    '0'
}

fn substitute_formula(formula: &str, comb: &Vec<bool>) -> String {
    let mut comb_formula: String = String::new();

    for c in formula.chars() {
        if c.is_alphabetic() {
            comb_formula.push(get_value(c, comb));
        } else {
            comb_formula.push(c);
        }
    }
    comb_formula
}

fn generate_truth_table(formula: &str) -> Result<Vec<Vec<bool>>, InvalidFormula> {
    let mut truth_table: Vec<Vec<bool>> = Vec::new();
    if !parse_formula(formula) {
        return Err(InvalidFormula);
    }
    let combs = generate_combinations(formula);
    for mut comb in combs {
        let comb_formula = substitute_formula(formula, &comb);
        let tree_root = build_node(&mut comb_formula.chars().rev());
        let result = evaluate_node(tree_root);

        if result.is_ok() {
            comb.push(result.unwrap());
            truth_table.push(comb);
        } else {
            return Err(InvalidFormula);
        }
    }
    Ok(truth_table)
}

fn main() {
    let result = generate_truth_table("AB&C|");
    if result.is_ok() {
        print_combs(result.unwrap());
    } else {
        println!("BAD");
    }
}



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
    for val in found_alpha.iter() {
        println!("{}", val);
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

fn parse_formula(formula: &str) -> bool {
    if !check_characters(formula) {
        return false;
    } else if !check_alpha_order(formula) {
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

fn  generate_combinations(formula: &str) -> Vec<Vec<bool>> {
    let nb_vars: usize = count_nb_variables(formula);
    let nb_combs = u32::pow(2, nb_vars.try_into().unwrap());
    let mut curr_comb: u32 = 0;
    let mut comb_list: Vec<Vec<bool>> = Vec::new();


    let mut comb = 0;    
    while curr_comb < nb_combs {
        comb = gray_code(curr_comb);
        // for i in range nb_vars
        curr_comb += 1;
    }

    return comb_list;
}

fn generate_truth_table(formula: &str) -> Result<Vec<Vec<bool>>, InvalidFormula> {
    let mut truth_table: Vec<Vec<bool>> = Vec::new();
    if !parse_formula(formula) {
        return Err(InvalidFormula);
    }
    let combs = generate_combinations(formula);   

    Ok(truth_table)
}

fn main() {
    let result = generate_truth_table("BBBAB|");
    if result.is_ok() {
        println!("GOOD");
    } else {
        println!("BAD");
    }
}
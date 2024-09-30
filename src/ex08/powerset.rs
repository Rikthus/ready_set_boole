fn  fill_comb_vec(comb_vec: &mut Vec<bool>, comb: usize, nb_var: usize) {
    if nb_var == 0 {
        return;
    }

    fill_comb_vec(comb_vec, comb >> 1, nb_var - 1);
    comb_vec.push(if comb & 1 == 1 {true} else {false});
}

fn  generate_combinations(nb_var: usize) -> Vec<Vec<bool>> {
    let mut curr_comb: usize = 0;
    let mut comb_list: Vec<Vec<bool>> = Vec::new();

    while curr_comb < nb_var * nb_var {
        let mut comb_vec: Vec<bool> = Vec::new();

        fill_comb_vec(&mut comb_vec, curr_comb, nb_var);
        comb_list.push(comb_vec);
        curr_comb += 1;
    }

    return comb_list;
}

fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut p_set: Vec<Vec<i32>> = Vec::new();
    let all_combs: Vec<Vec<bool>> = generate_combinations(set.len());

    for comb in all_combs.iter() {
        let mut subset: Vec<i32> = Vec::new();

        for (index, value) in comb.iter().enumerate() {
            if *value == true {
                subset.push(set[index]);
            }
        }
        p_set.push(subset);
    }

    return p_set;
}

fn  main() {
    let set = vec![1, 2, 4, 33, -2];

    let p_set = powerset(set);

    for set in p_set.iter() {
        print!("{{");
        for val in set.iter() {
            print!(" {}", *val);
        }
        print!(" }}\n");
    }
}

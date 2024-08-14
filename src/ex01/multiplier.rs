fn adder(a: u32, b: u32) -> u32 {
    let mut carry = a & b;
    let mut sum = a ^ b;
    let mut tmp_carry;

    while carry != 0 {
        carry = carry << 1;

        tmp_carry = carry & sum;
        sum = carry ^ sum;
        carry = tmp_carry
    }
    carry | sum
}

fn  multiplier(a: u32, b: u32) -> u32 {
    let mut result = 0;
    let multiplied;
    let multiplicator;

    if a == 0 {
        return 0;
    }

    if a > b {
        multiplied = a;
        multiplicator = b;
    } else {
        multiplied = b;
        multiplicator = a;
    }
    for _ in 0..multiplicator {
        result = adder(result, multiplied);
    }
    result
}
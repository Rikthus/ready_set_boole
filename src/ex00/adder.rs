// Recursive adder
fn  rec_adder(a: u32, b: u32) -> u32 {
    if a & b == 0 {
        return a | b;
    }

    rec_adder((a & b) << 1, a ^ b)
}

// Iteratif adder
fn it_adder(a: u32, b: u32) -> 32 {
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
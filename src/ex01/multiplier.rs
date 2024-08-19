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
    let mut bit_index = 0;
    let mut result = 0;
    let mut control_bit = 1;

    while bit_index < 32 {
        if a & control_bit != 0 {
            result = adder(result, b << bit_index);
        }
        control_bit = control_bit << 1;
        bit_index += 1;
    }
    result
}
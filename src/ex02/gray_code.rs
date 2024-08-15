fn  gray_code(n: u32) -> u32 {
    if n == 0{
        return 0;
    }

    let a: u32 = n % 2;
    let b: u32 = (n >> 1) % 2;

    return (a ^ b) + (gray_code(n >> 1) << 1);
}
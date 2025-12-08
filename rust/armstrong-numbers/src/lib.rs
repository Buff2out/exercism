pub fn is_armstrong_number(num: u32) -> bool {
    if 0 == num {
        return true;
    }
    let count = (num as f64).log10().floor() as u32 + 1;

    let sum: u32 = (1..=count)
        .map(|i| num % 10_u32.pow(i) / 10_u32.pow(i - 1))
        .map(|x| (x).pow(count))
        .sum();
    sum == num
}

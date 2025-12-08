/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(i, sum), c| {
            c.to_digit(10).map(|d| {
                let added = if 1 == i % 2 {
                    let multiplyed = d * 2;
                    if multiplyed > 9 {
                        multiplyed - 9
                    } else {
                        multiplyed
                    }
                } else {
                    d
                };
                (i + 1, sum + added)
            })
        })
        .map_or(false, |(i, sum)| i > 1 && 0 == sum % 10)
}

use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .copied()
        .filter(|&x| 0 < x)
        .flat_map(|num| (num..limit).step_by(num as usize))
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}

pub fn egg_count(display_value: u32) -> usize {
    let mut display_value_mut = display_value;
    let mut result = 0;
    while display_value_mut != 0 {
        if 1 == display_value_mut % 2 {
            result += 1;
            display_value_mut -= 1;
        }
        display_value_mut /= 2;
    }

    result
}

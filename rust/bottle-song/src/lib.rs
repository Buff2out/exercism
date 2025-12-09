pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let end_bottles = start_bottles - take_down;
    
    ((end_bottles + 1)..=start_bottles)
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn verse(n: u32) -> String {
    let current_bottle = bottle_str(n);
    let next_bottle = bottle_str(n - 1).to_lowercase();

    format!(
        "{0} hanging on the wall,\n\
         {0} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {1} hanging on the wall.",
        current_bottle, next_bottle
    )
}

fn bottle_str(n: u32) -> String {
    let count_str = match n {
        10 => "Ten", 9 => "Nine", 8 => "Eight", 7 => "Seven", 6 => "Six",
        5 => "Five", 4 => "Four", 3 => "Three", 2 => "Two", 1 => "One",
        _ => "No",
    };

    let plural = if n == 1 { "bottle" } else { "bottles" };
    
    format!("{count_str} green {plural}")
}

pub fn raindrops(n: u32) -> String {
    let mut res = String::new();
    if 0 == n % 3 {
        res.push_str("Pling");
    }
    if 0 == n % 5 {
        res.push_str("Plang");
    }
    if 0 == n % 7 {
        res.push_str("Plong");
    }

    if res.is_empty() { n.to_string() } else { res }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets_stack = Vec::new();

    for c in string.chars() {
        match c {
            '{' | '(' | '[' => brackets_stack.push(c),
            '}' => {
                if brackets_stack.pop() != Some('{') {
                    return false;
                }
            }
            ')' => {
                if brackets_stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if brackets_stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => {}
        }
    }
    brackets_stack.is_empty()
}

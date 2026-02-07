pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred",
        "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry",
    ];

    let student_idx = students
        .iter()
        .position(|&s| s == student)
        .expect("student not found");
    
    let start = student_idx * 2;
    let end = start + 2;

    diagram
        .lines()
        .flat_map(|line| {
            line.get(start..end).unwrap_or("").chars()
        })
        .map(|c| match c {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => "",
        })
        .collect()
}

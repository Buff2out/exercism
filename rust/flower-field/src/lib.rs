pub enum Cell {
    Some(u32),
    Flower,
    None,
}

pub trait NeighbourCounter {
    fn count_neighbours(&self, x: usize, y: usize) -> u8;
}

impl NeighbourCounter for [&str] {
    fn count_neighbours(&self, y: usize, x: usize) -> u8 {
        if 0 == self.len() {
            return 0;
        }
        let mut count = 0;
        for i in y.saturating_sub(1)..=(y + 1).min(self.len() - 1) {
            for j in x.saturating_sub(1)..=(x + 1).min(self[0].len() - 1) {
                if i == y && j == x {
                    continue;
                }
                if b'*' == self[i].as_bytes()[j] {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if 0 == garden.len() {
        return vec![];
    }
    garden
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let new_line: String = line
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(j, &byte)| {
                    if b'*' == byte {
                        return '*';
                    }

                    let count = garden.count_neighbours(i, j);

                    match count {
                        0 => ' ',
                        c => (b'0' + c) as char,
                    }
                })
                .collect();
            new_line
        })
        .collect()
}

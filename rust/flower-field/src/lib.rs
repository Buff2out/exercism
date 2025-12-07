pub trait NeighbourCounter {
    fn place_cell(&self, x: usize, y: usize) -> char;
}

impl NeighbourCounter for [&str] {
    fn place_cell(&self, x: usize, y: usize) -> char {
        if b'*' == self[y].as_bytes()[x] {
            return '*';
        };
        let count = (y.saturating_sub(1)..=(y + 1).min(self.len() - 1))
            .flat_map(|ny| {
                (x.saturating_sub(1)..=(x + 1).min(self[0].len() - 1)).map(move |nx| (ny, nx))
            })
            .filter(|&(ny, nx)| b'*' == self[ny].as_bytes()[nx])
            .count();
        match count {
            0 => ' ',
            c => (b'0' + c as u8) as char,
        }
    }
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }
    (0..garden.len())
        .map(|y| {
            (0..garden[y].len())
                .map(|x| garden.place_cell(x, y))
                .collect()
        })
        .collect()
}

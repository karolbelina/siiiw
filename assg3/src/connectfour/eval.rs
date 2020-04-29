use super::{Board, Disc};

pub fn line_counter(singles: i32, doubles: i32, triples: i32, quadruples: i32) -> Box<dyn Fn(&Board) -> i32> {
    Box::new(move |board: &Board| -> i32 {
        let count = |position: (usize, usize), direction: (isize, isize)| -> (usize, usize, usize) {
            let mut yellow_count = 0;
            let mut empty_count = 0;
            let mut red_count = 0;
            for i in 0..4 {
                let x = (position.0 as isize + i * direction.0) as usize;
                let y = (position.1 as isize + i * direction.1) as usize;
                match board.columns[x].get(y) {
                    Some(&disc) => {
                        if disc == Disc::Yellow {
                            yellow_count += 1;
                        } else {
                            red_count += 1;
                        }
                    },
                    None => empty_count += 1
                }
            }
            return (yellow_count, empty_count, red_count);
        };

        let score = |grouping: (usize, usize, usize)| -> i32 {
            match grouping {
                (4, 0, 0) => quadruples,
                (3, 1, 0) => triples,
                (2, 2, 0) => doubles,
                (1, 3, 0) => singles,
                (0, 3, 1) => -singles,
                (0, 2, 2) => -doubles,
                (0, 1, 3) => -triples,
                (0, 0, 4) => -quadruples,
                _         => 0
            }
        };

        let (width, height) = (board.columns.len(), board.bound);
        let mut total_score = 0;
        
        // evaluate columns
        for x in 0..width { // [0, 6]
            for y in 0..=height - 4 { // [0, 2]
                total_score += score(count((x, y), (0, 1)));
            }
        }
        // evaluate rows
        for x in 0..=width - 4 { // [0, 3]
            for y in 0..height { // [0, 5]
                total_score += score(count((x, y), (1, 0)));
            }
        }
        // evaluate diagonals
        for x in 0..=width - 4 { // [0, 3]
            for y in 0..=height - 4 { // [0, 2]
                total_score += score(count((x, y), (1, 1)));
                total_score += score(count((x, y + 3), (1, -1)));
            }
        }

        return total_score;
    })
}

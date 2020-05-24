use super::{Board, Disc};

impl Board {
    fn count(&self, position: (usize, usize), direction: (isize, isize)) -> (usize, usize, usize) {
        let mut yellow_count = 0;
        let mut empty_count = 0;
        let mut red_count = 0;
        for i in 0..4 {
            let x = (position.0 as isize + i * direction.0) as usize;
            let y = (position.1 as isize + i * direction.1) as usize;
            match self.columns[x].get(y) {
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
    }

    fn score<F>(&self, f: F) -> i32
        where F: Fn((usize, usize, usize)) -> i32
    {
        let (width, height) = (self.columns.len(), self.bound);
        let mut total_score: i32 = 0;
        
        // evaluate columns
        for x in 0..width { // [0, 6]
            for y in 0..=height - 4 { // [0, 2]
                total_score += f(self.count((x, y), (0, 1)));
            }
        }
        // evaluate rows
        for x in 0..=width - 4 { // [0, 3]
            for y in 0..height { // [0, 5]
                total_score += f(self.count((x, y), (1, 0)));
            }
        }
        // evaluate diagonals
        for x in 0..=width - 4 { // [0, 3]
            for y in 0..=height - 4 { // [0, 2]
                total_score += f(self.count((x, y), (1, 1)));
                total_score += f(self.count((x, y + 3), (1, -1)));
            }
        }

        return total_score;
    }
}

pub fn basic() -> Box<dyn Fn(&Board) -> i32> {
    Box::new(move |board: &Board| -> i32 {
        if board.check_for_win(Disc::Yellow) {
            1
        } else if board.check_for_win(Disc::Red) {
            -1
        } else {
            0
        }
    })
}

pub fn line_counter(singles: i32, doubles: i32, triples: i32, quadruples: i32) -> Box<dyn Fn(&Board) -> i32> {
    Box::new(move |board: &Board| -> i32 {
        let scoring = |grouping: (usize, usize, usize)| -> i32 {
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

        return board.score(scoring);
    })
}

pub fn advanced(doubles: i32, triples: i32, centers: i32) -> Box<dyn Fn(&Board) -> i32> {
    Box::new(move |board: &Board| -> i32 {
        use std::i32;
        
        let scoring = |grouping: (usize, usize, usize)| -> i32 {
            match grouping {
                (3, 1, 0) => triples,
                (2, 2, 0) => doubles,
                (0, 2, 2) => -doubles,
                (0, 1, 3) => -triples,
                _         => 0
            }
        };

        let number_of_discs: i32 = board.columns.iter().map(|column| column.len() as i32).sum();

        // add or subtract number of discs to encourage winning in the fewest number of discs placed
        if board.check_for_win(Disc::Yellow) {
            i32::MAX - number_of_discs
        } else if board.check_for_win(Disc::Red) {
            i32::MIN + number_of_discs
        } else {
            // preference placing the disc in the center
            let center_discs: i32 = board.columns[board.columns.len() / 2].len() as i32;

            board.score(scoring) + center_discs * centers
        }
    })
}

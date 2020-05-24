mod utils;
mod connectfour;
mod game;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[test]
fn research() {
    use game::algorithms::{minimax, alpha_beta_pruning};
    use game::{Node};
    use connectfour::eval::{basic, line_counter, advanced};
    use connectfour::{ConnectFour, Board, DiscDrop, Disc};
    use rand::Rng;
    use std::time::{Duration, Instant};
    use std::io::{self, Write};

    let yellow_player = |board: &Board, stats: &mut usize| -> Option<DiscDrop> {
        alpha_beta_pruning::<ConnectFour, _>(board, 3, true, &advanced(10, 50, 4), stats)
    };

    let red_player = |board: &Board, stats: &mut usize| -> Option<DiscDrop> {
        alpha_beta_pruning::<ConnectFour, _>(board, 3, false, &advanced(10, 50, -20), stats)
    };

    let mut total_yellow_turn_time: Duration = Duration::new(0, 0);
    let mut total_yellow_turns: usize = 0;
    let mut total_red_turn_time: Duration = Duration::new(0, 0);
    let mut total_red_turns: usize = 0;
    let mut total_yellow_wins: usize = 0;
    let mut total_red_wins: usize = 0;
    let mut total_draws: usize = 0;
    let mut total_yellow_visited_states: usize = 0;
    let mut total_red_visited_states: usize = 0;

    println!();
    io::stdout().flush().unwrap();
    
    for i in 0..200 {
        let mut board = Board::new(7, 6);
        print!("Simulating game #{}", i + 1);
        io::stdout().flush().unwrap();
        board = board.make_decision(DiscDrop {
            column: rand::thread_rng().gen_range(0, 7),
            disc: Disc::Yellow,
        });
        total_yellow_turns += 1;
        print!(".");
        io::stdout().flush().unwrap();
        loop {
            let mut red_stats = 0;
            let red_start_time = Instant::now();
            let red_move = red_player(&board, &mut red_stats);
            total_red_turn_time += red_start_time.elapsed();
            total_red_visited_states += red_stats;
            board = board.make_decision(red_move.unwrap());
            total_red_turns += 1;
            print!(".");
            io::stdout().flush().unwrap();
            if board.check_for_win(Disc::Red) {
                total_red_wins += 1;
                break;
            }
            if board.check_for_draw() {
                total_draws += 1;
                break;
            }
            let mut yellow_stats = 0;
            let yellow_start_time = Instant::now();
            let yellow_move = yellow_player(&board, &mut yellow_stats);
            total_yellow_turn_time += yellow_start_time.elapsed();
            total_yellow_visited_states += yellow_stats;
            board = board.make_decision(yellow_move.unwrap());
            total_yellow_turns += 1;
            print!(".");
            io::stdout().flush().unwrap();
            if board.check_for_win(Disc::Yellow) {
                total_yellow_wins += 1;
                break;
            }
            if board.check_for_draw() {
                total_draws += 1;
                break;
            }
        }
        println!("done")
    }

    let total_games = total_yellow_wins + total_red_wins + total_draws;

    println!();
    println!("Liczba rozgrywek: {}", total_games);
    println!("Średni czas ruchu żółtego gracza: {:?}", total_yellow_turn_time / total_yellow_turns as u32);
    println!("Średni czas ruchu czerwonego gracza: {:?}", total_red_turn_time / total_red_turns as u32);
    println!("Procent zwycięstw żółtego gracza: {:.2}%", (total_yellow_wins as f32 * 100.0) / (total_games as f32));
    println!("Procent zwycięstw czerwonego gracza: {:.2}%", (total_red_wins as f32 * 100.0) / (total_games as f32));
    println!("Procent remisów: {:.2}%", (total_draws as f32 * 100.0) / (total_games as f32));
    println!("Średnia liczba sprawdzonych stanów gry dla żółtego gracza: {:.2}", (total_yellow_visited_states as f32) / (total_yellow_turns as f32));
    println!("Średnia liczba sprawdzonych stanów gry dla czerwonego gracza: {:.2}", (total_red_visited_states as f32) / (total_red_turns as f32));
    println!("Średnia liczba ruchów żółtego gracza: {:.2}", (total_yellow_turns as f32) / (total_games as f32));
    println!("Średnia liczba ruchów czerwonego gracza: {:.2}", (total_red_turns as f32) / (total_games as f32));
    println!();
    println!(
        "{};{:.2?};{:.2?};{:.2}%;{:.2}%;{:.2}%;{:.2};{:.2};{:.2};{:.2}",
        total_games,
        total_yellow_turn_time / total_yellow_turns as u32,
        total_red_turn_time / total_red_turns as u32,
        (total_yellow_wins as f32 * 100.0) / (total_games as f32),
        (total_red_wins as f32 * 100.0) / (total_games as f32),
        (total_draws as f32 * 100.0) / (total_games as f32),
        (total_yellow_visited_states as f32) / (total_yellow_turns as f32),
        (total_red_visited_states as f32) / (total_red_turns as f32),
        (total_yellow_turns as f32) / (total_games as f32),
        (total_red_turns as f32) / (total_games as f32)
    );
    println!();
}

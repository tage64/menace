use menace::*;
use std::collections::HashMap;
use std::io::{self, Write as _};
use std::ops::AddAssign;

const TRAIN_CYCLES: u32 = 10000000;
const TRAIN_CHUNKS: u32 = 4;
const TRAIN_CHUNK_SIZE: u32 = TRAIN_CYCLES / TRAIN_CHUNKS;

fn main() -> io::Result<()> {
    let mut machine = Machine::new();
    let mut result_chunks: Vec<HashMap<GameResult, u32>> = vec![HashMap::new()];
    for i in 1..=TRAIN_CYCLES {
        let result = machine.play_training_match();
        if i % TRAIN_CHUNK_SIZE == 0 {
            if let Some(chunk) = result_chunks.last() {
                let get_percent = |filter: &dyn Fn(GameResult) -> bool| {
                    chunk
                        .iter()
                        .filter(|&(&r, _)| filter(r))
                        .map(|(_, &c)| c as f32)
                        .sum::<f32>()
                        / TRAIN_CHUNK_SIZE as f32
                        * 100.0
                };
                let draws = get_percent(&|r| r == Draw);
                let crosses = get_percent(&|r| match r {
                    Win {
                        winner: Crosses, ..
                    } => true,
                    _ => false,
                });
                let naughts = get_percent(&|r| match r {
                    Win {
                        winner: Naughts, ..
                    } => true,
                    _ => false,
                });
                let resignations = get_percent(&|r| match r {
                    Win {
                        reason: Resignation,
                        ..
                    } => true,
                    _ => false,
                });
                println!(
                    "{}: draws: {draws:.1}, wins: crosses: {crosses:.1}, naughts: {naughts:.1}, \
                     resignations: {resignations:.1}",
                    i / TRAIN_CHUNK_SIZE
                );
            }
            result_chunks.push(HashMap::new());
        }
        result_chunks
            .last_mut()
            .unwrap()
            .entry(result)
            .or_default()
            .add_assign(1);
    }
    println!("Trained on {} positions", machine.values().len());
    println!("Starting a game against the machine:");
    let stdin = io::stdin();
    let mut stdin_buf = String::new();
    let mut pos = State::new();
    let machine_player = Crosses;
    let you = machine_player.opponent();
    let mut turn = Crosses;
    let result = loop {
        if turn == machine_player {
            println!("Move scores: {}", machine.get_move_scores(pos));
            let Some(m) = machine.select_move(pos) else {
                break Win {
                    winner: you,
                    reason: Resignation,
                };
            };
            println!("My move: {m}");
            pos.play(m, machine_player);
            println!("{pos:?}");
        } else {
            print!("Your move: ");
            io::stdout().flush()?;
            stdin_buf.clear();
            stdin.read_line(&mut stdin_buf)?;
            let m = match stdin_buf.trim().parse::<Move>() {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Error: {e}");
                    continue;
                }
            };
            if !pos.legal_moves().contains(m) {
                eprintln!("Error: The move {m} is not a legal move in this position.");
                continue;
            }
            pos.play(m, you);
        }
        if let Some(res) = pos.result(turn) {
            break res;
        }
        turn = turn.opponent();
    };
    println!("{result:?}");
    if let Win { winner, .. } = result {
        if winner == machine_player {
            println!("Haha! You lost!");
        } else {
            println!("The machine is bad, so you won!");
        }
    }
    Ok(())
}

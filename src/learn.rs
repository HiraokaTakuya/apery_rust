use crate::bitboard::*;
#[cfg(feature = "kppt")]
use crate::evaluate::kppt::*;
use crate::file_to_vec::*;
use crate::huffman_code::*;
use crate::movegen::*;
use crate::movetypes::*;
use crate::position::*;
use crate::search::*;
use crate::thread::*;
use crate::tt::*;
use crate::types::*;
use crate::usi::*;
use crate::usioption::*;
use rand::prelude::*;
use rand::Rng;
use std::io::prelude::*;

#[allow(dead_code)]
struct TeacherWriter {
    file: std::fs::File,
}

#[allow(dead_code)]
impl TeacherWriter {
    fn new<P>(path: P) -> std::io::Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let file = std::fs::File::create(path)?;
        Ok(Self { file })
    }

    fn write(&mut self, hcpes: &[HuffmanCodedPositionAndEval]) {
        self.file.write_all(as_u8_slice(hcpes)).unwrap();
    }
}

fn random_move(pos: &mut Position, rng: &mut ThreadRng) {
    match rng.gen_range(0..2) {
        0 => {
            // King move and opponent king move 1/2 probability.
            let mut mlist = MoveList::new();
            let ksq = pos.king_square(pos.side_to_move());
            for to in ATTACK_TABLE.king.attack(ksq) {
                let m = Move::new_unpromote(ksq, to, pos.piece_on(ksq));
                if pos.pseudo_legal::<NotSearchingType>(m) && pos.legal(m) {
                    mlist.push(m);
                }
            }
            if let Some(ext_move) = mlist.slice(0).choose(rng) {
                let m = ext_move.mv;
                let gives_check = pos.gives_check(m);
                pos.do_move(m, gives_check);
                match rng.gen_range(0..2) {
                    0 => {} // nop
                    1 => {
                        // opponent king move 1/2 probability.
                        let mut mlist = MoveList::new();
                        let current_size = 0;
                        mlist.generate::<LegalType>(pos, current_size);
                        if let Some(ext_move) = mlist.slice(0).choose(rng) {
                            let m = ext_move.mv;
                            let gives_check = pos.gives_check(m);
                            pos.do_move(m, gives_check);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        1 => {
            // All move.
            for _ in 0..2 {
                // Each color move.
                let mut mlist = MoveList::new();
                let current_size = 0;
                mlist.generate::<LegalType>(pos, current_size);
                match mlist.slice(0).choose(rng) {
                    Some(ext_move) => {
                        let m = ext_move.mv;
                        let gives_check = pos.gives_check(m);
                        pos.do_move(m, gives_check);
                    }
                    None => break,
                }
            }
        }
        _ => unreachable!(),
    }
}

pub fn generate_teachers(args: &[&str]) {
    if args.len() != 5 {
        eprintln!("Invalid generate_teachers command.");
        eprintln!("expected:");
        eprintln!(
            r#"generate_teachers <output_file_path> <root_positions_file_path> <search_depth> <num_threads> <num_teachers>"#
        );
        return;
    }
    let output = args[0];
    let root_positions_file_path = args[1];
    let search_depth = args[2];
    let num_threads = args[3];
    let num_teachers = args[4];
    let writer = std::sync::Arc::new(std::sync::Mutex::new(match TeacherWriter::new(output) {
        Ok(w) => w,
        Err(_) => {
            eprintln!(r#"Cannot create file "{}"."#, output);
            return;
        }
    }));
    let roots: std::sync::Arc<Vec<HuffmanCodedPosition>> = std::sync::Arc::new(match file_to_vec(root_positions_file_path) {
        Ok(v) => v,
        Err(_) => {
            eprintln!(r#"Cannot read file "{}"."#, root_positions_file_path);
            return;
        }
    });
    let search_depth = match search_depth.parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!(r#"Cannot parse "{}" as search_depth."#, search_depth);
            return;
        }
    };
    let num_threads = match num_threads.parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!(r#"Cannot parse "{}" as num_threads."#, num_threads);
            return;
        }
    };
    let num_teachers = match num_teachers.parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!(r#"Cannot parse "{}" as num_teachers."#, num_teachers);
            return;
        }
    };
    let count_teachers = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut v = vec![];
    for _ in 0..num_threads {
        let writer = writer.clone();
        let roots = roots.clone();
        let count_teachers = count_teachers.clone();
        let worker = move || {
            let mut rng = rand::thread_rng();
            let mut thread_pool = ThreadPool::new();
            let mut tt = TranspositionTable::new();
            #[cfg(feature = "kppt")]
            let mut ehash = EvalHash::new(); // todo: All threads use same ehash.
            let mut reductions = Reductions::new(1);
            thread_pool.set(
                1,
                &mut tt,
                #[cfg(feature = "kppt")]
                &mut ehash,
                &mut reductions,
            );
            let mut is_ready = false;
            let usi_options = {
                let mut u = UsiOptions::new();
                [
                    (UsiOptions::MULTI_PV, "1"),
                    (UsiOptions::THREADS, "1"),
                    (UsiOptions::USI_HASH, "1024"),
                    #[cfg(feature = "kppt")]
                    (UsiOptions::EVAL_HASH, "256"),
                    (UsiOptions::BOOK_ENABLE, "false"),
                ]
                .iter()
                .for_each(|(name, value)| {
                    setoption(
                        &["name", name, "value", value],
                        &mut u,
                        &mut thread_pool,
                        &mut tt,
                        #[cfg(feature = "kppt")]
                        &mut ehash,
                        &mut reductions,
                        &mut is_ready,
                    );
                });
                u
            };
            let limits = {
                let mut l = LimitsType::new();
                l.start_time = Some(std::time::Instant::now());
                l.depth = Some(search_depth);
                l
            };
            let ponder_mode = false;
            let hide_all_output = true;
            const MAX_MOVES: i32 = 400;
            let mut hcpes: Vec<HuffmanCodedPositionAndEval> = vec![];
            'game_start: while count_teachers.load(std::sync::atomic::Ordering::Relaxed) < num_teachers {
                hcpes.clear();
                let hcp = &roots[rng.gen_range(0..roots.len())];
                let mut pos = Position::new_from_huffman_coded_position(hcp).unwrap();
                random_move(&mut pos, &mut rng);
                let mut position_key_appearances = std::collections::HashMap::new();
                let start_ply = pos.ply();
                let max_moves_ply = start_ply + MAX_MOVES;
                let game_result;
                let end_ply;
                loop {
                    if pos.ply() >= max_moves_ply {
                        game_result = GameResult::Draw;
                        end_ply = pos.ply() as i16;
                        hcpes.iter_mut().for_each(|hcpe| {
                            hcpe.end_ply = end_ply;
                            hcpe.game_result = game_result;
                        });
                        break;
                    }
                    let key_count = position_key_appearances.entry(pos.key()).or_insert(0);
                    *key_count += 1;
                    if *key_count >= 2 {
                        // assume sennnichite.
                        game_result = GameResult::Draw;
                        end_ply = pos.ply() as i16;
                        hcpes.iter_mut().for_each(|hcpe| {
                            hcpe.end_ply = end_ply;
                            hcpe.game_result = game_result;
                        });
                        break;
                    }
                    thread_pool.start_thinking(&pos, &mut tt, limits.clone(), &usi_options, ponder_mode, hide_all_output);
                    thread_pool.wait_for_search_finished();
                    let rm = thread_pool.last_best_root_move.lock().unwrap();
                    let rm = rm.as_ref().unwrap();
                    const RESIGN_THRESH: Value = Value(4000);
                    if rm.score.abs() <= RESIGN_THRESH {
                        hcpes.push(HuffmanCodedPositionAndEval {
                            hcp: HuffmanCodedPosition::from(&pos),
                            value: rm.score.0 as i16,
                            best_move16: u32::from(rm.pv[0].0) as u16,
                            end_ply: 0,                    // set after.
                            game_result: GameResult::Draw, // set after.
                            padding: 0,
                        });
                        count_teachers.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    } else {
                        if pos.ply() == start_ply {
                            // Not use for teacher data because this game has no usefull positions.
                            continue 'game_start;
                        }
                        game_result = if pos.side_to_move() == Color::BLACK {
                            if rm.score < Value::ZERO {
                                GameResult::WhiteWin
                            } else {
                                GameResult::BlackWin
                            }
                        } else if rm.score < Value::ZERO {
                            GameResult::BlackWin
                        } else {
                            GameResult::WhiteWin
                        };
                        end_ply = pos.ply() as i16;
                        hcpes.iter_mut().for_each(|hcpe| {
                            hcpe.end_ply = end_ply;
                            hcpe.game_result = game_result;
                        });
                        break;
                    }
                    let gives_check = pos.gives_check(rm.pv[0]);
                    pos.do_move(rm.pv[0], gives_check);
                }
                writer.lock().unwrap().write(&hcpes);
            }
        };
        v.push(
            std::thread::Builder::new()
                .stack_size(crate::stack_size::STACK_SIZE)
                .spawn(worker)
                .unwrap(),
        );
    }
    for th in v {
        th.join().unwrap();
    }
}

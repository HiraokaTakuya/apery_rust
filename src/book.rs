use crate::movetypes::*;
use crate::position::*;
use crate::types::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Info {
    value: Value,
    probability: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Book(std::collections::BTreeMap<String, std::collections::BTreeMap<UsiMove, Info>>);

impl Book {
    #[allow(dead_code)]
    fn new() -> Book {
        Book(std::collections::BTreeMap::new())
    }
    #[allow(dead_code)]
    fn insert(&mut self, sfen: String, mv: Move, info: Info) {
        let set = self
            .0
            .entry(sfen)
            .or_insert_with(|| std::collections::BTreeMap::new());
        set.insert(mv.to_usi(), info);
    }
    #[allow(dead_code)]
    fn probe(&self, pos: &Position, rng: &mut ThreadRng) -> Option<Move> {
        let sfen = pos.to_sfen();
        match self.0.get(&sfen) {
            Some(candidates) => {
                let sum_probability = candidates
                    .values()
                    .fold(0, |sum, info| sum + info.probability);
                if sum_probability == 0 {
                    return None;
                }
                let num = rng.gen_range(0, sum_probability);
                let mut count = 0;
                for (usi_move, info) in candidates.iter() {
                    count += info.probability;
                    if count >= num {
                        let mv = Move::new_from_usi(usi_move, pos);
                        debug_assert!(mv.is_some());
                        return mv;
                    }
                }
                unreachable!();
            }
            None => {
                return None;
            }
        }
    }
}

#[test]
fn test_book() {
    const STACK_SIZE: usize = 128 * 1024 * 1024;
    std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(|| {
            let sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
            let mut pos = Position::new_from_sfen(sfen).unwrap();
            let mut b = Book::new();
            b.insert(
                sfen.to_string(),
                Move::new_from_usi_str("2g2f", &pos).unwrap(),
                Info {
                    value: Value(36),
                    probability: 6,
                },
            );
            b.insert(
                sfen.to_string(),
                Move::new_from_usi_str("7g7f", &pos).unwrap(),
                Info {
                    value: Value(99),
                    probability: 99,
                },
            );
            b.insert(
                sfen.to_string(),
                Move::new_from_usi_str("6i7h", &pos).unwrap(),
                Info {
                    value: Value(20),
                    probability: 1,
                },
            );
            // overwrite
            b.insert(
                sfen.to_string(),
                Move::new_from_usi_str("7g7f", &pos).unwrap(),
                Info {
                    value: Value(36),
                    probability: 4,
                },
            );
            let m = Move::new_from_usi_str("2g2f", &pos).unwrap();
            let gives_check = pos.gives_check(m);
            pos.do_move(m, gives_check);
            b.insert(
                pos.to_sfen(),
                Move::new_from_usi_str("3c3d", &pos).unwrap(),
                Info {
                    value: Value(-99),
                    probability: 1,
                },
            );
            assert_eq!(
                r#"{"lnsgkgsnl/1r5b1/ppppppppp/9/9/7P1/PPPPPPP1P/1B5R1/LNSGKGSNL w - 2":{"3c3d":{"value":-99,"probability":1}},"lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1":{"2g2f":{"value":36,"probability":6},"6i7h":{"value":20,"probability":1},"7g7f":{"value":36,"probability":4}}}"#,
                serde_json::to_string(&b).unwrap(),
            );
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn test_book_probe() {
    const STACK_SIZE: usize = 128 * 1024 * 1024;
    std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(|| {
            let sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
            let pos = Position::new_from_sfen(sfen).unwrap();
            let mut b = Book::new();
            b.insert(
                sfen.to_string(),
                Move::new_from_usi_str("2g2f", &pos).unwrap(),
                Info {
                    value: Value(36),
                    probability: 70,
                },
            );
            b.insert(
                sfen.to_string(),
                Move::new_from_usi_str("7g7f", &pos).unwrap(),
                Info {
                    value: Value(99),
                    probability: 30,
                },
            );

            let mut rng = rand::thread_rng();
            for i in 0.. {
                assert!(i < 10000);
                match b.probe(&pos, &mut rng) {
                    Some(mv) => {
                        if mv.to_usi_string() == "2g2f" {
                            break;
                        }
                    }
                    None => unreachable!(),
                }
                let mv = b.probe(&pos, &mut rng).unwrap();
                if mv.to_usi_string() == "2g2f" {
                    break;
                }
            }
            for i in 0.. {
                assert!(i < 10000);
                match b.probe(&pos, &mut rng) {
                    Some(mv) => {
                        if mv.to_usi_string() == "2g2f" {
                            break;
                        }
                    }
                    None => unreachable!(),
                }
                let mv = b.probe(&pos, &mut rng).unwrap();
                if mv.to_usi_string() == "7g7f" {
                    break;
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

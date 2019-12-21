use crate::movetypes::*;
use crate::position::*;
use crate::types::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Info {
    value: Value,
    win: u64,
    lose: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Book(std::collections::BTreeMap<String, std::collections::BTreeMap<UsiMove, Info>>);

impl Book {
    #[allow(dead_code)]
    pub fn new() -> Book {
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
    pub fn probe(&self, pos: &Position, rng: &mut ThreadRng) -> Option<Move> {
        let sfen = pos.to_sfen();
        match self.0.get(&sfen) {
            Some(candidates) => {
                let move_and_weights = candidates
                    .iter()
                    .map(|(usi_move, info)| {
                        let win_rate = info.win as f64 / (info.win + info.lose) as f64;
                        let weight = win_rate * win_rate;
                        (usi_move, weight)
                    })
                    .collect::<Vec<_>>();
                let dist =
                    rand::distributions::WeightedIndex::new(move_and_weights.iter().map(|x| x.1))
                        .unwrap();
                let usi_move = move_and_weights[dist.sample(rng)].0;
                let m = Move::new_from_usi(usi_move, pos);
                return m;
            }
            None => {
                return None;
            }
        }
    }
    pub fn from_file<P>(path: P) -> Result<Book, Box<dyn std::error::Error>>
    where
        P: AsRef<std::path::Path>,
    {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let book = serde_json::from_reader(reader)?;
        Ok(book)
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
                    win: 6,
                    lose: 4,
                },
            );
            b.insert(
                sfen.to_string(),
                Move::new_from_usi_str("7g7f", &pos).unwrap(),
                Info {
                    value: Value(99),
                    win: 99,
                    lose: 3,
                },
            );
            b.insert(
                sfen.to_string(),
                Move::new_from_usi_str("6i7h", &pos).unwrap(),
                Info {
                    value: Value(20),
                    win: 1,
                    lose: 10,
                },
            );
            // overwrite
            b.insert(
                sfen.to_string(),
                Move::new_from_usi_str("7g7f", &pos).unwrap(),
                Info {
                    value: Value(36),
                    win: 3,
                    lose: 9,
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
                    win: 1,
                    lose: 2,
                },
            );
            assert_eq!(
                r#"{"lnsgkgsnl/1r5b1/ppppppppp/9/9/7P1/PPPPPPP1P/1B5R1/LNSGKGSNL w - 2":{"3c3d":{"value":-99,"win":1,"lose":2}},"lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1":{"2g2f":{"value":36,"win":6,"lose":4},"6i7h":{"value":20,"win":1,"lose":10},"7g7f":{"value":36,"win":3,"lose":9}}}"#,
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
                    win: 70,
                    lose: 30,
                },
            );
            b.insert(
                sfen.to_string(),
                Move::new_from_usi_str("7g7f", &pos).unwrap(),
                Info {
                    value: Value(99),
                    win: 30,
                    lose: 10,
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

#[test]
fn test_book_from_file() {
    const STACK_SIZE: usize = 128 * 1024 * 1024;
    std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(|| {
            let path = std::path::Path::new("test/book.json");
            let book = Book::from_file(path).unwrap();
            assert_eq!(
                r#"{"lnsgkgsnl/1r5b1/ppppppppp/9/9/7P1/PPPPPPP1P/1B5R1/LNSGKGSNL w - 2":{"3c3d":{"value":-99,"win":1,"lose":2}},"lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1":{"2g2f":{"value":36,"win":6,"lose":4},"6i7h":{"value":20,"win":1,"lose":10},"7g7f":{"value":36,"win":3,"lose":9}}}"#,
                serde_json::to_string(&book).unwrap(),
            );
        })
        .unwrap()
        .join()
        .unwrap();
}

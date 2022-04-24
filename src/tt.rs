use crate::movetypes::*;
use crate::position::*;
use crate::thread::*;
use crate::types::*;
use rayon::prelude::*;

#[derive(Clone, Copy)]
pub struct TtEntry {
    key16: u16,
    mv16: u16,
    value16: i16,
    eval16: i16,
    genbound8: u8,
    depth8: u8,
}

impl TtEntry {
    fn new() -> Self {
        Self {
            key16: 0,
            mv16: 0,
            value16: 0,
            eval16: 0,
            genbound8: 0,
            depth8: 0,
        }
    }
    pub fn mv(&self, pos: &Position) -> Option<Move> {
        // This can be illegal move.
        let m = Move(unsafe { std::num::NonZeroU32::new_unchecked(u32::from(self.mv16)) });
        let m = if !Some(m).is_normal_move() || m.is_drop() {
            m
        } else {
            Move(unsafe {
                std::num::NonZeroU32::new_unchecked(m.0.get() | ((pos.piece_on(m.from()).0 as u32) << Move::MOVED_PIECE_SHIFT))
            })
        };
        if pos.pseudo_legal::<SearchingType>(m) {
            Some(m)
        } else {
            None
        }
    }
    pub fn value(&self) -> Value {
        Value(i32::from(self.value16))
    }
    pub fn eval(&self) -> Value {
        Value(i32::from(self.eval16))
    }
    pub fn depth(&self) -> Depth {
        Depth(i32::from(self.depth8)) + Depth::OFFSET
    }
    pub fn is_pv(&self) -> bool {
        (self.genbound8 & 0x4) != 0
    }
    pub fn bound(&self) -> Bound {
        Bound(i32::from(self.genbound8) & 0x3)
    }
    #[allow(dead_code)]
    pub fn generation(&self) -> u8 {
        self.genbound8 & GENERATION_MASK as u8
    }
    pub fn save(
        &mut self,
        key: Key,
        value: Value,
        pv: bool,
        bound: Bound,
        depth: Depth,
        mv: Option<Move>,
        eval: Value,
        generation: u8,
    ) {
        let key = key.excluded_turn().0 as u16;
        if let Some(mv) = mv {
            self.mv16 = u32::from(mv.0) as u16;
        } else if key != self.key16 {
            self.mv16 = 0;
        }

        if bound == Bound::EXACT || key != self.key16 || depth.0 - Depth::OFFSET.0 > i32::from(self.depth8) - 4 {
            debug_assert!(depth > Depth::OFFSET);
            debug_assert!(depth.0 < 256 + Depth::OFFSET.0);
            self.key16 = key;
            self.value16 = value.0 as i16;
            self.eval16 = eval.0 as i16;
            self.genbound8 = (i32::from(generation) | (i32::from(pv) << 2) | bound.0) as u8;
            self.depth8 = (depth.0 - Depth::OFFSET.0) as u8;
        }
    }
}

const CLUSTER_SIZE: usize = 3;

const GENERATION_BITS: u32 = 3;
const GENERATION_DELTA: u8 = 1 << GENERATION_BITS;
const GENERATION_CYCLE: i32 = 255 + (1 << GENERATION_BITS);
const GENERATION_MASK: i32 = (0xff << GENERATION_BITS) & 0xff;

#[repr(align(32))]
#[derive(Clone, Copy)]
struct TtCluster {
    entry: [TtEntry; CLUSTER_SIZE],
    _padding: [u8; 2],
}

impl TtCluster {
    fn new() -> Self {
        Self {
            entry: [TtEntry::new(); CLUSTER_SIZE],
            _padding: [0; 2],
        }
    }
}

pub struct TranspositionTable {
    table: Vec<TtCluster>,
    cluster_count: usize,
    generation8: u8,
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        TranspositionTable {
            table: vec![],
            cluster_count: 0,
            generation8: 0,
        }
    }
    pub fn resize(&mut self, mega_byte_size: usize, thread_pool: &mut ThreadPool) {
        thread_pool.wait_for_search_finished();
        self.cluster_count = mega_byte_size * 1024 * 1024 / std::mem::size_of::<TtCluster>();
        debug_assert!(self.cluster_count & 1 == 0);
        // self.table can be very large and takes much time to clear, so parallelize self.clear().
        self.table.clear();
        self.table.shrink_to_fit();
        self.table = (0..self.cluster_count).into_par_iter().map(|_| TtCluster::new()).collect();
    }
    // parallel zero clearing.
    pub fn clear(&mut self) {
        self.table.par_iter_mut().for_each(|x| {
            *x = unsafe { std::mem::zeroed() };
        });
    }
    pub fn new_search(&mut self) {
        self.generation8 = self.generation8.wrapping_add(GENERATION_DELTA);
    }
    fn cluster_index(&self, key: Key) -> usize {
        fn mul_hi64(l: u64, r: u64) -> u64 {
            ((u128::from(l) * u128::from(r)) >> 64) as u64
        }
        let index = mul_hi64(key.excluded_turn().0, self.cluster_count as u64); // [0, self.cluster_count / 2 - 1]
        ((index << 1) | key.turn_bit()) as usize // [0, self.cluster_count - 1]
    }
    fn get_mut_cluster(&mut self, index: usize) -> &mut TtCluster {
        debug_assert!(index < self.table.len());
        unsafe { self.table.get_unchecked_mut(index) }
    }
    pub fn probe(&mut self, key: Key) -> (&mut TtEntry, bool) {
        let generation8 = self.generation8;
        let key16 = key.excluded_turn().0 as u16;
        let cluster = self.get_mut_cluster(self.cluster_index(key));
        for i in 0..cluster.entry.len() {
            if cluster.entry[i].key16 == key16 || i32::from(cluster.entry[i].depth8) == 0 {
                cluster.entry[i].genbound8 = generation8 | (cluster.entry[i].genbound8 & (GENERATION_DELTA - 1)); // refresh
                let found = i32::from(cluster.entry[i].depth8) != 0;
                return (&mut cluster.entry[i], found);
            }
        }
        let replace = cluster
            .entry
            .iter_mut()
            .min_by(|x, y| {
                let left = i32::from(x.depth8)
                    - ((GENERATION_CYCLE + i32::from(generation8) - i32::from(x.genbound8)) & GENERATION_MASK);
                let right = i32::from(y.depth8)
                    - ((GENERATION_CYCLE + i32::from(generation8) - i32::from(y.genbound8)) & GENERATION_MASK);
                left.cmp(&right)
            })
            .unwrap();
        let found = false;
        (replace, found)
    }
    pub fn generation(&self) -> u8 {
        self.generation8
    }
}

#[test]
fn test_size() {
    assert_eq!(std::mem::size_of::<TtEntry>(), 10);
    assert_eq!(std::mem::size_of::<TtCluster>(), 32);
    assert_eq!(std::mem::size_of::<[TtCluster; 4]>(), 128);
}

#[test]
fn test_cluster_index() {
    #[cfg(feature = "kppt")]
    use crate::evaluate::kppt::*;
    use crate::search::*;
    std::thread::Builder::new()
        .stack_size(crate::stack_size::STACK_SIZE)
        .spawn(|| {
            let mut thread_pool = ThreadPool::new();
            let mut tt = TranspositionTable::new();
            #[cfg(feature = "kppt")]
            let mut ehash = EvalHash::new();
            let mut reductions = Reductions::new();
            thread_pool.set(
                1,
                &mut tt,
                #[cfg(feature = "kppt")]
                &mut ehash,
                &mut reductions,
            );
            tt.resize(1, &mut thread_pool);
            #[cfg(feature = "kppt")]
            ehash.resize(1, &mut thread_pool);

            // If key is all 1 bits, index is max.
            let key = Key(0xffff_ffff_ffff_ffff);
            let index = tt.cluster_index(key);
            assert_eq!(index, tt.cluster_count - 1);
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn test_probe() {
    #[cfg(feature = "kppt")]
    use crate::evaluate::kppt::*;
    use crate::search::*;
    std::thread::Builder::new()
        .stack_size(crate::stack_size::STACK_SIZE)
        .spawn(|| {
            let mut thread_pool = ThreadPool::new();
            let mut tt = TranspositionTable::new();
            #[cfg(feature = "kppt")]
            let mut ehash = EvalHash::new();
            let mut reductions = Reductions::new();
            thread_pool.set(
                1,
                &mut tt,
                #[cfg(feature = "kppt")]
                &mut ehash,
                &mut reductions,
            );
            tt.resize(1, &mut thread_pool);
            #[cfg(feature = "kppt")]
            ehash.resize(1, &mut thread_pool);
            let pv = false;
            let gen8 = tt.generation8;

            use rand::prelude::*;
            let mut rand: StdRng = SeedableRng::seed_from_u64(123);
            let key = Key(0x0123_4567_89ab_cdef);
            let cluster_index = tt.cluster_index(key);

            let (tte, found) = tt.probe(key);
            assert!(!found);
            let (d2_val, d2) = (Value(20), Depth(2));
            tte.save(key, d2_val, pv, Bound::EXACT, d2, None, Value(0), gen8); // cluster: [(d2, gen_old), 0, 0]
            let (_, found) = tt.probe(key);
            assert!(found);

            fn gen_same_cluster_index_key(rng: &mut StdRng, cluster_index: usize, tt: &TranspositionTable) -> Key {
                loop {
                    let key = Key(rng.gen());
                    let c_index = tt.cluster_index(key);
                    if c_index == cluster_index {
                        return key;
                    }
                }
            }
            let key = gen_same_cluster_index_key(&mut rand, cluster_index, &tt);
            let (tte, found) = tt.probe(key);
            assert!(!found);
            let (d1_val, d1) = (Value(10), Depth(1));
            tte.save(key, d1_val, pv, Bound::EXACT, d1, None, Value(0), gen8); // cluster: [(d2, gen_old), (d1, gen_old), 0]
            let (_, found) = tt.probe(key);
            assert!(found);

            let key = gen_same_cluster_index_key(&mut rand, cluster_index, &tt);
            let (tte, found) = tt.probe(key);
            assert!(!found);
            let (d9_val, d9) = (Value(90), Depth(9));
            tte.save(key, d9_val, pv, Bound::EXACT, d9, None, Value(0), gen8); // cluster: [(d2, gen_old), (d1, gen_old), (d9, gen_old)]
            let (_, found) = tt.probe(key);
            assert!(found);

            tt.new_search();
            let gen8 = tt.generation8;

            let key = gen_same_cluster_index_key(&mut rand, cluster_index, &tt);
            let (tte, found) = tt.probe(key);
            assert!(!found);
            assert_eq!(tte.value(), d1_val); // the entry is most shallow depth
            let (d1_val, d1) = (Value(10), Depth(1));
            tte.save(key, d1_val, pv, Bound::EXACT, d1, None, Value(0), gen8); // cluster: [(d2, gen_old), (d1, gen_new), (d9, gen_old)]
            let (_, found) = tt.probe(key);
            assert!(found);

            let key = gen_same_cluster_index_key(&mut rand, cluster_index, &tt);
            let (tte, found) = tt.probe(key);
            assert!(!found);
            assert_eq!(tte.value(), d2_val); // old and shallow entry.
            let (d3_val, d3) = (Value(30), Depth(3));
            tte.save(key, d3_val, pv, Bound::EXACT, d3, None, Value(0), gen8); // cluster: [d3, gen_new), (d1, gen_new), (d9, gen_old)]
            let (_, found) = tt.probe(key);
            assert!(found);

            let key = gen_same_cluster_index_key(&mut rand, cluster_index, &tt);
            let (tte, found) = tt.probe(key);
            assert!(!found);
            assert_eq!(tte.value(), d1_val); // d9 entry has very deep depth. d9 isn't chosen.
            let (d2_val, d2) = (Value(20), Depth(2));
            tte.save(key, d2_val, pv, Bound::EXACT, d2, None, Value(0), gen8); // cluster: [d3, gen_new), (d2, gen_new), (d9, gen_old)]
            let (_, found) = tt.probe(key);
            assert!(found);
        })
        .unwrap()
        .join()
        .unwrap();
}

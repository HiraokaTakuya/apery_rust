use crate::bitboard::*;
use crate::movetypes::*;
use crate::position::*;
use crate::types::*;

pub trait AllowMovesTrait {
    const ALLOW_CAPTURES: bool;
    const ALLOW_QUIETS: bool;
    const EVASIONS: bool;
    const LEGALS: bool;
    const ALL: bool;
    const ALLOW_PSEUDO_LEGAL: bool;
}

pub struct CaptureOrPawnPromotionsType;
pub struct QuietsWithoutPawnPromotionsType;
pub struct EvasionsType;
pub struct EvasionsAllType;
pub struct NonEvasionsType;
pub struct NonEvasionsAllType;
pub struct LegalType;
pub struct LegalAllType;

impl AllowMovesTrait for CaptureOrPawnPromotionsType {
    const ALLOW_CAPTURES: bool = true;
    const ALLOW_QUIETS: bool = false;
    const EVASIONS: bool = false;
    const LEGALS: bool = false;
    const ALL: bool = false;
    const ALLOW_PSEUDO_LEGAL: bool = true;
}
impl AllowMovesTrait for QuietsWithoutPawnPromotionsType {
    const ALLOW_CAPTURES: bool = false;
    const ALLOW_QUIETS: bool = true;
    const EVASIONS: bool = false;
    const LEGALS: bool = false;
    const ALL: bool = false;
    const ALLOW_PSEUDO_LEGAL: bool = true;
}
impl AllowMovesTrait for EvasionsType {
    const ALLOW_CAPTURES: bool = true;
    const ALLOW_QUIETS: bool = true;
    const EVASIONS: bool = true;
    const LEGALS: bool = false;
    const ALL: bool = false;
    const ALLOW_PSEUDO_LEGAL: bool = true;
}
impl AllowMovesTrait for EvasionsAllType {
    const ALLOW_CAPTURES: bool = true;
    const ALLOW_QUIETS: bool = true;
    const EVASIONS: bool = true;
    const LEGALS: bool = false;
    const ALL: bool = true;
    const ALLOW_PSEUDO_LEGAL: bool = true;
}
impl AllowMovesTrait for NonEvasionsType {
    const ALLOW_CAPTURES: bool = true;
    const ALLOW_QUIETS: bool = true;
    const EVASIONS: bool = false;
    const LEGALS: bool = false;
    const ALL: bool = false;
    const ALLOW_PSEUDO_LEGAL: bool = true;
}
impl AllowMovesTrait for NonEvasionsAllType {
    const ALLOW_CAPTURES: bool = true;
    const ALLOW_QUIETS: bool = true;
    const EVASIONS: bool = false;
    const LEGALS: bool = false;
    const ALL: bool = true;
    const ALLOW_PSEUDO_LEGAL: bool = true;
}
impl AllowMovesTrait for LegalType {
    const ALLOW_CAPTURES: bool = true;
    const ALLOW_QUIETS: bool = true;
    const EVASIONS: bool = false;
    const LEGALS: bool = true;
    const ALL: bool = false;
    const ALLOW_PSEUDO_LEGAL: bool = false;
}
impl AllowMovesTrait for LegalAllType {
    const ALLOW_CAPTURES: bool = true;
    const ALLOW_QUIETS: bool = true;
    const EVASIONS: bool = false;
    const LEGALS: bool = true;
    const ALL: bool = true;
    const ALLOW_PSEUDO_LEGAL: bool = false;
}

pub struct MoveList {
    pub ext_moves: [std::mem::MaybeUninit<ExtMove>; ExtMove::MAX_LEGAL_MOVES],
    pub size: usize,
}

impl MoveList {
    pub fn new() -> Self {
        unsafe {
            Self {
                ext_moves: std::mem::MaybeUninit::uninit().assume_init(),
                size: 0,
            }
        }
    }
    pub fn slice(&self, begin: usize) -> &[ExtMove] {
        unsafe {
            std::slice::from_raw_parts(
                self.ext_moves.get_unchecked(begin).as_ptr() as *const ExtMove,
                self.size - begin,
            )
        }
    }
    pub fn slice_mut(&mut self, begin: usize) -> &mut [ExtMove] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.ext_moves.get_unchecked_mut(begin).as_ptr() as *mut ExtMove,
                self.size - begin,
            )
        }
    }
    #[allow(dead_code)]
    fn contains(&self, m: Move) -> bool {
        self.slice(0).iter().any(|x| x.mv == m)
    }
    #[inline]
    pub fn push(&mut self, m: Move) {
        debug_assert!(self.size < self.ext_moves.len());
        unsafe {
            (*self.ext_moves.get_unchecked_mut(self.size).as_mut_ptr()).mv = m;
        }
        self.size += 1;
    }
    fn generate_all<AMT: AllowMovesTrait>(&mut self, pos: &Position, current_size: usize) {
        self.size = current_size;
        let us = pos.side_to_move();
        let target = if AMT::ALLOW_CAPTURES && AMT::ALLOW_QUIETS {
            !pos.pieces_c(us)
        } else if AMT::ALLOW_CAPTURES {
            pos.pieces_c(us.inverse())
        } else {
            debug_assert!(AMT::ALLOW_QUIETS);
            pos.empty_bb()
        };
        let target_pawn = if AMT::ALLOW_CAPTURES && AMT::ALLOW_QUIETS {
            !pos.pieces_c(us)
        } else if AMT::ALLOW_CAPTURES {
            pos.pieces_c(us.inverse()) | (pos.empty_bb() & Bitboard::opponent_field_mask(us))
        } else {
            debug_assert!(AMT::ALLOW_QUIETS);
            pos.empty_bb() & !Bitboard::opponent_field_mask(us)
        };
        self.generate_for_piece::<PawnType, AMT>(pos, &target_pawn);
        self.generate_for_piece::<LanceType, AMT>(pos, &target);
        self.generate_for_piece::<KnightType, AMT>(pos, &target);
        self.generate_for_piece::<SilverType, AMT>(pos, &target);
        self.generate_for_piece::<BishopType, AMT>(pos, &target);
        self.generate_for_piece::<RookType, AMT>(pos, &target);
        self.generate_for_piece::<GoldType, AMT>(pos, &target);
        self.generate_for_piece::<KingType, AMT>(pos, &target);
        self.generate_for_piece::<HorseType, AMT>(pos, &target);
        self.generate_for_piece::<DragonType, AMT>(pos, &target);
        if AMT::ALLOW_QUIETS {
            let target = pos.empty_bb();
            self.generate_drop::<AMT>(pos, &target);
        }
    }
    pub fn generate_evasions<ISALL: Bool>(&mut self, pos: &Position, current_size: usize) {
        self.size = current_size;
        let us = pos.side_to_move();
        let ksq_of_evasion = pos.king_square(us);
        let checkers = pos.checkers();
        let mut copy_checkers = checkers;
        let mut checker_sq;
        let mut not_target = Bitboard::ZERO;
        let mut checkers_num: i8 = 0;
        // Rust's do-while
        while {
            checker_sq = copy_checkers.pop_lsb_unchecked();
            not_target |=
                pos.effect_bb_of_checker_where_king_cannot_escape(checker_sq, pos.piece_on(checker_sq), &pos.occupied_bb());
            checkers_num += 1;
            copy_checkers.to_bool() // loop condition
        } {}
        let to_bb = ATTACK_TABLE.king.attack(ksq_of_evasion) & !pos.pieces_c(us) & !not_target;
        for to in to_bb {
            self.push(Move::new_unpromote(ksq_of_evasion, to, pos.piece_on(ksq_of_evasion)));
        }

        if 1 < checkers_num {
            // double check. king only can move.
            return;
        }

        let target_drop = Bitboard::between_mask(checker_sq, ksq_of_evasion);
        let target_move = target_drop | Bitboard::square_mask(checker_sq);

        if ISALL::BOOL {
            self.generate_for_piece::<PawnType, EvasionsAllType>(pos, &target_move);
            self.generate_for_piece::<LanceType, EvasionsAllType>(pos, &target_move);
            self.generate_for_piece::<KnightType, EvasionsAllType>(pos, &target_move);
            self.generate_for_piece::<SilverType, EvasionsAllType>(pos, &target_move);
            self.generate_for_piece::<BishopType, EvasionsAllType>(pos, &target_move);
            self.generate_for_piece::<RookType, EvasionsAllType>(pos, &target_move);
            self.generate_for_piece::<GoldType, EvasionsAllType>(pos, &target_move);
            self.generate_for_piece::<HorseType, EvasionsAllType>(pos, &target_move);
            self.generate_for_piece::<DragonType, EvasionsAllType>(pos, &target_move);

            self.generate_drop::<EvasionsAllType>(pos, &target_drop);
        } else {
            self.generate_for_piece::<PawnType, EvasionsType>(pos, &target_move);
            self.generate_for_piece::<LanceType, EvasionsType>(pos, &target_move);
            self.generate_for_piece::<KnightType, EvasionsType>(pos, &target_move);
            self.generate_for_piece::<SilverType, EvasionsType>(pos, &target_move);
            self.generate_for_piece::<BishopType, EvasionsType>(pos, &target_move);
            self.generate_for_piece::<RookType, EvasionsType>(pos, &target_move);
            self.generate_for_piece::<GoldType, EvasionsType>(pos, &target_move);
            self.generate_for_piece::<HorseType, EvasionsType>(pos, &target_move);
            self.generate_for_piece::<DragonType, EvasionsType>(pos, &target_move);

            self.generate_drop::<EvasionsType>(pos, &target_drop);
        }
    }
    fn generate_drop_for_possessions(&mut self, possessions: &[Piece], to_bb: Bitboard) {
        for to in to_bb {
            for &pc in possessions {
                self.push(Move::new_drop(pc, to));
            }
        }
    }
    fn generate_drop<AMT: AllowMovesTrait>(&mut self, pos: &Position, target: &Bitboard) {
        let us = pos.side_to_move();
        debug_assert_eq!(
            {
                let target;
                if AMT::EVASIONS {
                    let checkers = pos.checkers();
                    match checkers.count_ones() {
                        1 => {}
                        2 => return,
                        _ => unreachable!(),
                    }
                    let ksq = pos.king_square(us);
                    target = Bitboard::between_mask(ksq, checkers.lsb_unchecked());
                } else if AMT::ALLOW_QUIETS {
                    target = pos.empty_bb();
                } else {
                    unreachable!();
                }
                target
            },
            *target
        );
        let hand = pos.hand(us);
        if hand.exist(PieceType::PAWN) {
            // avoid two pawns.
            let rank = Rank::new_from_color_and_rank_as_black(us, RankAsBlack::RANK1);
            let mut to_bb = *target & !Bitboard::rank_mask(rank);
            let pawns_bb = pos.pieces_cp(us, PieceType::PAWN);
            for pawn_sq in pawns_bb {
                let pawn_file = File::new(pawn_sq);
                to_bb &= !Bitboard::file_mask(pawn_file);
            }

            // avoid drop pawn mate.
            let them = us.inverse();
            let ksq = pos.king_square(them);
            let drop_pawn_check_bb = ATTACK_TABLE.pawn.attack(them, ksq);
            if (drop_pawn_check_bb & to_bb).to_bool() {
                debug_assert_eq!(drop_pawn_check_bb.count_ones(), 1);
                let to = drop_pawn_check_bb.lsb_unchecked();
                if pos.is_drop_pawn_mate(us, to) {
                    debug_assert!(to_bb.is_set(to));
                    to_bb ^= Bitboard::square_mask(to);
                }
            }

            // drop pawns
            let piece_pawn = Piece::new(us, PieceType::PAWN);
            for to in to_bb {
                self.push(Move::new_drop(piece_pawn, to));
            }
        }
        if hand.except_pawn_exist() {
            let mut possessions = arrayvec::ArrayVec::<Piece, 6>::new();
            let sgbr_num;
            let sgbrl_num;
            {
                let f = |c: Color, pt: PieceType, possessions: &mut arrayvec::ArrayVec<Piece, 6>| {
                    if hand.exist(pt) {
                        unsafe {
                            possessions.push_unchecked(Piece::new(c, pt));
                        }
                    }
                };
                f(us, PieceType::ROOK, &mut possessions);
                f(us, PieceType::BISHOP, &mut possessions);
                f(us, PieceType::GOLD, &mut possessions);
                f(us, PieceType::SILVER, &mut possessions);
                sgbr_num = possessions.len();
                f(us, PieceType::LANCE, &mut possessions);
                sgbrl_num = possessions.len();
                f(us, PieceType::KNIGHT, &mut possessions);
            }
            let (to_bb_r1, to_bb_r2, to_bb) = {
                let r1 = Rank::new_from_color_and_rank_as_black(us, RankAsBlack::RANK1);
                let r2 = Rank::new_from_color_and_rank_as_black(us, RankAsBlack::RANK2);
                let mask1 = Bitboard::rank_mask(r1);
                let mask2 = Bitboard::rank_mask(r2);
                (*target & mask1, *target & mask2, *target & !(mask1 | mask2))
            };
            self.generate_drop_for_possessions(&possessions[..sgbr_num], to_bb_r1);
            self.generate_drop_for_possessions(&possessions[..sgbrl_num], to_bb_r2);
            self.generate_drop_for_possessions(&possessions[..], to_bb);
        }
    }
    fn generate_for_piece<PTT: PieceTypeTrait, AMT: AllowMovesTrait>(&mut self, pos: &Position, target: &Bitboard) {
        match PTT::PIECE_TYPE {
            PieceType::PAWN => self.generate_for_pawn::<AMT>(pos, target),
            PieceType::LANCE => self.generate_for_lance::<AMT>(pos, target),
            PieceType::KNIGHT => self.generate_for_knight::<AMT>(pos, target),
            PieceType::SILVER => self.generate_for_silver::<AMT>(pos, target),
            PieceType::BISHOP => self.generate_for_bishop_or_rook::<AMT>(PieceType::BISHOP, pos, target),
            PieceType::ROOK => self.generate_for_bishop_or_rook::<AMT>(PieceType::ROOK, pos, target),
            PieceType::KING => self.generate_for_king::<AMT>(pos, target),
            PieceType::GOLD => self.generate_for_gold::<AMT>(pos, target),
            PieceType::PRO_PAWN => unreachable!(),
            PieceType::PRO_LANCE => unreachable!(),
            PieceType::PRO_KNIGHT => unreachable!(),
            PieceType::PRO_SILVER => unreachable!(),
            PieceType::HORSE => self.generate_for_horse_or_dragon::<AMT>(PieceType::HORSE, pos, target),
            PieceType::DRAGON => self.generate_for_horse_or_dragon::<AMT>(PieceType::DRAGON, pos, target),
            _ => unreachable!(),
        }
    }
    fn generate_for_pawn<AMT: AllowMovesTrait>(&mut self, pos: &Position, target: &Bitboard) {
        let us = pos.side_to_move();
        let from_bb = pos.pieces_cp(us, PieceType::PAWN);
        let to_bb = if us == Color::BLACK {
            const_assert_eq!(Square::DELTA_N.0, -1);
            from_bb >> 1
        } else {
            const_assert_eq!(Square::DELTA_S.0, 1);
            from_bb << 1
        } & *target;

        fn make_to_bb<AMT: AllowMovesTrait>(pos: &Position, from_bb: Bitboard, us: Color) -> Bitboard {
            let mut to_bb = if us == Color::BLACK {
                const_assert_eq!(Square::DELTA_N.0, -1);
                from_bb >> 1
            } else {
                const_assert_eq!(Square::DELTA_S.0, 1);
                from_bb << 1
            };
            to_bb &= !pos.pieces_c(us);
            if AMT::EVASIONS {
                let checkers = pos.checkers();
                match checkers.count_ones() {
                    1 => {
                        let ksq = pos.king_square(us);
                        to_bb &= checkers | Bitboard::between_mask(ksq, checkers.lsb_unchecked());
                    }
                    2 => to_bb = Bitboard::ZERO, // Only king can move.
                    _ => unreachable!(),
                }
            }
            // allow_capture is allow capture or pawn_promotion
            if !AMT::ALLOW_CAPTURES {
                to_bb &= !(pos.pieces_c(us.inverse()) | Bitboard::opponent_field_mask(us));
            }
            if !AMT::ALLOW_QUIETS {
                to_bb &= pos.pieces_c(us.inverse()) | Bitboard::opponent_field_mask(us);
            }
            to_bb
        }
        debug_assert_eq!(make_to_bb::<AMT>(pos, from_bb, us), to_bb);

        let (delta, pc) = if us == Color::BLACK {
            (Square::DELTA_S, Piece::B_PAWN)
        } else {
            (Square::DELTA_N, Piece::W_PAWN)
        };
        for to in to_bb {
            let from = to.add_unchecked(delta);
            let rank_to = Rank::new(to);
            if rank_to.is_opponent_field(us) {
                self.push(Move::new_promote(from, to, pc));
                if AMT::ALL && rank_to != Rank::new_from_color_and_rank_as_black(us, RankAsBlack::RANK1) {
                    self.push(Move::new_unpromote(from, to, pc));
                }
            } else {
                self.push(Move::new_unpromote(from, to, pc));
            };
        }
    }
    fn generate_for_lance<AMT: AllowMovesTrait>(&mut self, pos: &Position, target: &Bitboard) {
        debug_assert!(pos.checkers().count_ones() != 2 || !target.to_bool()); // if double check (pos.checkers() == 2), target is all zero.
        let us = pos.side_to_move();
        let from_bb = pos.pieces_cp(us, PieceType::LANCE);
        for from in from_bb {
            let to_bb = ATTACK_TABLE.lance.attack(us, from, &pos.occupied_bb()) & *target;

            fn make_to_bb<AMT: AllowMovesTrait>(pos: &Position, from: Square, us: Color) -> Bitboard {
                let mut to_bb = ATTACK_TABLE.lance.attack(us, from, &pos.occupied_bb()) & !pos.pieces_c(us);
                if AMT::EVASIONS {
                    let checkers = pos.checkers();
                    match checkers.count_ones() {
                        1 => {
                            let ksq = pos.king_square(us);
                            to_bb &= checkers | Bitboard::between_mask(ksq, checkers.lsb_unchecked());
                        }
                        2 => to_bb = Bitboard::ZERO, // Only king can move.
                        _ => unreachable!(),
                    }
                }
                if !AMT::ALLOW_CAPTURES {
                    to_bb &= !pos.pieces_c(us.inverse());
                }
                if !AMT::ALLOW_QUIETS {
                    to_bb &= pos.pieces_c(us.inverse());
                }
                to_bb
            }
            debug_assert_eq!(make_to_bb::<AMT>(pos, from, us), to_bb);
            let pc = pos.piece_on(from);
            for to in to_bb {
                let rank_to = Rank::new(to);
                if rank_to.is_opponent_field(us) {
                    self.push(Move::new_promote(from, to, pc));
                    if AMT::ALL {
                        if rank_to != Rank::new_from_color_and_rank_as_black(us, RankAsBlack::RANK1) {
                            self.push(Move::new_unpromote(from, to, pc));
                        }
                    } else {
                        // avoid unpromote quiet move to rank3. because it is useless move.
                        if AMT::ALLOW_CAPTURES
                            && rank_to == Rank::new_from_color_and_rank_as_black(us, RankAsBlack::RANK3)
                            && pos.piece_on(to) != Piece::EMPTY
                        {
                            self.push(Move::new_unpromote(from, to, pc));
                        }
                    }
                } else {
                    self.push(Move::new_unpromote(from, to, pc));
                }
            }
        }
    }
    fn generate_for_knight<AMT: AllowMovesTrait>(&mut self, pos: &Position, target: &Bitboard) {
        debug_assert!(pos.checkers().count_ones() != 2 || !target.to_bool()); // if double check (pos.checkers() == 2), target is all zero.
        let us = pos.side_to_move();
        let from_bb = pos.pieces_cp(us, PieceType::KNIGHT);
        for from in from_bb {
            let to_bb = ATTACK_TABLE.knight.attack(us, from) & *target;

            fn make_to_bb<AMT: AllowMovesTrait>(pos: &Position, from: Square, us: Color) -> Bitboard {
                let mut to_bb = ATTACK_TABLE.knight.attack(us, from) & !pos.pieces_c(us);
                if AMT::EVASIONS {
                    let checkers = pos.checkers();
                    match checkers.count_ones() {
                        1 => {
                            let ksq = pos.king_square(us);
                            to_bb &= checkers | Bitboard::between_mask(ksq, checkers.lsb_unchecked());
                        }
                        2 => to_bb = Bitboard::ZERO, // Only king can move.
                        _ => unreachable!(),
                    }
                }
                if !AMT::ALLOW_CAPTURES {
                    to_bb &= !pos.pieces_c(us.inverse());
                }
                if !AMT::ALLOW_QUIETS {
                    to_bb &= pos.pieces_c(us.inverse());
                }
                to_bb
            }
            debug_assert_eq!(make_to_bb::<AMT>(pos, from, us), to_bb);
            let pc = pos.piece_on(from);
            for to in to_bb {
                let rank_to = Rank::new(to);
                if rank_to.is_opponent_field(us) {
                    self.push(Move::new_promote(from, to, pc));
                }
                if !rank_to.is_in_front_of(us, RankAsBlack::RANK3) {
                    self.push(Move::new_unpromote(from, to, pc));
                }
            }
        }
    }
    fn generate_for_silver<AMT: AllowMovesTrait>(&mut self, pos: &Position, target: &Bitboard) {
        debug_assert!(pos.checkers().count_ones() != 2 || !target.to_bool()); // if double check (pos.checkers() == 2), target is all zero.
        let us = pos.side_to_move();
        let from_bb = pos.pieces_cp(us, PieceType::SILVER);
        for from in from_bb {
            let to_bb = ATTACK_TABLE.silver.attack(us, from) & *target;

            fn make_to_bb<AMT: AllowMovesTrait>(pos: &Position, from: Square, us: Color) -> Bitboard {
                let mut to_bb = ATTACK_TABLE.silver.attack(us, from) & !pos.pieces_c(us);
                if AMT::EVASIONS {
                    let checkers = pos.checkers();
                    match checkers.count_ones() {
                        1 => {
                            let ksq = pos.king_square(us);
                            to_bb &= checkers | Bitboard::between_mask(ksq, checkers.lsb_unchecked());
                        }
                        2 => to_bb = Bitboard::ZERO, // Only king can move.
                        _ => unreachable!(),
                    }
                }
                if !AMT::ALLOW_CAPTURES {
                    to_bb &= !pos.pieces_c(us.inverse());
                }
                if !AMT::ALLOW_QUIETS {
                    to_bb &= pos.pieces_c(us.inverse());
                }
                to_bb
            }
            debug_assert_eq!(make_to_bb::<AMT>(pos, from, us), to_bb);
            let from_is_opponent_field = Rank::new(from).is_opponent_field(us);
            let pc = pos.piece_on(from);
            for to in to_bb {
                if from_is_opponent_field || Rank::new(to).is_opponent_field(us) {
                    self.push(Move::new_promote(from, to, pc));
                }
                self.push(Move::new_unpromote(from, to, pc));
            }
        }
    }
    fn generate_for_gold<AMT: AllowMovesTrait>(&mut self, pos: &Position, target: &Bitboard) {
        debug_assert!(pos.checkers().count_ones() != 2 || !target.to_bool()); // if double check (pos.checkers() == 2), target is all zero.
        let us = pos.side_to_move();
        let from_bb = pos.pieces_golds() & pos.pieces_c(us);
        for from in from_bb {
            let to_bb = ATTACK_TABLE.gold.attack(us, from) & *target;

            fn make_to_bb<AMT: AllowMovesTrait>(pos: &Position, from: Square, us: Color) -> Bitboard {
                let mut to_bb = ATTACK_TABLE.gold.attack(us, from) & !pos.pieces_c(us);
                if AMT::EVASIONS {
                    let checkers = pos.checkers();
                    match checkers.count_ones() {
                        1 => {
                            let ksq = pos.king_square(us);
                            to_bb &= checkers | Bitboard::between_mask(ksq, checkers.lsb_unchecked());
                        }
                        2 => to_bb = Bitboard::ZERO, // Only king can move.
                        _ => unreachable!(),
                    }
                }
                if !AMT::ALLOW_CAPTURES {
                    to_bb &= !pos.pieces_c(us.inverse());
                }
                if !AMT::ALLOW_QUIETS {
                    to_bb &= pos.pieces_c(us.inverse());
                }
                to_bb
            }
            debug_assert_eq!(make_to_bb::<AMT>(pos, from, us), to_bb);
            for to in to_bb {
                self.push(Move::new_unpromote(from, to, pos.piece_on(from)));
            }
        }
    }

    fn generate_for_king<AMT: AllowMovesTrait>(&mut self, pos: &Position, target: &Bitboard) {
        debug_assert!(!pos.checkers().to_bool()); // not evasion
        let us = pos.side_to_move();
        let from = pos.king_square(us);
        let to_bb = ATTACK_TABLE.king.attack(from) & *target;

        fn make_to_bb<AMT: AllowMovesTrait>(pos: &Position, from: Square, us: Color) -> Bitboard {
            let mut to_bb = ATTACK_TABLE.king.attack(from) & !pos.pieces_c(us);
            if AMT::EVASIONS {
                let checkers = pos.checkers();
                let mut copy_checkers = checkers;
                let mut checker_sq;
                let mut not_target = Bitboard::ZERO;
                // Rust's do-while
                while {
                    checker_sq = copy_checkers.pop_lsb_unchecked();
                    not_target |= pos.effect_bb_of_checker_where_king_cannot_escape(
                        checker_sq,
                        pos.piece_on(checker_sq),
                        &pos.occupied_bb(),
                    );
                    copy_checkers.to_bool() // loop condition
                } {}
                to_bb &= !not_target;
            }
            if !AMT::ALLOW_CAPTURES {
                to_bb &= !pos.pieces_c(us.inverse());
            }
            if !AMT::ALLOW_QUIETS {
                to_bb &= pos.pieces_c(us.inverse());
            }
            to_bb
        }
        debug_assert_eq!(make_to_bb::<AMT>(pos, from, us), to_bb);
        for to in to_bb {
            self.push(Move::new_unpromote(from, to, pos.piece_on(from)));
        }
    }
    fn generate_for_bishop_or_rook<AMT: AllowMovesTrait>(&mut self, pt: PieceType, pos: &Position, target: &Bitboard) {
        debug_assert!(pos.checkers().count_ones() != 2 || !target.to_bool()); // if double check (pos.checkers() == 2), target is all zero.
        let us = pos.side_to_move();
        let from_bb = pos.pieces_cp(us, pt);
        for from in from_bb {
            let to_bb = ATTACK_TABLE.attack(pt, us, from, &pos.occupied_bb()) & *target;
            fn make_to_bb<AMT: AllowMovesTrait>(pos: &Position, pt: PieceType, from: Square, us: Color) -> Bitboard {
                let mut to_bb = ATTACK_TABLE.attack(pt, us, from, &pos.occupied_bb()) & !pos.pieces_c(us);
                if AMT::EVASIONS {
                    let checkers = pos.checkers();
                    match checkers.count_ones() {
                        1 => {
                            let ksq = pos.king_square(us);
                            to_bb &= checkers | Bitboard::between_mask(ksq, checkers.lsb_unchecked());
                        }
                        2 => to_bb = Bitboard::ZERO, // Only king can move.
                        _ => unreachable!(),
                    }
                }
                if !AMT::ALLOW_CAPTURES {
                    to_bb &= !pos.pieces_c(us.inverse());
                }
                if !AMT::ALLOW_QUIETS {
                    to_bb &= pos.pieces_c(us.inverse());
                }
                to_bb
            }
            debug_assert_eq!(make_to_bb::<AMT>(pos, pt, from, us), to_bb);
            let from_is_opponent_field = Rank::new(from).is_opponent_field(us);
            let pc = pos.piece_on(from);
            for to in to_bb {
                if from_is_opponent_field || Rank::new(to).is_opponent_field(us) {
                    self.push(Move::new_promote(from, to, pc));
                    if AMT::ALL {
                        self.push(Move::new_unpromote(from, to, pc));
                    }
                } else {
                    self.push(Move::new_unpromote(from, to, pc));
                }
            }
        }
    }
    fn generate_for_horse_or_dragon<AMT: AllowMovesTrait>(&mut self, pt: PieceType, pos: &Position, target: &Bitboard) {
        debug_assert!(pos.checkers().count_ones() != 2 || !target.to_bool()); // if double check (pos.checkers() == 2), target is all zero.
        let us = pos.side_to_move();
        let from_bb = pos.pieces_cp(us, pt);
        for from in from_bb {
            let to_bb = ATTACK_TABLE.attack(pt, us, from, &pos.occupied_bb()) & *target;
            fn make_to_bb<AMT: AllowMovesTrait>(pos: &Position, pt: PieceType, from: Square, us: Color) -> Bitboard {
                let mut to_bb = ATTACK_TABLE.attack(pt, us, from, &pos.occupied_bb()) & !pos.pieces_c(us);
                if AMT::EVASIONS {
                    let checkers = pos.checkers();
                    match checkers.count_ones() {
                        1 => {
                            let ksq = pos.king_square(us);
                            to_bb &= checkers | Bitboard::between_mask(ksq, checkers.lsb_unchecked());
                        }
                        2 => to_bb = Bitboard::ZERO, // Only king can move.
                        _ => unreachable!(),
                    }
                }
                if !AMT::ALLOW_CAPTURES {
                    to_bb &= !pos.pieces_c(us.inverse());
                }
                if !AMT::ALLOW_QUIETS {
                    to_bb &= pos.pieces_c(us.inverse());
                }
                to_bb
            }
            debug_assert_eq!(make_to_bb::<AMT>(pos, pt, from, us), to_bb);
            for to in to_bb {
                self.push(Move::new_unpromote(from, to, pos.piece_on(from)));
            }
        }
    }
    pub fn generate_recaptures(&mut self, pos: &Position, to: Square) {
        let us = pos.side_to_move();
        let from_bb = pos.attackers_to(us, to, &pos.occupied_bb());
        let to_is_opponent_field = Rank::new(to).is_opponent_field(us);
        for from in from_bb {
            let pc = pos.piece_on(from);
            let pt = PieceType::new(pc);
            match pt {
                PieceType::PAWN
                | PieceType::LANCE
                | PieceType::KNIGHT
                | PieceType::SILVER
                | PieceType::BISHOP
                | PieceType::ROOK => {
                    self.push(if to_is_opponent_field || Rank::new(from).is_opponent_field(us) {
                        Move::new_promote(from, to, pc)
                    } else {
                        Move::new_unpromote(from, to, pc)
                    });
                }
                PieceType::GOLD
                | PieceType::KING
                | PieceType::PRO_PAWN
                | PieceType::PRO_LANCE
                | PieceType::PRO_KNIGHT
                | PieceType::PRO_SILVER
                | PieceType::HORSE
                | PieceType::DRAGON => {
                    self.push(Move::new_unpromote(from, to, pc));
                }
                _ => unreachable!(),
            }
        }
    }
    fn generate_legals<ISALL: Bool>(&mut self, pos: &Position, current_size: usize) {
        if pos.in_check() {
            if ISALL::BOOL {
                self.generate_evasions::<True>(pos, current_size);
            } else {
                self.generate_evasions::<False>(pos, current_size);
            }
        } else if ISALL::BOOL {
            self.generate_all::<NonEvasionsAllType>(pos, current_size);
        } else {
            self.generate_all::<NonEvasionsType>(pos, current_size);
        }

        let mut i = 0;
        while i != self.size {
            let m = unsafe { (*self.ext_moves[i].as_ptr()).mv };
            if pos.legal(m) {
                i += 1;
            } else {
                self.size -= 1;
                unsafe {
                    (*self.ext_moves[i].as_mut_ptr()).mv = (*self.ext_moves[self.size].as_ptr()).mv;
                }
            }
        }
    }
    pub fn generate<AMT: AllowMovesTrait>(&mut self, pos: &Position, current_size: usize) {
        if AMT::LEGALS {
            if AMT::ALL {
                self.generate_legals::<True>(pos, current_size);
            } else {
                self.generate_legals::<False>(pos, current_size);
            }
        } else if AMT::EVASIONS {
            if AMT::ALL {
                self.generate_evasions::<True>(pos, current_size);
            } else {
                self.generate_evasions::<False>(pos, current_size);
            }
        } else {
            self.generate_all::<AMT>(pos, current_size);
        }
    }
}

#[test]
fn test_move_new() {
    assert_eq!(
        Move::new_unpromote(Square::SQ77, Square::SQ76, Piece::B_PAWN).to_usi_string(),
        "7g7f".to_string()
    );
    assert_eq!(
        Move::new_promote(Square::SQ74, Square::SQ73, Piece::B_PAWN).to_usi_string(),
        "7d7c+".to_string()
    );
    assert_eq!(
        Move::new_drop(Piece::B_PAWN, Square::SQ76).to_usi_string(),
        "P*7f".to_string()
    );
    assert_eq!(
        Move::new_drop(Piece::W_PAWN, Square::SQ76).to_usi_string(),
        "P*7f".to_string()
    );
}

#[test]
fn test_move_null() {
    assert!(!Move::NULL.is_promotion());
    assert!(Move::NULL.from() == Move::NULL.to());
}

#[test]
fn test_move_piece_moved() {
    for &pc in &[Piece::B_PAWN, Piece::B_SILVER, Piece::W_ROOK] {
        let (from, to) = if Color::new(pc) == Color::BLACK {
            (Square::SQ24, Square::SQ23)
        } else {
            (Square::SQ26, Square::SQ27)
        };
        assert_eq!(Move::new_promote(from, to, pc).piece_moved_before_move(), pc);
        assert_eq!(Move::new_promote(from, to, pc).piece_moved_after_move(), pc.to_promote());
        assert_eq!(Move::new_unpromote(from, to, pc).piece_moved_before_move(), pc);
        assert_eq!(Move::new_unpromote(from, to, pc).piece_moved_after_move(), pc);
        assert_eq!(Move::new_drop(pc, to).piece_moved_before_move(), pc);
        assert_eq!(Move::new_drop(pc, to).piece_moved_after_move(), pc);
    }
}

#[test]
fn test_generate_for_piece() {
    let sfen = "4k4/9/9/9/9/9/4l4/4bp3/4KP3 b - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let us = pos.side_to_move();
    let target = pos.pieces_c(us.inverse());
    mlist.generate_for_piece::<KingType, CaptureOrPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 2);
    assert!(mlist.contains(Move::new_unpromote(Square::SQ59, Square::SQ48, Piece::B_KING)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ59, Square::SQ58, Piece::B_KING))); // illegal but make this.

    let sfen = "4k4/9/9/9/9/9/4l4/4bp3/4KP3 b - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let target = pos.empty_bb();
    mlist.generate_for_piece::<KingType, QuietsWithoutPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 2);
    assert!(mlist.contains(Move::new_unpromote(Square::SQ59, Square::SQ68, Piece::B_KING)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ59, Square::SQ69, Piece::B_KING))); // illegal but make this.

    let sfen = "4k4/7p1/9/9/4BB3/5P3/9/9/s3K4 b - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let target = pos.pieces_c(us.inverse());
    mlist.generate_for_piece::<BishopType, CaptureOrPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 2);
    assert!(mlist.contains(Move::new_promote(Square::SQ55, Square::SQ22, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ99, Piece::B_BISHOP)));

    let sfen = "4k4/7p1/9/9/4BB3/5P3/9/9/s3K4 b - 1";
    let mut mlist = MoveList::new();
    let target = pos.empty_bb();
    let pos = Position::new_from_sfen(sfen).unwrap();
    mlist.generate_for_piece::<BishopType, QuietsWithoutPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 23);
    assert!(mlist.contains(Move::new_promote(Square::SQ55, Square::SQ33, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ44, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ64, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_promote(Square::SQ55, Square::SQ73, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_promote(Square::SQ55, Square::SQ82, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_promote(Square::SQ55, Square::SQ91, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ66, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ77, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ88, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ34, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ36, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ27, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ18, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ54, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ56, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ67, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ78, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ89, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_promote(Square::SQ45, Square::SQ23, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_promote(Square::SQ45, Square::SQ12, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_promote(Square::SQ45, Square::SQ63, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_promote(Square::SQ45, Square::SQ72, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_promote(Square::SQ45, Square::SQ81, Piece::B_BISHOP)));

    let sfen = "4k4/4l4/9/9/5B3/9/9/9/4K4 b - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let target = Bitboard::between_mask(Square::SQ52, Square::SQ59) | Bitboard::square_mask(Square::SQ52);
    mlist.generate_for_piece::<BishopType, EvasionsType>(&pos, &target);
    assert_eq!(mlist.size, 2);
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ54, Piece::B_BISHOP)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ56, Piece::B_BISHOP)));

    let sfens = [
        ("8k/1pP6/1G7/5G3/9/9/9/9/8K b - 1", Piece::B_GOLD),
        ("8k/1pP6/1+P7/5+P3/9/9/9/9/8K b - 1", Piece::B_PRO_PAWN),
        ("8k/1pP6/1+L7/5+L3/9/9/9/9/8K b - 1", Piece::B_PRO_LANCE),
        ("8k/1pP6/1+N7/5+N3/9/9/9/9/8K b - 1", Piece::B_PRO_KNIGHT),
        ("8k/1pP6/1+S7/5+S3/9/9/9/9/8K b - 1", Piece::B_PRO_SILVER),
    ];
    for &(sfen, pc) in sfens.iter() {
        let mut mlist = MoveList::new();
        let pos = Position::new_from_sfen(sfen).unwrap();
        let us = pos.side_to_move();
        let target = pos.pieces_c(us.inverse());
        mlist.generate_for_piece::<GoldType, CaptureOrPawnPromotionsType>(&pos, &target);
        assert_eq!(mlist.size, 1);
        assert!(mlist.contains(Move::new_unpromote(Square::SQ83, Square::SQ82, pc)));
    }

    let sfens = [
        ("8k/1pP6/1G7/5G3/9/9/9/9/8K b - 1", Piece::B_GOLD),
        ("8k/1pP6/1+P7/5+P3/9/9/9/9/8K b - 1", Piece::B_PRO_PAWN),
        ("8k/1pP6/1+L7/5+L3/9/9/9/9/8K b - 1", Piece::B_PRO_LANCE),
        ("8k/1pP6/1+N7/5+N3/9/9/9/9/8K b - 1", Piece::B_PRO_KNIGHT),
        ("8k/1pP6/1+S7/5+S3/9/9/9/9/8K b - 1", Piece::B_PRO_SILVER),
    ];
    for &(sfen, pc) in sfens.iter() {
        let mut mlist = MoveList::new();
        let pos = Position::new_from_sfen(sfen).unwrap();
        let target = pos.empty_bb();
        mlist.generate_for_piece::<GoldType, QuietsWithoutPawnPromotionsType>(&pos, &target);
        assert_eq!(mlist.size, 10);
        assert!(mlist.contains(Move::new_unpromote(Square::SQ83, Square::SQ73, pc)));
        assert!(mlist.contains(Move::new_unpromote(Square::SQ83, Square::SQ84, pc)));
        assert!(mlist.contains(Move::new_unpromote(Square::SQ83, Square::SQ92, pc)));
        assert!(mlist.contains(Move::new_unpromote(Square::SQ83, Square::SQ93, pc)));
        assert!(mlist.contains(Move::new_unpromote(Square::SQ44, Square::SQ33, pc)));
        assert!(mlist.contains(Move::new_unpromote(Square::SQ44, Square::SQ34, pc)));
        assert!(mlist.contains(Move::new_unpromote(Square::SQ44, Square::SQ43, pc)));
        assert!(mlist.contains(Move::new_unpromote(Square::SQ44, Square::SQ45, pc)));
        assert!(mlist.contains(Move::new_unpromote(Square::SQ44, Square::SQ53, pc)));
        assert!(mlist.contains(Move::new_unpromote(Square::SQ44, Square::SQ54, pc)));
    }

    let sfen = "8k/1pP6/1S7/5S3/9/9/S8/9/8K b - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let us = pos.side_to_move();
    let target = pos.pieces_c(us.inverse());
    mlist.generate_for_piece::<SilverType, CaptureOrPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 2);
    assert!(mlist.contains(Move::new_unpromote(Square::SQ83, Square::SQ82, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_promote(Square::SQ83, Square::SQ82, Piece::B_SILVER)));

    let sfen = "8k/1pP6/1S7/5S3/9/9/S8/9/8K b - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let target = pos.empty_bb();
    mlist.generate_for_piece::<SilverType, QuietsWithoutPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 17);
    assert!(mlist.contains(Move::new_unpromote(Square::SQ83, Square::SQ74, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_promote(Square::SQ83, Square::SQ74, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ83, Square::SQ92, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_promote(Square::SQ83, Square::SQ92, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ83, Square::SQ94, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_promote(Square::SQ83, Square::SQ94, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ44, Square::SQ33, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_promote(Square::SQ44, Square::SQ33, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ44, Square::SQ43, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_promote(Square::SQ44, Square::SQ43, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ44, Square::SQ53, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_promote(Square::SQ44, Square::SQ53, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ44, Square::SQ35, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ44, Square::SQ55, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ97, Square::SQ86, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ97, Square::SQ88, Piece::B_SILVER)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ97, Square::SQ96, Piece::B_SILVER)));

    let sfen = "p7k/1p7/1Np6/2N6/3N5/9/9/9/8K b - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let us = pos.side_to_move();
    let target = pos.pieces_c(us.inverse());
    mlist.generate_for_piece::<KnightType, CaptureOrPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 4);
    assert!(mlist.contains(Move::new_promote(Square::SQ83, Square::SQ91, Piece::B_KNIGHT)));
    assert!(mlist.contains(Move::new_promote(Square::SQ74, Square::SQ82, Piece::B_KNIGHT)));
    assert!(mlist.contains(Move::new_promote(Square::SQ65, Square::SQ73, Piece::B_KNIGHT)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ65, Square::SQ73, Piece::B_KNIGHT)));

    let sfen = "8k/9/9/9/3n5/2n6/1nP6/1P7/P7K w - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let us = pos.side_to_move();
    let target = pos.pieces_c(us.inverse());
    mlist.generate_for_piece::<KnightType, CaptureOrPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 4);
    assert!(mlist.contains(Move::new_promote(Square::SQ87, Square::SQ99, Piece::W_KNIGHT)));
    assert!(mlist.contains(Move::new_promote(Square::SQ76, Square::SQ88, Piece::W_KNIGHT)));
    assert!(mlist.contains(Move::new_promote(Square::SQ65, Square::SQ77, Piece::W_KNIGHT)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ65, Square::SQ77, Piece::W_KNIGHT)));

    let sfen = "p7k/1p7/1Np6/2N6/3N5/9/9/9/8K b - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let target = pos.empty_bb();
    mlist.generate_for_piece::<KnightType, QuietsWithoutPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 4);
    assert!(mlist.contains(Move::new_promote(Square::SQ83, Square::SQ71, Piece::B_KNIGHT)));
    assert!(mlist.contains(Move::new_promote(Square::SQ74, Square::SQ62, Piece::B_KNIGHT)));
    assert!(mlist.contains(Move::new_promote(Square::SQ65, Square::SQ53, Piece::B_KNIGHT)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ65, Square::SQ53, Piece::B_KNIGHT)));

    let sfen = "8k/9/9/9/3n5/2n6/1nP6/1P7/P7K w - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let target = pos.empty_bb();
    mlist.generate_for_piece::<KnightType, QuietsWithoutPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 4);
    assert!(mlist.contains(Move::new_promote(Square::SQ87, Square::SQ79, Piece::W_KNIGHT)));
    assert!(mlist.contains(Move::new_promote(Square::SQ76, Square::SQ68, Piece::W_KNIGHT)));
    assert!(mlist.contains(Move::new_promote(Square::SQ65, Square::SQ57, Piece::W_KNIGHT)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ65, Square::SQ57, Piece::W_KNIGHT)));

    let sfen = "p7k/1p7/2p6/9/LLLL5/9/9/9/8K b - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let us = pos.side_to_move();
    let target = pos.pieces_c(us.inverse());
    mlist.generate_for_piece::<LanceType, CaptureOrPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 4);
    assert!(mlist.contains(Move::new_promote(Square::SQ75, Square::SQ73, Piece::B_LANCE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ75, Square::SQ73, Piece::B_LANCE)));
    assert!(mlist.contains(Move::new_promote(Square::SQ85, Square::SQ82, Piece::B_LANCE)));
    assert!(mlist.contains(Move::new_promote(Square::SQ95, Square::SQ91, Piece::B_LANCE)));

    let sfen = "p7k/1p7/2p6/9/LLLL5/9/9/9/8K b - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let target = pos.empty_bb();
    mlist.generate_for_piece::<LanceType, QuietsWithoutPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 10);
    assert!(mlist.contains(Move::new_promote(Square::SQ65, Square::SQ61, Piece::B_LANCE)));
    assert!(mlist.contains(Move::new_promote(Square::SQ65, Square::SQ62, Piece::B_LANCE)));
    assert!(mlist.contains(Move::new_promote(Square::SQ65, Square::SQ63, Piece::B_LANCE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ65, Square::SQ64, Piece::B_LANCE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ75, Square::SQ74, Piece::B_LANCE)));
    assert!(mlist.contains(Move::new_promote(Square::SQ85, Square::SQ83, Piece::B_LANCE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ85, Square::SQ84, Piece::B_LANCE)));
    assert!(mlist.contains(Move::new_promote(Square::SQ95, Square::SQ92, Piece::B_LANCE)));
    assert!(mlist.contains(Move::new_promote(Square::SQ95, Square::SQ93, Piece::B_LANCE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ95, Square::SQ94, Piece::B_LANCE)));

    let sfen = "p7k/PPp6/2PPp4/4PPp2/6PP1/9/9/9/8K b - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let us = pos.side_to_move();
    let target = pos.pieces_c(us.inverse()) | (pos.empty_bb() & Bitboard::opponent_field_mask(us));
    mlist.generate_for_piece::<PawnType, CaptureOrPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 7);
    assert!(mlist.contains(Move::new_promote(Square::SQ92, Square::SQ91, Piece::B_PAWN)));
    assert!(mlist.contains(Move::new_promote(Square::SQ82, Square::SQ81, Piece::B_PAWN)));
    assert!(mlist.contains(Move::new_promote(Square::SQ73, Square::SQ72, Piece::B_PAWN)));
    assert!(mlist.contains(Move::new_promote(Square::SQ63, Square::SQ62, Piece::B_PAWN)));
    assert!(mlist.contains(Move::new_promote(Square::SQ54, Square::SQ53, Piece::B_PAWN)));
    assert!(mlist.contains(Move::new_promote(Square::SQ44, Square::SQ43, Piece::B_PAWN)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ35, Square::SQ34, Piece::B_PAWN)));

    let sfen = "p7k/PPp6/2PPp4/4PPp2/6PP1/9/9/9/8K b - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let us = pos.side_to_move();
    let target = pos.empty_bb() & !Bitboard::opponent_field_mask(us);
    mlist.generate_for_piece::<PawnType, QuietsWithoutPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 1);
    assert!(mlist.contains(Move::new_unpromote(Square::SQ25, Square::SQ24, Piece::B_PAWN)));

    let sfen = "4k4/7p1/9/9/4+B+B3/5P3/9/9/s3K4 b - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let us = pos.side_to_move();
    let target = pos.pieces_c(us.inverse());
    mlist.generate_for_piece::<HorseType, CaptureOrPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 2);
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ22, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ99, Piece::B_HORSE)));

    let sfen = "4k4/7p1/9/9/4+B+B3/5P3/9/9/s3K4 b - 1";
    let mut mlist = MoveList::new();
    let pos = Position::new_from_sfen(sfen).unwrap();
    let target = pos.empty_bb();
    mlist.generate_for_piece::<HorseType, QuietsWithoutPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 28);
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ33, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ44, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ64, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ73, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ82, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ91, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ66, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ77, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ88, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ54, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ56, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ55, Square::SQ65, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ34, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ36, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ27, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ18, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ54, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ56, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ67, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ78, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ89, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ23, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ12, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ63, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ72, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ81, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ35, Piece::B_HORSE)));
    assert!(mlist.contains(Move::new_unpromote(Square::SQ45, Square::SQ44, Piece::B_HORSE)));
}
#[test]
fn test_generate_recaptures() {
    let sfen = "k1B1R1+B2/9/4p+R3/3SPG3/3N5/9/9/9/K8 b p 1";
    let pos = Position::new_from_sfen(sfen).unwrap();
    let mut mlist = MoveList::new();
    let capture_square = Square::SQ53;
    mlist.generate_recaptures(&pos, capture_square);
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "4453KI")
        .is_some());
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "5453TO")
        .is_some());
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "6453NG")
        .is_some());
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "6553NK")
        .is_some());
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "7153UM")
        .is_some());
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "5153RY")
        .is_some());
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "4353RY")
        .is_some());
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "3153UM")
        .is_some());
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "7153KA")
        .is_none());
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "6553KE")
        .is_none());
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "5453FU")
        .is_none());
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "6453GI")
        .is_none());
}
#[test]
fn test_generate_drop() {
    let sfen = "l6nl/5+P1gk/2np1S3/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL w GR5pnsg 1";
    let pos = Position::new_from_sfen(sfen).unwrap();
    let mut mlist = MoveList::new();
    let target = pos.empty_bb();
    mlist.generate_drop::<QuietsWithoutPawnPromotionsType>(&pos, &target);
    assert_eq!(mlist.size, 167);

    let sfen = "l5+R2/1k2r2p1/1sngn4/l1ppp2P1/5pp2/lPPPP4/1KSG4P/1SSB5/1N1G4+b w GLPn5p 130";
    let pos = Position::new_from_sfen(sfen).unwrap();
    let mut mlist = MoveList::new();
    let target = pos.empty_bb();
    mlist.generate_drop::<QuietsWithoutPawnPromotionsType>(&pos, &target);
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "0081FU")
        .is_some());
    assert!(Move::new_from_csa_str(&"0081FU", &pos).is_some());

    let sfen = "ln3G2l/7k1/3pgsn2/2p2bpp1/p4p3/3sSbn1P/P2P1GPP1/2+r3S1K/L3RG1NL w P6p 106";
    let pos = Position::new_from_sfen(sfen).unwrap();
    let mut mlist = MoveList::new();
    let target = pos.empty_bb();
    mlist.generate_drop::<QuietsWithoutPawnPromotionsType>(&pos, &target);
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "0017FU")
        .is_some());
    assert!(Move::new_from_csa_str(&"0017FU", &pos).is_some());
}
#[test]
fn test_generate_evasion() {
    let sfen = "9/4k4/r8/3b5/4L4/9/9/9/4K4 w pnsg 1";
    let pos = Position::new_from_sfen(sfen).unwrap();

    let mut mlist = MoveList::new();
    mlist.generate::<EvasionsType>(&pos, 0);
    assert_eq!(mlist.size, 17);
    assert_eq!(
        mlist
            .slice(0)
            .iter()
            .filter(|&x| x.mv.piece_moved_before_move() == Piece::W_ROOK)
            .count(),
        1
    );
    assert_eq!(
        mlist
            .slice(0)
            .iter()
            .filter(|&x| x.mv.piece_moved_before_move() == Piece::W_BISHOP)
            .count(),
        2
    );
    assert_eq!(
        mlist
            .slice(0)
            .iter()
            .filter(|&x| x.mv.piece_moved_before_move() == Piece::W_KING)
            .count(),
        6
    );
    assert_eq!(mlist.slice(0).iter().filter(|&x| x.mv.is_drop()).count(), 8);
}

#[test]
fn test_generate_all() {
    let sfen = "l6nl/5+P1gk/2np1S3/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL w GR5pnsg 1";
    let pos = Position::new_from_sfen(sfen).unwrap();

    let mut mlist = MoveList::new();
    mlist.generate_all::<NonEvasionsType>(&pos, 0);
    assert_eq!(mlist.size, 199);

    let mut mlist = MoveList::new();
    mlist.generate_all::<CaptureOrPawnPromotionsType>(&pos, 0);
    assert_eq!(mlist.size, 2);

    let mut mlist = MoveList::new();
    mlist.generate_all::<QuietsWithoutPawnPromotionsType>(&pos, 0);
    assert_eq!(mlist.size, 197);
}

#[test]
fn test_move_new_from_csa_str() {
    let sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let pos = Position::new_from_sfen(sfen).unwrap();

    let m_str = "7776FU";
    if let Some(m) = Move::new_from_csa_str(m_str, &pos) {
        assert_eq!(m.to_csa_string(&pos), m_str);
    } else {
        assert!(false);
    }
    let m_str_illegal = "7775FU";
    assert!(Move::new_from_csa_str(m_str_illegal, &pos).is_none());
}

#[test]
fn test_pawn_drop_mate() {
    let sfen = "kl7/1n7/K8/9/9/9/9/9/9 b P 1";
    let pos = Position::new_from_sfen(sfen).unwrap();
    let mut mlist = MoveList::new();
    mlist.generate_all::<NonEvasionsType>(&pos, 0);
    assert!(mlist
        .slice(0)
        .iter()
        .find(|&x| x.mv.to_csa_string(&pos) == "0092FU")
        .is_none());
}

#[test]
fn test_is_normal_move() {
    assert!(!None.is_normal_move());
    assert!(!Some(Move::NULL).is_normal_move());
    assert!(!Some(Move::WIN).is_normal_move());
    assert!(!Some(Move::RESIGN).is_normal_move());
    assert!(Some(Move::new_unpromote(Square::SQ11, Square::SQ12, Piece::W_PAWN)).is_normal_move());
    assert!(Some(Move::new_drop(Piece::B_PAWN, Square::SQ12)).is_normal_move());
}

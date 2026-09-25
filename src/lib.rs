#![allow(dead_code)]

/// Thin pieces should be regarded to be the "same" color as the background, e.g. black background -> thin = black.
/// Fat pieces should be regarded to be the "same" color as the text, e.g. black text -> fat = black.
mod piece_repr {
    use super::PieceType;
    // In unicode these are regarded as white
    mod thin {
        pub(crate) const KING: &'static str = "♔";
        pub(crate) const QUEEN: &'static str = "♕";
        pub(crate) const ROOK: &'static str = "♖";
        pub(crate) const BISHOP: &'static str = "♗";
        pub(crate) const KNIGHT: &'static str = "♘";
        pub(crate) const PAWN: &'static str = "♙";
    }
    // In unicode these are regarded as black
    mod fat {
        pub(crate) const KING: &'static str = "♚";
        pub(crate) const QUEEN: &'static str = "♛";
        pub(crate) const ROOK: &'static str = "♜";
        pub(crate) const BISHOP: &'static str = "♝";
        pub(crate) const KNIGHT: &'static str = "♞";
        pub(crate) const PAWN: &'static str = "♟";
    }
    pub(crate) fn match_thin(pt: PieceType) -> &'static str {
        return match pt {
            PieceType::PAWN => thin::PAWN,
            PieceType::ROOK => thin::ROOK,
            PieceType::KNIGHT => thin::KNIGHT,
            PieceType::BISHOP => thin::BISHOP,
            PieceType::QUEEN => thin::QUEEN,
            PieceType::KING => thin::KING,
        };
    }
    pub(crate) fn match_fat(pt: PieceType) -> &'static str {
        return match pt {
            PieceType::PAWN => fat::PAWN,
            PieceType::ROOK => fat::ROOK,
            PieceType::KNIGHT => fat::KNIGHT,
            PieceType::BISHOP => fat::BISHOP,
            PieceType::QUEEN => fat::QUEEN,
            PieceType::KING => fat::KING,
        };
    }
}



trait Shiftable<S, E> {
    fn shift(self: Self, shift: i8) -> Result<S, E>; 
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum File {
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
}

impl TryFrom<i8> for File {
    type Error = &'static str;
    fn try_from(value: i8) -> Result<File, Self::Error> {
        return match value {
            1 => Ok(File::A),
            2 => Ok(File::B),
            3 => Ok(File::C),
            4 => Ok(File::D),
            5 => Ok(File::E),
            6 => Ok(File::F),
            7 => Ok(File::G),
            8 => Ok(File::H),
            _ => Err("Value out of bounds"),
        };
    }
}

#[test]
fn test_file_try_from() {
    for i in 1..=8 {
        assert!(File::try_from(i).is_ok());
    }
    assert!(File::try_from(-1).is_err());
    assert!(File::try_from(0).is_err());
    assert!(File::try_from(9).is_err());
    assert!(File::try_from(i8::MAX).is_err());
    assert!(File::try_from(i8::MIN).is_err());
}

pub type Rank = i8;


impl Shiftable<Rank, &'static str> for Rank {
    fn shift(self: Rank, offset: i8) -> Result<Rank, &'static str> {
        if self + offset < 1 || self + offset > 8 {
            return Err("Shift out of bounds");
        }
        return Ok(self+offset);
    }
}

#[test]
fn test_rank_shift() {
    assert!(Rank::try_from(1).unwrap().shift(0).is_ok());
    assert!(Rank::try_from(1).unwrap().shift(1).is_ok());
    assert!(Rank::try_from(1).unwrap().shift(7).is_ok());
    assert!(Rank::try_from(1).unwrap().shift(-1).is_err());

    assert!(Rank::try_from(8).unwrap().shift(0).is_ok());
    assert!(Rank::try_from(8).unwrap().shift(-1).is_ok());
    assert!(Rank::try_from(8).unwrap().shift(-7).is_ok());
    assert!(Rank::try_from(8).unwrap().shift(1).is_err());
}

impl Shiftable<File, &'static str> for File {
    fn shift(self: File, offset: i8) -> Result<File, &'static str> {
        let s: i8 = self as i8;
        if s + offset < 1 || s + offset > 8 {
            return Err("Shift out of bounds");
        }
        return File::try_from(s + offset);
    }
}

#[test]
fn test_file_shift() {
    assert!(File::try_from(1).unwrap().shift(0).is_ok());
    assert!(File::try_from(1).unwrap().shift(1).is_ok());
    assert!(File::try_from(1).unwrap().shift(7).is_ok());
    assert!(File::try_from(1).unwrap().shift(-1).is_err());

    assert!(File::try_from(8).unwrap().shift(0).is_ok());
    assert!(File::try_from(8).unwrap().shift(-1).is_ok());
    assert!(File::try_from(8).unwrap().shift(-7).is_ok());
    assert!(File::try_from(8).unwrap().shift(1).is_err());
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Position {
    file: File,
    rank: Rank,
}

impl Position {
    pub fn new(file: File, rank: Rank) -> Position {
        if rank > 8 || rank < 1 {
            panic!("Rank out of bounds");
        }
        return Position {
            file: file,
            rank: rank,
        };
    }
    fn shift_clone(self: Position, file_offset: i8, rank_offset: i8) -> Result<Position, &'static str> {
        let mut p: Position = self.clone();
        match p.file.shift(file_offset) {
            Ok(f) => p.file = f, 
            Err(m) => return Err(m),
        }
        match p.rank.shift(rank_offset) {
            Ok(r) => p.rank = r, 
            Err(m) => return Err(m),
        }
        return Ok(p);
    }
}

#[test]
fn test_position_new() {
    for i in 1..=8 {
        Position::new(File::A, i);
    }
}

#[test]
#[should_panic]
fn test_position_new_panic_minus1() {
    Position::new(File::A, -1);
}

#[test]
#[should_panic]
fn test_position_new_panic_0() {
    Position::new(File::A, 0);
}

#[test]
#[should_panic]
fn test_position_new_panic_9() {
    Position::new(File::A, 9);
}

#[test]
#[should_panic]
fn test_position_new_panic_max() {
    Position::new(File::A, i8::MAX);
}

#[test]
#[should_panic]
fn test_position_new_panic_min() {
    Position::new(File::A, i8::MIN);
}


#[derive(Clone, Copy, Debug)]
struct BitBoard(u64);

impl BitBoard {
    fn new(position: Position) -> BitBoard {
        if position.rank > 8 || position.rank < 1 {
            panic!("Rank out of bounds");
        }
        return BitBoard {
            0: 1<<((position.file as u8 -1) + 8*((8-position.rank) as u8)),
        };
    }
    fn crowded_bitboard(positions: Vec<Position>) -> BitBoard {
        let mut b: BitBoard = BitBoard { 0: 0 };
        for p in positions {
            b.0 |= 1<<((p.file as u8 - 1) + 8*((8-p.rank) as u8));
        }
        return b;
    }

    fn at_position(&self, position: Position) -> bool {
        return self.0 & 1<<((position.file as u8 - 1) + 8*((8-position.rank) as u8)) != 0;
    }

    /// Returns bitboard with new position on success and old position on error.
    fn translate(self: BitBoard, new_position: Position) -> Result<BitBoard, BitBoard> {
        return Ok(BitBoard::new(new_position));
    } 
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    PAWN = 0,
    ROOK = 1,
    KNIGHT = 2,
    BISHOP = 3,
    QUEEN = 4,
    KING = 5,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Color {
    BLACK = 0,
    WHITE = 1,
}

impl Color {
    fn invert(&self) -> Color {
        return match self{ Color::BLACK => Color::WHITE, Color::WHITE => Color::BLACK}
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Ply {
    Quiet {
        old_position: Position,
        new_position: Position,
    },
    Capture {
        old_position: Position,
        new_position: Position,
        captured_piece: PieceType,
    },
}


#[derive(Debug, Clone, Copy)]
struct Piece {
    bitboard: BitBoard,
    position: Position,
    piece_type: PieceType,
    color: Color,
}
#[test]
#[should_panic]
fn test_piece_capture_different_old_pos() {
    let mut p = Piece::new(Position::new(File::A, 1), PieceType::PAWN, Color::WHITE);
    p.perform_ply(Ply::Capture { old_position: Position::new(File::A, 2), new_position: Position::new(File::B, 3), captured_piece: PieceType::PAWN });
}
#[test]
#[should_panic]
fn test_piece_quiet_different_old_pos() {
    let mut p = Piece::new(Position::new(File::A, 1), PieceType::PAWN, Color::WHITE);
    p.perform_ply(Ply::Quiet { old_position: Position::new(File::A, 2), new_position: Position::new(File::A, 3)});
}

impl Piece {
    fn perform_ply(self: &mut Self, ply: Ply) {
        match ply {
            Ply::Capture {
                old_position,
                new_position,
                captured_piece: _
            } => {
                if old_position != self.position { panic!("Mismatched position!"); }
                self.position = new_position;
            },
            Ply::Quiet {
                old_position,
                new_position
            } => {
                if old_position != self.position { panic!("Mismatched position!"); }
                self.position = new_position;
            },
        }
    }
    fn new(position: Position, piece_type: PieceType, color: Color) -> Piece {
        let bitboard: BitBoard = BitBoard::new(position.clone());
        return Piece {
            bitboard,
            position,
            piece_type,
            color,
        };
    }
    fn get_color(self: &Self) -> Color {
        return self.color.clone();
    }
    fn get_bitboard(self: &Self) -> BitBoard {
        return BitBoard::new(self.position.clone());
    }
    fn get_type(self: &Self) -> PieceType {
        return self.piece_type;
    }
    fn compile_bitboard(&self, game: &Game) -> BitBoard {
        let mut v: Vec<Position> = Vec::new();
        for i in game.pieces.iter() {
            v.push(i.position);
        }
        return BitBoard::crowded_bitboard(v);
    }
    fn compile_bitboard_color(&self, game: &Game, color: Color) -> BitBoard {
        let mut v: Vec<Position> = Vec::new();
        for i in game.pieces.iter() {
            if i.color == color {
                v.push(i.position);
            }
        }
        return BitBoard::crowded_bitboard(v);
    }
    fn find_plies_pawn(self: &Self, game: &Game) -> Vec<Ply> {
        let mut plys: Vec<Ply> = Vec::new();
        let flip: i8 = match self.color {
            Color::BLACK => -1,
            Color::WHITE => 1,
        };
        match self.position.shift_clone(0, 1*flip) {
            Ok(p) => {
                if game.find_piece_at_position(p) == Option::None {
                    plys.push(Ply::Quiet { old_position: self.position.clone(), new_position: p })
                }
                if (flip == 1 && self.position.rank == 2) || (flip == -1 && self.position.rank == 7) {
                    match self.position.shift_clone(0, 2*flip) {
                        Ok(p) => { match game.find_piece_at_position(p) {
                            Option::Some(_) => (),
                            Option::None => plys.push(Ply::Quiet { old_position: self.position.clone(), new_position: p }),
                        }}, _ => (),
                    }
                }
            }, _ => (),
        }
        match self.position.shift_clone(1, 1*flip) {
            Ok(p) => { match game.find_piece_at_position(p) {
                Option::Some(idx) => plys.push(
                    Ply::Capture {
                        old_position: self.position.clone(),
                        new_position: p,
                        captured_piece: game.pieces.get(idx).unwrap().piece_type}
                    ),
                    Option::None => (),
            }}, _ => (),
        }
        match self.position.shift_clone(-1, 1*flip) {
            Ok(p) => { match game.find_piece_at_position(p) {
                Option::Some(idx) => plys.push(
                    Ply::Capture {
                        old_position: self.position.clone(),
                        new_position: p,
                        captured_piece: game.pieces.get(idx).unwrap().piece_type}
                    ),
                    Option::None => (),
            }}, _ => (),
        }
        return plys;
    }

    fn find_plies_rook(self: &Self, game: &Game) -> Vec<Ply> {
        let mut plys: Vec<Ply> = Vec::new();
        let bb: BitBoard = self.compile_bitboard(game);
        let bbo: BitBoard = self.compile_bitboard_color(game, self.color.invert());
        for r in (self.position.rank+1)..=8 {
            if bb.at_position(Position::new(self.position.file.clone(), r)) {
                if bbo.at_position(Position::new(self.position.file.clone(), r)) {
                    let new_pos = self.position.shift_clone(0, r-self.position.rank).unwrap();
                    plys.push(Ply::Capture { old_position: self.position.clone(), new_position: new_pos, captured_piece: game.pieces.get(game.find_piece_at_position(new_pos).unwrap()).unwrap().piece_type });
                }
                break;
            } else {
                plys.push(Ply::Quiet { old_position: self.position.clone(), new_position: self.position.shift_clone(0, r-self.position.rank).unwrap()});
            }
        }

        for r in (1..=(self.position.rank-1)).rev() {
            if bb.at_position(Position::new(self.position.file.clone(), r)) {
                if bbo.at_position(Position::new(self.position.file.clone(), r)) {
                    let new_pos = self.position.shift_clone(0, r-self.position.rank).unwrap();
                    plys.push(Ply::Capture {
                        old_position: self.position.clone(),
                        new_position: new_pos,
                        captured_piece: game.pieces.get(game.find_piece_at_position(new_pos).unwrap()).unwrap().piece_type
                    });
                }
                break;
            } else {
                plys.push(Ply::Quiet { old_position: self.position.clone(), new_position: self.position.shift_clone(0, r-self.position.rank).unwrap()});
            }
        }

        for fi in (self.position.file as i8 + 1)..=8 {
            let f = File::try_from(fi).unwrap();
            if bb.at_position(Position::new(f.clone(), self.position.rank.clone())) {
                if bbo.at_position(Position::new(f.clone(), self.position.rank.clone())) {
                    let new_pos = Position::new(f.clone(), self.position.rank.clone());
                    plys.push(Ply::Capture { old_position: self.position.clone(), new_position: new_pos, captured_piece: game.pieces.get(game.find_piece_at_position(new_pos).unwrap()).unwrap().piece_type });
                }
                break;
            } else {
                let new_pos = Position::new(f.clone(), self.position.rank.clone());
                plys.push(Ply::Quiet { old_position: self.position.clone(), new_position: new_pos});
            }
        }
        for fi in (1..=(self.position.file as i8 - 1)).rev() {
            let f = File::try_from(fi).unwrap();
            if bb.at_position(Position::new(f.clone(), self.position.rank.clone())) {
                if bbo.at_position(Position::new(f.clone(), self.position.rank.clone())) {
                    let new_pos = Position::new(f.clone(), self.position.rank.clone());
                    plys.push(Ply::Capture { old_position: self.position.clone(), new_position: new_pos, captured_piece: game.pieces.get(game.find_piece_at_position(new_pos).unwrap()).unwrap().piece_type });
                } 
                break;
            } else {
                let new_pos = Position::new(f.clone(), self.position.rank.clone());
                plys.push(Ply::Quiet { old_position: self.position.clone(), new_position: new_pos});
            }
        }
        return plys;
    }

    fn find_plies_knight(self: &Self, game: &Game) -> Vec<Ply> {
        let mut possible: Vec<Result<Position, &'static str>> = Vec::new();
        possible.push(self.position.shift_clone(2, 1));
        possible.push(self.position.shift_clone(2, -1));
        possible.push(self.position.shift_clone(1, 2));
        possible.push(self.position.shift_clone(1, -2));

        possible.push(self.position.shift_clone(-2, 1));
        possible.push(self.position.shift_clone(-2, -1));
        possible.push(self.position.shift_clone(-1, 2));
        possible.push(self.position.shift_clone(-1, -2));
        let mut actual: Vec<Ply> = Vec::new();
        for p in possible {
            match p {
                Err(_) => continue,
                Ok(pos) => {
                    match game.find_piece_at_position(pos) {
                        None => actual.push(Ply::Quiet { old_position: self.position.clone(), new_position: pos }),
                        Some(idx) => {
                            let piece = game.pieces.get(idx).unwrap();
                            if piece.color == self.color {
                                continue;
                            } else {
                                actual.push(Ply::Capture { old_position: self.position.clone(), new_position: piece.position.clone(), captured_piece: piece.get_type()})
                            }
                        }
                    }
                },
            }
        }
        return actual;
    }

    fn find_plies_bishop(self: &Self, game: &Game) -> Vec<Ply> {
        let mut plys: Vec<Ply> = Vec::new();
        let bb: BitBoard = self.compile_bitboard(game);
        let bbo: BitBoard = self.compile_bitboard_color(game, self.color.invert());
        for i in 1..=8 {
            match self.position.shift_clone(i, i) {
                Err(_) => break,
                Ok(pos) => {
                    if bb.at_position(pos.clone()) {
                        if bbo.at_position(pos.clone()) {
                            plys.push(Ply::Capture { old_position: self.position.clone(), new_position: pos, captured_piece: game.pieces.get(game.find_piece_at_position(pos).unwrap()).unwrap().piece_type });
                        } else {
                            break;
                        }
                    } else {
                        plys.push(Ply::Quiet { old_position: self.position.clone(), new_position: pos});
                }
            }
        }
        }

        for i in 1..=8 {
            match self.position.shift_clone(-1*i, i) {
                Err(_) => break,
                Ok(pos) => {
                    if bb.at_position(pos.clone()) {
                        if bbo.at_position(pos.clone()) {
                            plys.push(Ply::Capture { old_position: self.position.clone(), new_position: pos, captured_piece: game.pieces.get(game.find_piece_at_position(pos).unwrap()).unwrap().piece_type });
                        } else {
                            break;
                        }
                    } else {
                        plys.push(Ply::Quiet { old_position: self.position.clone(), new_position: pos});
                }
            }
        }
        }
        for i in 1..=8 {
            match self.position.shift_clone(i, -1*i) {
                Err(_) => break,
                Ok(pos) => {
                    if bb.at_position(pos.clone()) {
                        if bbo.at_position(pos.clone()) {
                            plys.push(Ply::Capture { old_position: self.position.clone(), new_position: pos, captured_piece: game.pieces.get(game.find_piece_at_position(pos).unwrap()).unwrap().piece_type });
                        } else {
                            break;
                        }
                    } else {
                        plys.push(Ply::Quiet { old_position: self.position.clone(), new_position: pos});
                }
            }
        }
        }
        for i in 1..=8 {
            match self.position.shift_clone(-1*i, -1*i) {
                Err(_) => break,
                Ok(pos) => {
                    if bb.at_position(pos.clone()) {
                        if bbo.at_position(pos.clone()) {
                            plys.push(Ply::Capture { old_position: self.position.clone(), new_position: pos, captured_piece: game.pieces.get(game.find_piece_at_position(pos).unwrap()).unwrap().piece_type });
                        } else {
                            break;
                        }
                    } else {
                        plys.push(Ply::Quiet { old_position: self.position.clone(), new_position: pos});
                }
            }
        }
        }

        return plys;
    }

    fn find_plies_queen(self: &Self, game: &Game) -> Vec<Ply> {
        let mut p: Vec<Ply> = self.find_plies_bishop(game);
        p.extend(self.find_plies_rook(game));
        return p;
    }
    fn find_plies_king(self: &Self, game: &Game) -> Vec<Ply> {
        let mut possible: Vec<Result<Position, &'static str>> = Vec::new();
        possible.push(self.position.shift_clone(1, -1));
        possible.push(self.position.shift_clone(1, 0));
        possible.push(self.position.shift_clone(1, 1));

        possible.push(self.position.shift_clone(-1, -1));
        possible.push(self.position.shift_clone(-1, 0));
        possible.push(self.position.shift_clone(-1, 1));

        possible.push(self.position.shift_clone(0, -1));
        possible.push(self.position.shift_clone(0, 1));
        let mut actual: Vec<Ply> = Vec::new();
        for p in possible {
            match p {
                Err(_) => continue,
                Ok(pos) => {
                    match game.find_piece_at_position(pos) {
                        None => actual.push(Ply::Quiet { old_position: self.position.clone(), new_position: pos }),
                        Some(idx) => {
                            let piece = game.pieces.get(idx).unwrap();
                            if piece.color == self.color {
                                continue;
                            } else {
                                actual.push(Ply::Capture { old_position: self.position.clone(), new_position: piece.position.clone(), captured_piece: piece.get_type()})
                            }
                        }
                    }
                },
            }
        }
        return actual;
    }
        
    fn find_plies(self: &Self, game: &Game) -> Vec<Ply> {
        return match self.piece_type {
            PieceType::PAWN => self.find_plies_pawn(game),
            PieceType::ROOK => self.find_plies_rook(game),
            PieceType::KNIGHT => self.find_plies_knight(game),
            PieceType::BISHOP => self.find_plies_bishop(game),
            PieceType::QUEEN => self.find_plies_queen(game),
            PieceType::KING => self.find_plies_king(game),
        };
    }
}



#[derive(Debug, Clone, Copy)]
pub struct PieceRepresentation {
    pub color: Color,
    pub piece_type: PieceType,
}

impl PieceRepresentation {
    fn new(color: Color, piece_type: PieceType) -> PieceRepresentation {
        return PieceRepresentation { color, piece_type };
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pieces: Vec<Piece>,
    is_dark: bool,
    turn: Color,
}

impl Game {
    pub fn dark_mode(&mut self, x: bool) {
        self.is_dark = x;
    }

    pub fn new() -> Game {
        let mut game: Game = Game {
            pieces: Vec::new(),
            is_dark: true,
            turn: Color::WHITE,
        };
        game.pieces.push(Piece::new(Position::new(File::A, 8), PieceType::ROOK, Color::BLACK));
        game.pieces.push(Piece::new(Position::new(File::B, 8), PieceType::KNIGHT, Color::BLACK));
        game.pieces.push(Piece::new(Position::new(File::C, 8), PieceType::BISHOP, Color::BLACK));
        game.pieces.push(Piece::new(Position::new(File::D, 8), PieceType::KING, Color::BLACK));
        game.pieces.push(Piece::new(Position::new(File::E, 8), PieceType::QUEEN, Color::BLACK));
        game.pieces.push(Piece::new(Position::new(File::F, 8), PieceType::BISHOP, Color::BLACK));
        game.pieces.push(Piece::new(Position::new(File::G, 8), PieceType::KNIGHT, Color::BLACK));
        game.pieces.push(Piece::new(Position::new(File::H, 8), PieceType::ROOK, Color::BLACK));

        game.pieces.push(Piece::new(Position::new(File::A, 7), PieceType::PAWN, Color::BLACK));
        game.pieces.push(Piece::new(Position::new(File::B, 7), PieceType::PAWN, Color::BLACK));
        game.pieces.push(Piece::new(Position::new(File::C, 7), PieceType::PAWN, Color::BLACK));
        game.pieces.push(Piece::new(Position::new(File::D, 7), PieceType::PAWN, Color::BLACK));
        game.pieces.push(Piece::new(Position::new(File::E, 7), PieceType::PAWN, Color::BLACK));
        game.pieces.push(Piece::new(Position::new(File::F, 7), PieceType::PAWN, Color::BLACK));
        game.pieces.push(Piece::new(Position::new(File::G, 7), PieceType::PAWN, Color::BLACK));
        game.pieces.push(Piece::new(Position::new(File::H, 7), PieceType::PAWN, Color::BLACK));

        game.pieces.push(Piece::new(Position::new(File::A, 2), PieceType::PAWN, Color::WHITE));
        game.pieces.push(Piece::new(Position::new(File::B, 2), PieceType::PAWN, Color::WHITE));
        game.pieces.push(Piece::new(Position::new(File::C, 2), PieceType::PAWN, Color::WHITE));
        game.pieces.push(Piece::new(Position::new(File::D, 2), PieceType::PAWN, Color::WHITE));
        game.pieces.push(Piece::new(Position::new(File::E, 2), PieceType::PAWN, Color::WHITE));
        game.pieces.push(Piece::new(Position::new(File::F, 2), PieceType::PAWN, Color::WHITE));
        game.pieces.push(Piece::new(Position::new(File::G, 2), PieceType::PAWN, Color::WHITE));
        game.pieces.push(Piece::new(Position::new(File::H, 2), PieceType::PAWN, Color::WHITE));

        game.pieces.push(Piece::new(Position::new(File::A, 1), PieceType::ROOK, Color::WHITE));
        game.pieces.push(Piece::new(Position::new(File::B, 1), PieceType::KNIGHT, Color::WHITE));
        game.pieces.push(Piece::new(Position::new(File::C, 1), PieceType::BISHOP, Color::WHITE));
        game.pieces.push(Piece::new(Position::new(File::D, 1), PieceType::KING, Color::WHITE));
        game.pieces.push(Piece::new(Position::new(File::E, 1), PieceType::QUEEN, Color::WHITE));
        game.pieces.push(Piece::new(Position::new(File::F, 1), PieceType::BISHOP, Color::WHITE));
        game.pieces.push(Piece::new(Position::new(File::G, 1), PieceType::KNIGHT, Color::WHITE));
        game.pieces.push(Piece::new(Position::new(File::H, 1), PieceType::ROOK, Color::WHITE));

        return game;
    }
    /// Returns error if not piece is found at position.
    fn remove_piece(self: &mut Self, position: Position) -> Result<(), ()> {
        return match self.find_piece_at_position(position) {
            Some(index) => {self.pieces.remove(index); Ok(())},
            None => Err(()),
        };
    }

    fn find_piece_at_position(self: &Self, position: Position) -> Option<usize> {
        let mut index: usize = 0;
        let mut found: bool = false;
        for (i, p) in self.pieces.iter().enumerate() {
            index = i;
            if p.position == position {
                found = true;
                break;
            }
        }
        if !found {
            return None;
        }
        return Some(index);
    }
    pub fn whose_turn(&self) -> Color {
        return self.turn;
    }

    pub fn find_plies(self: &mut Self, position: Position) -> Option<Vec<Ply>> {
        let p = self.pieces.get(self.find_piece_at_position(position)?)?;
        if p.color != self.turn {
            return None;
        } else {
            return Some(p.find_plies(self));
        }
    }
    pub fn perform_ply(self: &mut Self, ply: Ply) {
        match ply {
            Ply::Quiet { old_position, new_position } => {
                if self.find_piece_at_position(new_position).is_some() {
                    panic!("Piece already at position {:?}", new_position);
                }
                let i = self.find_piece_at_position(old_position.clone()).unwrap();
                self.pieces.get_mut(i).unwrap().perform_ply(ply);
            },
            Ply::Capture { old_position, new_position, captured_piece } => {
                match self.find_piece_at_position(new_position) {
                    None => panic!("No piece at position {:?}", new_position),
                    Some(i) => {
                        if self.pieces.get(i).unwrap().piece_type != captured_piece {
                            panic!("Piece type in ply does not match type in vector");                            
                        } else {
                            let _ = self.pieces.remove(i);
                        }
                    }
                }
                let i = self.find_piece_at_position(old_position.clone()).unwrap();
                self.pieces.get_mut(i).unwrap().perform_ply(ply);
            }
        }
        match self.turn {
            Color::BLACK => self.turn = Color::WHITE,
            Color::WHITE => self.turn = Color::BLACK,
        }
    }
    pub fn get_matrix_board_repr(&self) -> [[Option<PieceRepresentation>; 8]; 8] {
        let mut repr: [[Option<PieceRepresentation>; 8]; 8] = [[None; 8]; 8];
        for p in self.pieces.clone() {
            repr[(8-p.position.rank) as usize][(p.position.file as i8 -1) as usize] = Some(PieceRepresentation::new(p.color.clone(), p.piece_type.clone()));
        }
        return repr;
    }
    pub fn is_check(&self) -> bool {
        for p in self.pieces.clone() {
            for ply in p.find_plies(self) {
                match ply {
                    Ply::Capture { old_position: _, new_position: _, captured_piece } => {
                        if captured_piece == PieceType::KING {
                            return true;
                        }
                    },
                    _ => (),
                }
            }
        }
        return false;
    }
    pub fn is_checkmate(&self) -> bool {
        if !self.is_check() {
            return false;
        }
        for p in self.pieces.iter().filter(|x| x.piece_type == PieceType::KING).next().unwrap().find_plies(self) {
            let mut new_game = self.clone();
            new_game.turn = new_game.turn.invert();
            new_game.perform_ply(p);
            if !new_game.is_check() {
                return false;
            }
        }
        return true;
    }
    pub fn print_board(&self) {
        println!("  a b c d e f g h");
        let mut is_black: bool = false;
        for i in (1..=8).rev() {
            print!("{i} ");
            for j in [File::A, File::B, File::C, File::D, File::E, File::F, File::G, File::H] {
                let c: &str;
                if self.is_dark {
                    match self.find_piece_at_position(Position::new(j, i as i8)) {
                        Some(idx) => {
                            let p: &Piece = self.pieces.get(idx).unwrap();
                            c = match p.color {
                                Color::BLACK => piece_repr::match_thin(p.piece_type),
                                Color::WHITE=> piece_repr::match_fat(p.piece_type),
                            }
                        },
                        None => c = if is_black  {" "} else {"⠿"},
                    }
                } else {
                    match self.find_piece_at_position(Position::new(j, i as i8)) {
                        Some(idx) => {
                            let p: &Piece = self.pieces.get(idx).unwrap();
                            c = match p.color {
                                Color::BLACK => piece_repr::match_fat(p.piece_type),
                                Color::WHITE=> piece_repr::match_thin(p.piece_type),
                            }
                        },
                        None => c = if is_black  {" "} else {"⠿"},
                    }
                }
                print!("{c} ");
                is_black = !is_black;
            }
            print!("\n");
            // First square on new rank is the same as the last on the previous rank
            is_black = !is_black;
        } 
    }
    pub fn print_board_ply(&self, plies: Vec<Ply>) {
        println!("  a b c d e f g h");
        let mut is_black: bool = false;
        for i in (1..=8).rev() {
            print!("{i} ");
            for j in [File::A, File::B, File::C, File::D, File::E, File::F, File::G, File::H] {
                let mut c: &str;
                if self.is_dark {
                    match self.find_piece_at_position(Position::new(j, i as i8)) {
                        Some(idx) => {
                            let p: &Piece = self.pieces.get(idx).unwrap();
                            c = match p.color {
                                Color::BLACK => piece_repr::match_thin(p.piece_type),
                                Color::WHITE=> piece_repr::match_fat(p.piece_type),
                            };
                        },
                        None => c = if is_black  {" "} else {"⠿"},
                    }
                } else {
                    match self.find_piece_at_position(Position::new(j, i as i8)) {
                        Some(idx) => {
                            let p: &Piece = self.pieces.get(idx).unwrap();
                            c = match p.color {
                                Color::BLACK => piece_repr::match_fat(p.piece_type),
                                Color::WHITE=> piece_repr::match_thin(p.piece_type),
                            };
                        },
                        None => c = if is_black  {" "} else {"⠿"},
                    }
                }
                for pl in plies.clone() {
                    match pl {
                        Ply::Quiet { old_position: _, new_position } => {
                            if new_position == Position::new(j, i as i8) {
                                c = ".";
                                break;
                            }
                        }
                        _ => (),
                    }
                }
                print!("{c} ");
                is_black = !is_black;
            }
            print!("\n");
            // First square on new rank is the same as the last on the previous rank
            is_black = !is_black;
        } 
    }
}











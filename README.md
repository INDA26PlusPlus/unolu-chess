# Tjack

Om du vill testa att spela schack är det bara att flytta `src/_main.rs` till `src/main.rs`

Positioner representeras som `Position` struct som innehåller `file` och `rank` av typerna `File` respektive `Rank`.
`File` är en enum.
`Rank` är en typedef:ad i8.

* För att använda tjack måste du instansiera `Game` struct:en.
* För att veta vems tur det är anropar du metoden `whose_turn()`. Den returnerar `Color` vilket kan antingen vara `Color::BLACK` eller `Color::WHITE`.
* För att få en brädrepresentation (8x8 matris där a8 motsvarar [0][0]) anropar du metoden `get_matrix_board_repr()`,
 den returnerar en 8x8 matris av typen `Option<PieceRepresentation>` där `PieceRepresentation` är en struct med medlemmarna `piece_type` och `color` som är av typerna `PieceType` respektive `Color`.
* För att hitta drag för en position anropar du metoden `find_plies( Position )`. Du får endast tillbaka drag om det är den färgens tur.
* För att utföra ett drag anropar du metoden `perform_ply( Ply )`
* För att kolla om det är schack anropar du metoden `is_check()`
* För att kolla om det är schackmatt anropar du metoden `is_checkmate()`
* Om du vill skriva ut brädet kan du anropa antingen metoden `print_board()` eller `print_boar_ply( Ply )`.
Det senare alternativet visar tysta drag.
* Om vill skriva ut och din terminal har mörk bakgrund bör du anropa metoden `dark_mode(true)` om du annarts har ljus bakgrund bör du anropa `dark_mode(false)`. 



### Typer

```rust
pub enum PieceType {
    PAWN = 0,
    ROOK = 1,
    KNIGHT = 2,
    BISHOP = 3,
    QUEEN = 4,
    KING = 5,
}

pub enum Color {
    BLACK = 0,
    WHITE = 1,
}

pub struct PieceRepresentation {
    color: Color,
    piece_type: PieceType,
}

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

pub type Rank = i8;
```

### Signaturer (Game)
```rust
pub fn new() -> Game
pub fn find_plies(self: &mut Self, position: Position) -> Option<Vec<Ply>>
pub fn perform_ply(self: &mut Self, ply: Ply)
pub fn get_matrix_board_repr(&self) -> [[Option<PieceRepresentation>; 8]; 8]
pub fn is_check(&self) -> bool
pub fn is_checkmate(&self) -> bool
pub fn print_board(&self)
pub fn print_board_ply(&self, plies: Vec<Ply>)
pub fn dark_mode(&mut self, x: bool)
```


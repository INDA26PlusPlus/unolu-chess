use std::io::{self, Write};

use tjack::*;

fn main() {
    let mut g = Game::new();
    g.print_board();
    for _ in 0..10 {
        do_ply(&mut g);
    }


}

fn do_ply(g: &mut Game) {
    let x = match g.whose_turn() {
        Color::WHITE => "white's",
        Color::BLACK => "black's",
    };
    println!("It is {x} turn");
    print!("Select piece (FR): ");
    io::stdout().flush().unwrap();
    let mut s: String = String::new();
    let _ = io::stdin().read_line(&mut s);
    println!();
    let mut x = s.chars();
    let f1 = match x.next().unwrap() {
        'a' => File::A,
        'b' => File::B,
        'c' => File::C,
        'd' => File::D,
        'e' => File::E,
        'f' => File::F,
        'g' => File::G,
        'h' => File::H,
        _ => panic!(),
    }; 

    let r1 = match x.next().unwrap() {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        _ => panic!(),
    }; 

    let plies: Vec<Ply> = g.find_plies(Position::new(f1, r1)).unwrap();
    println!("{:?}", plies);
    g.print_board_ply(plies.clone());
    print!("Select ply (FR): ");
    io::stdout().flush().unwrap();
    s.clear();
    let _ = io::stdin().read_line(&mut s);
    let _ = s.pop();
    let mut x = s.chars();
    let f2 = match x.next().unwrap() {
        'a' => File::A,
        'b' => File::B,
        'c' => File::C,
        'd' => File::D,
        'e' => File::E,
        'f' => File::F,
        'g' => File::G,
        'h' => File::H,
        _ => panic!(),
    }; 

    let r2 = match x.next().unwrap() {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        _ => panic!(),
    }; 
    let mut p: Ply = Ply::Quiet { old_position: Position::new(f2, r2), new_position: Position::new(f2, r2) };
    let mut valid = false;
    for pl in plies {
        match pl {
            Ply::Quiet { old_position, new_position } => {
                if new_position == Position::new(f2,r2) {
                    p = pl;
                    valid = true;
                    break;
                }
                
            }
            Ply::Capture { old_position, new_position, captured_piece} => {
                if new_position == Position::new(f2,r2) {
                    p = pl;
                    valid = true;
                    break;
                }
                
            }
        }
    }
    if !valid {
        panic!("INVALID PLY!");
    }
    g.perform_ply(p.clone());
    g.print_board();
    if g.is_check() {
        if g.is_checkmate() {
            println!("Checkmate!!!!");
            panic!();
        } else {
            println!("Check!!");
        }
    }
}

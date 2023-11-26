use std::{io, time::Instant};

use crate::taquin::{Taquin, ai::astar};

mod taquin;
mod test;


fn main() {
    
    let size = 3;

    let t = Taquin::new(size);
    let mut t_to_solv = Taquin::new_rand(size);
    t.show();
    println!("=================================");
    t_to_solv.show();

    // Display menu to choose whether to solve automatically or manually
    println!("Choose how to solve the Taquin:");
    println!("1. Solve automatically with AI");
    println!("2. Solve manually");

    let mut solve_choice = String::new();
    io::stdin().read_line(&mut solve_choice).expect("Failed to read line");

    if solve_choice.trim() == "1" {
        // Solve automatically with AI
        // Display mth AI : you can then chose the oose the heuristic for AI solving:");
        println!("1. Manhattan heuristic");
        println!("2. Hamming heuristic");

        let mut heuristic_choice = String::new();
        io::stdin().read_line(&mut heuristic_choice).expect("Failed to read line");

        let mouv_resolv;
        let now = Instant::now();
        if heuristic_choice.trim() == "1" {
            mouv_resolv = astar(&t_to_solv,&t, &Taquin::heuristic_manhattan);
        } else if heuristic_choice.trim() == "2" {
            mouv_resolv = astar(&t_to_solv,&t, &Taquin::heuristic_hamming);
        } else {
            println!("Invalid choice. Defaulting to Manhattan heuristic.");
            mouv_resolv = astar(&t_to_solv,&t, &Taquin::heuristic_manhattan);
        }

        let time = now.elapsed();
        println!("Move list : {:?}",mouv_resolv);
        println!("Solved in {} moves and {:.3?}", mouv_resolv.len(), time);

    } else if solve_choice.trim() == "2" {
        // Solve manually by hand

        let mut i = 0;
        let now = Instant::now();
        while t_to_solv != t {
            // Get user input for move
            let available_moves = t_to_solv.available_move();

            println!("Enter a move {:?}:",available_moves);
            let mut move_input = String::new();
            io::stdin().read_line(&mut move_input).expect("Failed to read line");

            let res = match move_input.trim().to_lowercase().as_str() {
                "up"|"u" => t_to_solv.make_move(taquin::Move::Up),
                "down"|"d" => t_to_solv.make_move(taquin::Move::Down),
                "left"|"l" => t_to_solv.make_move(taquin::Move::Left),
                "right"|"r" => t_to_solv.make_move(taquin::Move::Right),
                _ => false,
            };

            if !res {
                println!("Invalid move. Try again.");
            } else {
                i += 1;
                t_to_solv.show();
            }
        }
        let time = now.elapsed();
        println!("Solved in {} moves and {:.3?}", i, time);
    } else {
        println!("Invalid choice. Exiting.");
    }
}

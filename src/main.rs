use std::{io, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, time::Instant};

use crate::taquin::{Taquin, ai::astar};

mod taquin;



fn main() {

    let t = Taquin::new(3);
    t.show();
    println!("=================================");
    let mut t2 = Taquin::new_rand(3);

    t2.show();

    let mouv_resolv;
    let now = Instant::now();
    {
        // With manhattan heuristic
        mouv_resolv = astar(&t2, &t, &Taquin::heuristic_manhattan);
        
        // With hamming heuristic
        // mouv_resolv = astar(&t2, &t, &Taquin::heuristic_hamming);
    }
    let time = now.elapsed();

    println!("Résolue en {} coups et en {:.3?}",mouv_resolv.len(),time);
    
    
    
    
    // while t2 != t {
    //     // let mut s = String::new();

    //     // io::stdin()
    //     //     .read_line(&mut s)
    //     //     .expect("erreur");

    //     println!("Coup jouer : {:?}",v_test[i]);

    //     t2.make_move(v_test[i]);

    //     // let res = match s.trim() {
    //     //     "Up"|"up"|"u" => t2.make_move(taquin::Move::Up),
    //     //     "Down"|"down"|"d" => t2.make_move(taquin::Move::Down),
    //     //     "Left"|"left"|"l" => t2.make_move(taquin::Move::Left),
    //     //     "Right"|"right"|"r" => t2.make_move(taquin::Move::Right),
    //     //     _ => false,
    //     // };
    //     i +=1;
    //     t2.show();
    // }


}





#[cfg(test)]

#[test]
fn node_succesors_test() {
    use std::{cell::RefCell, rc::Rc};

    use crate::taquin::{ai::Node,Move, Taquin};

    let taquin = Taquin::new(3);
    let mut taquin1 = Taquin::new(3);
    taquin1.make_move(Move::Left);
    let mut taquin2 = Taquin::new(3);
    taquin2.make_move(Move::Up);

    let n0 = Node::new(taquin,0,0,None,None);

    let children = n0.succesors();

    let n0_ref = Rc::new(RefCell::new(n0));

    let n1 = Node::new(taquin1,0,0,Some(Move::Left),Some(Rc::clone(&n0_ref)));
    let n2 = Node::new(taquin2,0,0,Some(Move::Up),Some(Rc::clone(&n0_ref)));
    
    assert_eq!(children.len(), 2,"Find 2 children");
    
    assert!(children.contains(&n1) && children.contains(&n2),"Children for Up, One for Left");

} 


#[test]
fn node_reconstruct_test() {
    use std::{cell::RefCell, rc::Rc};
    use crate::taquin::{ai::Node,Move, Taquin};

    let dummy_vec = vec![Move::Up,Move::Left,Move::Down,Move::Left,Move::Up,Move::Up];

    let mut taquin1 = Taquin::new(3);

    let mut node = Node::new(taquin1.clone(), 0, 0, None, None);
    assert_eq!(Node::reconstruct_moves(&node).is_empty(),true,"root only tree");

    for m in &dummy_vec {
        assert!(taquin1.make_move(*m) == true, "make move");

        node = Node::new(taquin1.clone(), 0, 0, Some(*m), Some(Rc::new(RefCell::new(node))));
        
    }


    assert_eq!(dummy_vec,Node::reconstruct_moves(&node),"reconstruction");

}

#[test]
fn astar_test() {
    use crate::taquin::{ai::astar, Move, Taquin};
    
    let mut taquin1 = Taquin::new(3);
    let taquin2 = Taquin::new(3);
    taquin1.make_move(Move::Left);

    let moves = astar(&taquin1, &taquin2, &Taquin::heuristic_hamming);

    assert_eq!(moves,vec![Move::Right],"1 move");
    
    taquin1.make_move(Move::Left);
    let moves = astar(&taquin1, &taquin2, &Taquin::heuristic_hamming);
    
    assert_eq!(moves,vec![Move::Right,Move::Right],"2 move");
    
    taquin1.make_move(Move::Up);
    let moves = astar(&taquin1, &taquin2, &Taquin::heuristic_hamming);
    
    assert_eq!(moves,vec![Move::Down,Move::Right,Move::Right],"3 move");
    
    taquin1.make_move(Move::Up);
    let moves = astar(&taquin1, &taquin2, &Taquin::heuristic_hamming);
    
    assert_eq!(moves,vec![Move::Down,Move::Down,Move::Right,Move::Right],"4 move");

    taquin1.make_move(Move::Right);
    let moves = astar(&taquin1, &taquin2, &Taquin::heuristic_hamming);
    
    assert_eq!(moves,vec![Move::Left,Move::Down,Move::Down,Move::Right,Move::Right],"5 move");
    
    taquin1.make_move(Move::Right);
    let moves = astar(&taquin1, &taquin2, &Taquin::heuristic_hamming);
    
    assert_eq!(moves,vec![Move::Left,Move::Left,Move::Down,Move::Down,Move::Right,Move::Right],"6 move");
}
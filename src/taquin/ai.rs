use std::borrow::BorrowMut;
use std::collections::hash_map::DefaultHasher;
use std::{collections::HashSet};
use std::hash::{Hash, Hasher};

use priority_queue::PriorityQueue;

use super::{Taquin, Move};


#[derive(Eq, Debug, Clone)]
pub struct Node {
    taquin: Taquin,
    cost: u32,
    heuristic: u32,

    move_done: Option<Move>,

}

impl Hash for Node {
    fn hash<H: Hasher>(&self,state: &mut H) {
        self.taquin.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.taquin == other.taquin
    }
}

impl Node {
    
    pub fn new(t:Taquin, c:u32, h:u32, m: Option<Move>) -> Node {
        Node {
            taquin: t,
            cost: c,
            heuristic: h,
            move_done: m,
        }
    }

    pub fn get_heuristic(&self) -> u32 {
        self.heuristic
    }
    
    pub fn get_cost(&self) -> u32 {
        self.cost
    }

    pub fn succesors(&self) -> Vec<Node> {
        let mut res : Vec<Node> = vec![];
        let available_moves = self.taquin.available_move();

        for m in available_moves {
            let mut new_taquin = self.taquin.clone();

            new_taquin.make_move(m);

            let child_node = Node {
                taquin: new_taquin,
                cost: self.get_cost(),
                heuristic: 0,
                move_done: Some(m),
            };

            res.push(child_node);
        }

        return res;
    }

    pub fn reconstruct_moves(self, all_nodes: HashSet<Node>) -> Vec<Move> {
        let mut res = vec![];

        let mut node = self.clone();
        
        while node.move_done.is_some() {
            let m = node.move_done.unwrap();
            res.push(m);

            node.taquin.reverse_move(m);

            // println!("{:?}",m);
        
            node = all_nodes.get(&node).expect("Unable to reconstuct the moves").clone();
        }

        res.reverse();
        return res;
    }

}


pub fn astar(start:&Taquin, end:&Taquin, heuristic_funct: &dyn Fn(&Taquin,&Taquin) -> u32) -> Vec<Move> {

    let mut visited_nodes: HashSet<Node> = HashSet::new();

    let mut open_list: PriorityQueue<Node, u32> = PriorityQueue::new();
    let node_0 = Node::new(start.clone(),0,0,Option::None);
    let h_0 = node_0.get_heuristic();

    open_list.push(node_0,h_0);
    

    while !open_list.is_empty() {

        let (current_node,_current_heuristic) = open_list.pop().unwrap();

        
        if current_node.taquin == *end {
            return current_node.reconstruct_moves(visited_nodes);
        }

        for mut child in current_node.succesors() {
            if !(visited_nodes.contains(&child) || open_list.get(&child).is_some_and(|(existing_node,_)| existing_node.get_cost() < child.get_cost())) {
                child.cost = current_node.cost + 1;
                let child_heur = child.cost + heuristic_funct(&child.taquin,&end);
                child.heuristic = child_heur;
                open_list.push(child, child_heur);
            }
        }

        visited_nodes.insert(current_node);

    }

    panic!("Unable to resolve the game")
}
use std::cell::RefCell;
use std::cmp::Reverse;
use std::rc::Rc;
use std::{collections::HashSet};
use std::hash::{Hash, Hasher};

use priority_queue::PriorityQueue;

use super::{Taquin, Move};


#[derive(Debug, Clone)]
pub struct Node {
    taquin: Taquin,
    cost: u32,
    heuristic: u32,

    // Move made on the parent node to get this node
    move_done: Option<Move>,

    // Reference to the parent node
    parent_node: Option<Rc<RefCell<Node>>>,

}

impl Eq for Node {}

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
    
    pub fn new (t:Taquin, c:u32, h:u32, m: Option<Move>, parent: Option<Rc<RefCell<Node>>>) -> Node {


        Node {
            taquin: t,
            cost: c,
            heuristic: h,
            move_done: m,
            parent_node: parent,
        }
    }
    
    pub fn show_taquin(&self) {
        self.taquin.show();
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

        let self_ref = Rc::new(RefCell::new(self.clone()));

        for m in available_moves {
            let mut new_taquin = self.taquin.clone();

            new_taquin.make_move(m);

            let child_node = Node::new(new_taquin, self.get_cost(), 0, Some(m),Some(Rc::clone(&self_ref)));

            res.push(child_node);
        }

        return res;
    }

    pub fn reconstruct_moves (&self) -> Vec<Move> {
        let mut res = vec![];

        let mut node = self.clone();

        while node.parent_node.is_some() {
            res.push(node.move_done.unwrap());

            node = node.parent_node.unwrap().borrow().to_owned();
        }

        res.reverse();
        return res;
    }

}


pub fn astar(start:&Taquin, end:&Taquin, heuristic_funct: &dyn Fn(&Taquin,&Taquin) -> u32) -> Vec<Move> {

    let mut visited_nodes: HashSet<Node> = HashSet::new();

    let mut nodes_to_see: PriorityQueue<Node, Reverse<u32>> = PriorityQueue::new();

    let root = Node::new(start.clone(),0,heuristic_funct(&start,&end),None,None);

    let root_heur = root.get_heuristic();
    println!("starting heuristic {}",root_heur);

    nodes_to_see.push(root.clone(),Reverse(root_heur));
    

    while !nodes_to_see.is_empty() {
        let (current_node,_current_heuristic) = nodes_to_see.pop().unwrap();

        if current_node.taquin == *end {
            return current_node.reconstruct_moves();
        }
        
        for mut child in current_node.succesors() {

            if !(visited_nodes.contains(&child) || nodes_to_see.get(&child).is_some()) || nodes_to_see.get(&child).is_some_and(|(existing_node,_)| existing_node.get_cost() + 1 > child.get_cost()) {
                child.cost = current_node.cost + 1;
                
                let new_child_heur = child.cost + heuristic_funct(&child.taquin,&end);
                child.heuristic = new_child_heur;

                nodes_to_see.push(child.clone(), Reverse(new_child_heur));
            }

        }

        visited_nodes.insert(current_node);

    }

    panic!("Unable to resolve the game");
}
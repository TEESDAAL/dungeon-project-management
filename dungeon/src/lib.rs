use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use std::collections::VecDeque;

pub const GRID_SIZE: usize = 10;
// Horrible solution for constant issue!!!!!!!
macro_rules! NUM_NODES {
    () => {
        f32::powf(GRID_SIZE as f32, 1.5).round() as usize
    };
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Tile {
    Empty,
    Enemy(Enemy),
    Treasure(Treasure),
}


#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Enemy {}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Treasure {}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Node {
    pub x: isize,
    pub y: isize,
    pub value: Tile,
    pub neighbors: Vec<usize>,
    pub index: usize,
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub current_player_position: Option<usize>,
    pub goal_position: Option<usize>,
}

impl Graph {
    pub fn new() -> Graph {
        let mut graph = Graph {
            nodes: Vec::new(),
            current_player_position: None,
            goal_position: None,
        };
        graph.create_nodes();
        graph.connect_nodes();
        graph.populate_board();
        graph
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, node_1: usize, node_2: usize) {
        self.nodes[node_1].neighbors.push(node_2);
        self.nodes[node_2].neighbors.push(node_1);
    }

    pub fn get_node(&mut self, x: isize, y: isize) -> Option<usize> {
        for (index, node) in self.nodes.iter().enumerate() {
            if node.x == x && node.y == y {
                return Some(index);
            }
        }
        return None;
    }

    pub fn closest_node(
        &self,
        node_indices: &Vec<usize>,
        current_node_index: &usize,
    ) -> Option<usize> {
        let current_node = &self.nodes[*current_node_index];
        let mut node_indices = node_indices.clone();
        node_indices.shuffle(&mut rand::thread_rng());
        let mut closest_node: Option<usize> = None;
        let mut closest_distance = f32::INFINITY as usize;
        for index in node_indices.iter() {
            let node = &self.nodes[*index];
            let distance = ((current_node.x as isize - node.x as isize).pow(2)
                + (current_node.y as isize - node.y as isize).pow(2))
                as usize;
            if distance < closest_distance {
                closest_distance = distance;
                closest_node = Some(*index);
            }
        }
        closest_node
    }

    fn create_nodes(&mut self) {
        let mut locations: HashSet<(isize, isize)> = HashSet::new();
        while self.nodes.len() < NUM_NODES!() {
            let (x, y) = (
                rand::thread_rng().gen_range(0..GRID_SIZE) as isize,
                rand::thread_rng().gen_range(0..GRID_SIZE) as isize,
            );
            if locations.insert((x, y)) {
                self.add_node(Node {
                    x: x,
                    y: y,
                    neighbors: vec![],
                    value: Tile::Empty,
                    index: self.nodes.len(),
                });
            }
        }
    }

    fn connect_nodes(&mut self) {
        let mut unconnected_nodes: Vec<usize> = (0..self.nodes.len()).collect();
        let mut visited_nodes: Vec<usize> = Vec::new();
        visited_nodes.push(match unconnected_nodes.pop() {
            Some(index) => index,
            None => return,
        });
        while unconnected_nodes.len() > 0 {
            let mut closest_distance = f32::INFINITY as usize;
            let mut current_closest_node_pair: Option<(usize, usize)> = None;

            visited_nodes.shuffle(&mut rand::thread_rng());
            for node in &visited_nodes {
                let closest_node = self.closest_node(&unconnected_nodes, &node).unwrap();
                let distance = ((self.nodes[*node].x - self.nodes[closest_node].x).pow(2)
                    + (self.nodes[*node].y - self.nodes[closest_node].y).pow(2))
                    as usize;
                if distance < closest_distance {
                    closest_distance = distance;
                    current_closest_node_pair = Some((*node, closest_node));
                }
            }

            match current_closest_node_pair {
                Some((visited_node, non_visited_node)) => {
                    self.add_edge(visited_node, non_visited_node);
                    visited_nodes.push(non_visited_node);
                    unconnected_nodes.retain(|index| *index != non_visited_node);
                }
                None => return,
            }
        }
    }

    fn populate_board(&mut self) {
        let mut unpopulated_nodes: Vec<usize> = (0..self.nodes.len()).collect();
        unpopulated_nodes.shuffle(&mut rand::thread_rng());

        let player_index = unpopulated_nodes.pop().unwrap();
        self.current_player_position = Some(player_index);
        // self.nodes[player_index].value = Tile::Player;

        let mut goal_index = unpopulated_nodes.pop().unwrap();
        if goal_index == player_index{
            println!("How?");
            goal_index = unpopulated_nodes.pop().unwrap();
        }

        self.goal_position = Some(goal_index);
        // self.nodes[goal_index].value = Tile::Goal;
    }

    pub fn get_path(&self, start_node: usize, end_node: usize) -> Vec<usize> {
        // Open source code, written by Benjy under the MIT license.
        let mut parents: Vec<Option<usize>> = vec![None; self.nodes.len()];
        let mut nodes_to_visit: VecDeque<(usize, usize)> = VecDeque::new();
        nodes_to_visit.push_back((start_node, start_node));
        let mut visited_nodes: Vec<bool> = vec![false; self.nodes.len()];
        while let Some((parent, node)) = nodes_to_visit.pop_front() {
            visited_nodes[node] = true;
            parents[node] = Some(parent);
            if node == end_node {
                break;
            }
            for neighbor in &self.nodes[node].neighbors {
                if !visited_nodes[*neighbor] {
                    nodes_to_visit.push_back((node, *neighbor));
                }
            }
        }
        let mut path: Vec<usize> = Vec::new();
        let mut current_node = end_node;
        loop {
            path.push(current_node);
            if current_node == start_node {
                break;
            }
            current_node = parents[current_node].unwrap();
        }
        path
    }
}

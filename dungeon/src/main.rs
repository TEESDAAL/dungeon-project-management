use ::rand::seq::SliceRandom;
use ::rand::Rng;
use macroquad::prelude::*;
use std::collections::HashSet;

const NODE_SIZE: f32 = 20.;
const EDGE_SIZE: f32 = 2.;

const GRID_SIZE: usize = 10;
// Horrible solution for constant issue!!!!!!!
macro_rules! NUM_NODES {
    () => {
        f32::powf(GRID_SIZE as f32, 1.5).round() as usize
    };
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Tile {
    contains_player: bool,
    contains_goal: bool,
    enemy: Option<Enemy>,
    treasure: Option<Treasure>,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Enemy {}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Treasure {}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Node {
    x: isize,
    y: isize,
    value: Tile,
    neighbors: Vec<Node>,
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, node_1: usize, node_2: usize) {
        self.edges.push((node_1, node_2));
    }
    fn get_node(&mut self, x: isize, y: isize) -> Option<usize> {
        for (index, node) in self.nodes.iter().enumerate() {
            if node.x == x && node.y == y {
                return Some(index);
            }
        }
        return None;
    }

    fn closest_node(&self, node_indices: &Vec<usize>, current_node_index: &usize) -> Option<usize> {
        let current_node = &self.nodes[*current_node_index];
        let mut closest_node: Option<usize> = None;
        let mut closest_distance = f32::INFINITY as usize;
        for index in node_indices {
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
                ::rand::thread_rng().gen_range(0..GRID_SIZE) as isize,
                ::rand::thread_rng().gen_range(0..GRID_SIZE) as isize,
            );
            if locations.insert((x, y)) {
                self.add_node(Node {
                    x: x,
                    y: y,
                    neighbors: vec![],
                    value: Tile {
                        contains_player: false,
                        contains_goal: false,
                        enemy: None,
                        treasure: None,
                    },
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
        unpopulated_nodes.shuffle(&mut ::rand::thread_rng());

        let player_index = unpopulated_nodes.pop().unwrap();
        self.nodes[player_index].value.contains_player = true;

        let goal_index = unpopulated_nodes.pop().unwrap();
        self.nodes[goal_index].value.contains_goal = true;
    }
}

fn draw_graph(graph: &Graph) {
    let multiplier = screen_height() / GRID_SIZE as f32;
    for node in &graph.nodes {
        let color = if node.value.contains_player {
            RED
        } else if node.value.contains_goal {
            GREEN
        } else {
            BLUE
        };

        draw_circle(
            node.x as f32 * multiplier + NODE_SIZE,
            node.y as f32 * multiplier + NODE_SIZE,
            NODE_SIZE,
            color,
        );
    }

    for (node_index_1, node_index_2) in &graph.edges {
        let node_1 = &graph.nodes[*node_index_1];
        let node_2 = &graph.nodes[*node_index_2];
        draw_line(
            node_1.x as f32 * multiplier + NODE_SIZE,
            node_1.y as f32 * multiplier + NODE_SIZE,
            node_2.x as f32 * multiplier + NODE_SIZE,
            node_2.y as f32 * multiplier + NODE_SIZE,
            EDGE_SIZE,
            BLACK,
        )
    }
}

fn keyboard_actions(graph: &mut Graph) {
    if is_key_down(KeyCode::R) {
        graph.edges = Vec::new();
        graph.nodes = Vec::new();
        graph.create_nodes();
        graph.connect_nodes();
        graph.populate_board();
    }
}

fn mouse_events(graph: &mut Graph) {
    if is_mouse_button_down(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        let multiplier = screen_height() / GRID_SIZE as f32;
        let (x, y) = (
            ((mouse_x) / multiplier) as isize,
            ((mouse_y) / multiplier) as isize,
        );
        if let Some(node) = graph.get_node(x, y) {
            for node in &mut graph.nodes {
                node.value.contains_player = false;
            }
            graph.nodes[node].value.contains_player = true;
        }
    }
}

#[macroquad::main("MapMaker")]
async fn main() {
    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };
    graph.create_nodes();
    graph.connect_nodes();
    graph.populate_board();
    loop {
        keyboard_actions(&mut graph);
        mouse_events(&mut graph);
        clear_background(WHITE);
        draw_graph(&graph);
        next_frame().await
    }
}

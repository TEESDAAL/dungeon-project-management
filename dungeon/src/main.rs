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
struct Tile {}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Node {
    x: usize,
    y: usize,
    value: Tile,
    neighbors: Vec<Node>,
}

fn closest_node(nodes: &HashSet<Node>, current_node: &Node) -> Option<Node> {
    let mut closest_node: Option<Node> = None;
    let mut closest_distance = f32::INFINITY as usize;
    for node in nodes {
        let distance = ((current_node.x as isize - node.x as isize).pow(2)
            + (current_node.y as isize - node.y as isize).pow(2)) as usize;
        if distance < closest_distance {
            closest_distance = distance;
            closest_node = Some(node.clone());
        }
    }
    closest_node
}

struct Graph {
    nodes: HashSet<Node>,
    edges: HashSet<(Node, Node)>,
}

impl Graph {
    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node);
    }

    fn add_edge(&mut self, node_1: Node, node_2: Node) {
        self.edges.insert((node_1, node_2));
    }
    fn _get_node(&self, x: usize, y: usize) -> Option<Node> {
        for node in &self.nodes {
            if node.x == x && node.y == y {
                return Some(node.clone());
            }
        }
        return None;
    }

    fn create_nodes(&mut self) {
        let mut locations: HashSet<(usize, usize)> = HashSet::new();
        while self.nodes.len() < NUM_NODES!() {
            let (x, y) = (
                ::rand::thread_rng().gen_range(0..GRID_SIZE),
                ::rand::thread_rng().gen_range(0..GRID_SIZE),
            );
            if locations.insert((x, y)) {
                self.add_node(Node {
                    x: x,
                    y: y,
                    neighbors: vec![],
                    value: Tile {},
                });
            }
        }
    }

    fn connect_nodes(&mut self) {
        let mut unconnected_nodes = self.nodes.clone();
        let mut visited_nodes: HashSet<Node> = HashSet::new();
        let beginning_node = match unconnected_nodes.iter().next() {
            Some(node) => node.clone(),
            None => return,
        };
        unconnected_nodes.remove(&beginning_node);
        visited_nodes.insert(beginning_node);

        while unconnected_nodes.len() > 0 {
            let mut closest_distance = f32::INFINITY as usize;
            let mut current_closest_node_pair: Option<(Node, Node)> = None;

            for visited_node in &visited_nodes {
                let closest = closest_node(&unconnected_nodes, &visited_node).unwrap();
                let distance = ((closest.x as isize - visited_node.x as isize).pow(2)
                    + (closest.y as isize - visited_node.y as isize).pow(2))
                    as usize;
                if distance < closest_distance {
                    closest_distance = distance;
                    current_closest_node_pair = Some((visited_node.clone(), closest));
                }
            }
            match current_closest_node_pair {
                None => return,
                Some(node_pair) => {
                    let (node_1, node_2) = node_pair;
                    visited_nodes.insert(unconnected_nodes.take(&node_2).unwrap());
                    self.add_edge(node_1, node_2);
                }
            }
        }
    }
}

fn draw_graph(graph: &Graph) {
    let multiplier = screen_height() / GRID_SIZE as f32;
    for node in &graph.nodes {
        // println!("Drawing node at {}, {}", node.x, node.y
        draw_circle(
            node.x as f32 * multiplier + NODE_SIZE,
            node.y as f32 * multiplier + NODE_SIZE,
            NODE_SIZE,
            BLUE,
        )
    }

    for (node_1, node_2) in &graph.edges {
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
        graph.edges = HashSet::new();
        graph.nodes = HashSet::new();
        graph.create_nodes();
        graph.connect_nodes();
    }
}

#[macroquad::main("MapMaker")]
async fn main() {
    let mut graph = Graph {
        nodes: HashSet::new(),
        edges: HashSet::new(),
    };
    graph.create_nodes();
    graph.connect_nodes();
    loop {
        keyboard_actions(&mut graph);
        clear_background(WHITE);
        draw_graph(&graph);
        next_frame().await
    }
}

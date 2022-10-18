use ::rand::{seq::SliceRandom, Rng};
use lazy_static::{initialize, lazy_static};
use macroquad::prelude::*;
use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
};

pub const GRID_SIZE: usize = 6;
const NODE_SIZE: f32 = 100.;
const EDGE_SIZE: f32 = 10.;
const PLAYER_SIZE: f32 = NODE_SIZE / 1.5;
const ENEMY_SIZE: f32 = NODE_SIZE / 1.5;
const GOAL_SIZE: f32 = NODE_SIZE / 1.5;
const TREASURE_SIZE: f32 = NODE_SIZE / 1.5;

lazy_static! {
    pub static ref NUM_NODES: usize = (GRID_SIZE as f32).powf(1.5).round() as usize;
    pub static ref NUM_TREASURE: usize = max(1, (*NUM_NODES as f32 / 10.).round() as usize);
    pub static ref NUM_ENEMIES: usize = *NUM_NODES / 5;
    // pub static ref NUM_ENEMIES: usize = 0;
    pub static ref NODE_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/node.png"),
        Some(ImageFormat::Png),
    );
    pub static ref PLAYER_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/ferris-front.png"),
        Some(ImageFormat::Png),
    );
    pub static ref ENEMY_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/go-gopher.png"),
        Some(ImageFormat::Png)
    );
    pub static ref GOAL_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/logo.png"),
        Some(ImageFormat::Png),
    );
    pub static ref FOREST_BACKGROUND_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/forest.png"),
        Some(ImageFormat::Png),
    );
    pub static ref DESERT_BACKGROUND_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/desert.png"),
        Some(ImageFormat::Png),
    );
    pub static ref CAVERN_BACKGROUND_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/cavern.png"),
        Some(ImageFormat::Png),
    );
    pub static ref TREASURE_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/treasure.png"),
        Some(ImageFormat::Png),
    );
    pub static ref PLAYER_ARMOURED_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/armored_ferris.png"),
        Some(ImageFormat::Png)
    );
}

pub fn load_map_textures() {
    initialize(&NODE_TEXTURE);
    println!("Map Node Texture loaded");
    initialize(&PLAYER_TEXTURE);
    println!("Map player texture loaded");
    initialize(&PLAYER_ARMOURED_TEXTURE);
    println!("Map armoured player texture loaded");
    initialize(&ENEMY_TEXTURE);
    println!("Map enemy texture loaded");
    initialize(&NODE_TEXTURE);
    println!("Map node texture loaded");
    initialize(&GOAL_TEXTURE);
    println!("Map goal texture loaded");
    initialize(&TREASURE_TEXTURE);
    println!("Map treasure texture loaded");
    initialize(&FOREST_BACKGROUND_TEXTURE);
    println!("Background texture 1 loaded");
    initialize(&DESERT_BACKGROUND_TEXTURE);
    println!("Background texture 2 loaded");
    initialize(&CAVERN_BACKGROUND_TEXTURE);
    println!("Background texture 3 loaded");
}

#[derive(Clone, Copy)]
enum Sprite {
    Node,
    Player,
    PlayerArmoured,
    Enemy,
    Goal,
    Treasure,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Default)]
pub enum Tile {
    #[default]
    Empty,
    Enemy(Enemy),
    Treasure,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Enemy {}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Default)]
pub struct Node {
    pub x: isize,
    pub y: isize,
    pub value: Tile,
    pub neighbors: Vec<usize>,
    pub index: usize,
}

pub struct Graph {
    pub level: usize,
    pub nodes: Vec<Node>,
    pub current_player_position: Option<usize>,
    pub goal_position: Option<usize>,
    pub player_path: Vec<usize>,
    pub background_order: Vec<LevelInfo>,
}

pub struct LevelInfo {
    pub map_texture: Texture2D,
    pub ground_color: Color,
    pub sky_color: Color,
}

impl Sprite {
    fn draw(&self, x: f32, y: f32) {
        match self {
            Sprite::Enemy => {
                let enemy_shrink_factor = ENEMY_SIZE / ENEMY_TEXTURE.width();
                draw_texture_ex(
                    *ENEMY_TEXTURE,
                    x - ENEMY_TEXTURE.width() * enemy_shrink_factor / 2.,
                    y - ENEMY_TEXTURE.height() * enemy_shrink_factor / 2.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::from([
                            ENEMY_TEXTURE.width() * enemy_shrink_factor,
                            ENEMY_TEXTURE.height() * enemy_shrink_factor,
                        ])),
                        ..Default::default()
                    },
                );
            }
            Sprite::Player => {
                let player_shrink_factor = PLAYER_SIZE / PLAYER_TEXTURE.width();

                draw_texture_ex(
                    *PLAYER_TEXTURE,
                    x - PLAYER_TEXTURE.width() * player_shrink_factor / 2.,
                    y - PLAYER_TEXTURE.height() * player_shrink_factor / 2.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::from([
                            PLAYER_TEXTURE.width() * player_shrink_factor,
                            PLAYER_TEXTURE.height() * player_shrink_factor,
                        ])),
                        ..Default::default()
                    },
                );
            }
            Sprite::PlayerArmoured => {
                let player_shrink_factor = PLAYER_SIZE / PLAYER_ARMOURED_TEXTURE.width();

                draw_texture_ex(
                    *PLAYER_ARMOURED_TEXTURE,
                    x - PLAYER_ARMOURED_TEXTURE.width() * player_shrink_factor / 2.,
                    y - PLAYER_ARMOURED_TEXTURE.height() * player_shrink_factor / 2.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::from([
                            PLAYER_ARMOURED_TEXTURE.width() * player_shrink_factor,
                            PLAYER_ARMOURED_TEXTURE.height() * player_shrink_factor,
                        ])),
                        ..Default::default()
                    },
                );
            }
            Sprite::Goal => {
                let goal_shrink_factor = GOAL_SIZE / GOAL_TEXTURE.width();

                draw_texture_ex(
                    *GOAL_TEXTURE,
                    x - GOAL_TEXTURE.width() * goal_shrink_factor / 2.,
                    y - GOAL_TEXTURE.height() * goal_shrink_factor / 2.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::from([
                            GOAL_TEXTURE.width() * goal_shrink_factor,
                            GOAL_TEXTURE.height() * goal_shrink_factor,
                        ])),
                        ..Default::default()
                    },
                );
            }
            Sprite::Node => {
                let node_shrink_factor = NODE_SIZE / NODE_TEXTURE.width();
                draw_texture_ex(
                    *NODE_TEXTURE,
                    x - NODE_TEXTURE.width() * node_shrink_factor / 2.,
                    y - NODE_TEXTURE.height() * node_shrink_factor / 2.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::from([
                            NODE_TEXTURE.width() * node_shrink_factor,
                            NODE_TEXTURE.height() * node_shrink_factor,
                        ])),
                        ..Default::default()
                    },
                );
            }
            Sprite::Treasure => {
                let treasure_shrink_factor = TREASURE_SIZE / TREASURE_TEXTURE.width();
                draw_texture_ex(
                    *TREASURE_TEXTURE,
                    x - TREASURE_TEXTURE.width() * treasure_shrink_factor / 2.,
                    y - TREASURE_TEXTURE.height() * treasure_shrink_factor / 2.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::from([
                            TREASURE_TEXTURE.width() * treasure_shrink_factor,
                            TREASURE_TEXTURE.height() * treasure_shrink_factor,
                        ])),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

impl Graph {
    #[must_use]
    pub fn new() -> Graph {
        // Create a default graph then add the nodes, connect them and specialize them
        let mut graph = Graph {
            level: 0,
            nodes: Vec::new(),
            current_player_position: None,
            goal_position: None,
            player_path: Vec::new(),
            background_order: vec![
                LevelInfo {
                    map_texture: *FOREST_BACKGROUND_TEXTURE,
                    ground_color: GREEN,
                    sky_color: BLUE,
                },
                LevelInfo {
                    map_texture: *DESERT_BACKGROUND_TEXTURE,
                    ground_color: BEIGE,
                    sky_color: DARKBLUE,
                },
                LevelInfo {
                    map_texture: *CAVERN_BACKGROUND_TEXTURE,
                    ground_color: GRAY,
                    sky_color: BLACK,
                },
            ],
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
        // Loop through the nodes and return it if the x & y pos matches else return None
        for (index, node) in self.nodes.iter().enumerate() {
            if node.x == x && node.y == y {
                return Some(index);
            }
        }
        None
    }

    fn distance_between_nodes(node_1: &Node, node_2: &Node) -> f32 {
        ((node_1.x as f32 - node_2.x as f32).powf(2.)
            + (node_1.y as f32 - node_2.y as f32).powf(2.))
        .sqrt()
    }

    #[must_use]
    pub fn closest_node(
        &self,
        node_indices: &Vec<usize>,
        current_node_index: &usize,
    ) -> Option<usize> {
        // Converts the index into a reference to the current node
        let current_node = &self.nodes[*current_node_index];

        // Loop through the nodes and return the closest_node
        let mut closest_node: Option<usize> = None;
        let mut closest_distance = f32::INFINITY;

        for index in node_indices.iter() {
            let node = &self.nodes[*index];

            let distance = Self::distance_between_nodes(current_node, node);

            // Update the closest_node and distance if the
            // current node is closer to the node of interest
            if distance < closest_distance {
                closest_distance = distance;
                closest_node = Some(*index);
            }
        }
        closest_node
    }

    fn create_nodes(&mut self) {
        // Create a number of nodes equal to the NUM_NODES
        let mut locations: HashSet<(isize, isize)> = HashSet::new();
        while self.nodes.len() < *NUM_NODES {
            let (x, y) = (
                ::rand::thread_rng().gen_range(0..GRID_SIZE) as isize,
                ::rand::thread_rng().gen_range(0..GRID_SIZE) as isize,
            );
            // Only add the nodes if it is a unique node
            if locations.insert((x, y)) {
                self.add_node(Node {
                    x,
                    y,
                    index: self.nodes.len(),
                    ..Default::default()
                });
            }
        }
    }

    fn connect_nodes(&mut self) {
        // Connects the nodes via a minimum spanning tree
        let mut unconnected_nodes: Vec<usize> = (0..self.nodes.len()).collect();
        let mut visited_nodes: Vec<usize> = Vec::new();

        // Add an arbitrary node to act as the starting node
        visited_nodes.push(match unconnected_nodes.pop() {
            Some(index) => index,
            None => return,
        });

        while !unconnected_nodes.is_empty() {
            // Loop through all the nodes and link whichever node is closer
            let mut closest_distance = f32::INFINITY;
            let mut current_closest_node_pair: Option<(usize, usize)> = None;

            // Allows for more natural linking between nodes
            visited_nodes.shuffle(&mut ::rand::thread_rng());
            for node_index in &visited_nodes {
                let closest_index = self.closest_node(&unconnected_nodes, node_index).unwrap();
                let (closest_node, node) = (&self.nodes[closest_index], &self.nodes[*node_index]);

                let distance = Self::distance_between_nodes(closest_node, node);

                // Update the closest_distance if the current node is closer
                if distance < closest_distance {
                    closest_distance = distance;
                    current_closest_node_pair = Some((*node_index, closest_index));
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

    fn add_player(&mut self, unpopulated_nodes: &mut Vec<usize>) {
        for (i, index) in unpopulated_nodes.iter().enumerate() {
            if self.nodes[*index].neighbors.len() == 1 {
                self.current_player_position = Some(*index);
                unpopulated_nodes.remove(i);
                break;
            }
        }
        if self.current_player_position.is_none() {
            self.current_player_position = Some(unpopulated_nodes.pop().unwrap())
        }
    }
    fn add_goal(&mut self, unpopulated_nodes: &mut Vec<usize>) {
        for (i, index) in unpopulated_nodes.iter().enumerate() {
            if self.nodes[*index].neighbors.len() == 1 {
                self.goal_position = Some(*index);
                unpopulated_nodes.remove(i);
                break;
            }
        }
        if self.goal_position.is_none() {
            self.goal_position = Some(unpopulated_nodes.pop().unwrap())
        }
    }
    fn add_enemies(&mut self, unpopulated_nodes: &mut Vec<usize>) {
        let mut num_enemies = *NUM_ENEMIES;
        if num_enemies > 0 {
            let mut path = self.get_path(
                self.current_player_position.unwrap(),
                self.goal_position.unwrap(),
            );
            path.shuffle(&mut ::rand::thread_rng());
            for index in &path {
                if unpopulated_nodes.contains(index) {
                    num_enemies -= 1;
                    self.nodes[*index].value = Tile::Enemy(Enemy {});
                    break;
                }
            }
        }

        for _ in 0..num_enemies {
            self.nodes[unpopulated_nodes
                .pop()
                .expect("Ran out of nodes in enemies")]
            .value = Tile::Enemy(Enemy {});
        }
    }
    fn add_treasure(&mut self, unpopulated_nodes: &mut Vec<usize>) {
        let mut num_treasure = *NUM_TREASURE;
        for index in unpopulated_nodes.iter() {
            if self.nodes[*index].neighbors.len() == 1 {
                self.nodes[*index].value = Tile::Treasure;
                num_treasure -= 1;
                if num_treasure == 0 {
                    return;
                }
            }
        }
        for _ in 0..num_treasure {
            self.nodes[unpopulated_nodes.pop().unwrap()].value = Tile::Treasure;
        }
    }
    fn populate_board(&mut self) {
        let mut unpopulated_nodes: Vec<usize> = (0..self.nodes.len()).collect();
        unpopulated_nodes.shuffle(&mut ::rand::thread_rng());

        self.add_goal(&mut unpopulated_nodes);
        self.add_player(&mut unpopulated_nodes);
        self.add_treasure(&mut unpopulated_nodes);
        self.add_enemies(&mut unpopulated_nodes);
    }

    #[must_use]
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

    pub fn move_player(&mut self, index: usize) {
        self.nodes[self.current_player_position.unwrap()].value = Tile::Empty;

        if index == self.goal_position.unwrap() && self.player_path.last().is_none() {
            self.reload();
            return;
        }

        self.current_player_position = Some(index);
    }

    #[must_use]
    pub fn distance(&self, index_1: usize, index_2: usize) -> f32 {
        let (node_1, node_2) = (&self.nodes[index_1], &self.nodes[index_2]);
        (((node_1.x - node_2.x).pow(2) + (node_1.y - node_2.y).pow(2)) as f32).sqrt()
    }

    fn reload(&mut self) {
        *self = Graph::new();
    }
    fn draw_edges(&self) {
        let y_scalar = screen_height() / GRID_SIZE as f32;
        let x_scalar = screen_width() / GRID_SIZE as f32;
        for node in &self.nodes {
            for neighbor in &node.neighbors {
                draw_line(
                    node.x as f32 * x_scalar + NODE_SIZE / 2.0,
                    node.y as f32 * y_scalar + NODE_SIZE / 2.0,
                    self.nodes[*neighbor].x as f32 * x_scalar + NODE_SIZE / 2.0,
                    self.nodes[*neighbor].y as f32 * y_scalar + NODE_SIZE / 2.0,
                    EDGE_SIZE,
                    Color::from_rgba(0, 0, 0, 130),
                );
            }
        }
    }

    pub fn draw_terrain(&self, current_background: &usize) {
        let texture = self.background_order[*current_background].map_texture;
        let scalar = screen_width() / texture.width();
        draw_texture_ex(
            texture,
            screen_width() / 2. - texture.width() * scalar / 2.,
            screen_height() - texture.height() * scalar,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::from([
                    texture.width() * scalar,
                    texture.height() * scalar,
                ])),
                ..Default::default()
            },
        );
    }

    pub fn draw_graph(&self, armoured: &bool, current_background: &usize) {
        let y_scalar = screen_height() / GRID_SIZE as f32;
        let x_scalar = screen_width() / GRID_SIZE as f32;
        self.draw_terrain(current_background);
        self.draw_edges();
        for node in &self.nodes {
            let base_x = node.x as f32 * x_scalar + NODE_SIZE / 2.0;
            let base_y = node.y as f32 * y_scalar + NODE_SIZE / 2.0;

            Sprite::Node.draw(base_x, base_y);

            match node.value {
                Tile::Empty => (),
                Tile::Enemy(_) => Sprite::Enemy.draw(base_x, base_y),
                Tile::Treasure => Sprite::Treasure.draw(base_x, base_y),
            }

            if self.current_player_position.unwrap() == node.index {
                if *armoured {
                    Sprite::PlayerArmoured.draw(base_x, base_y);
                } else {
                    Sprite::Player.draw(base_x, base_y);
                }
            }

            if self.goal_position.unwrap() == node.index {
                Sprite::Goal.draw(base_x, base_y);
            }
        }
    }
}

pub fn keyboard_actions(graph: &mut Graph) {
    if is_key_down(KeyCode::R) {
        graph.reload();
    }
}

pub fn mouse_events(graph: &mut Graph) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let shrink_factor = NODE_SIZE / NODE_TEXTURE.width();

        let (mouse_x, mouse_y) = mouse_position();
        let y_scalar = screen_height() / GRID_SIZE as f32;
        let x_scalar = screen_width() / GRID_SIZE as f32;
        let (x, y) = (
            ((mouse_x - NODE_SIZE / 2.0 + NODE_TEXTURE.width() * shrink_factor / 2.) / x_scalar)
                .round() as isize,
            ((mouse_y - NODE_SIZE / 2.0 + NODE_TEXTURE.height() * shrink_factor / 2.) / y_scalar)
                .round() as isize,
        );
        if let Some(end_node) = graph.get_node(x, y) {
            graph.player_path = graph.get_path(graph.current_player_position.unwrap(), end_node);
        }
    }
}

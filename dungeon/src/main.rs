pub mod lib;
use crate::lib::*;
use macroquad::prelude::*;
use std::time::{Duration, Instant};
const NODE_SIZE: f32 = 20.;
const EDGE_SIZE: f32 = 2.;

fn draw_graph(graph: &Graph) {
    let multiplier = screen_height() / GRID_SIZE as f32;
    for node in &graph.nodes {
        let color = match &node.value {
            Tile::Empty => BLUE,
            Tile::Treasure(_) => GOLD,
            Tile::Enemy(_) => RED,
            Tile::Player => GREEN,
            Tile::Goal => PURPLE,
        };

        draw_circle(
            node.x as f32 * multiplier + NODE_SIZE,
            node.y as f32 * multiplier + NODE_SIZE,
            NODE_SIZE,
            color,
        );

        for neighbor in &node.neighbors {
            draw_line(
                node.x as f32 * multiplier + NODE_SIZE,
                node.y as f32 * multiplier + NODE_SIZE,
                graph.nodes[*neighbor].x as f32 * multiplier + NODE_SIZE,
                graph.nodes[*neighbor].y as f32 * multiplier + NODE_SIZE,
                EDGE_SIZE,
                BLACK,
            );
        }
    }
}

fn reload(graph: &mut Graph, path: &mut Vec<usize>) {
    *path = Vec::new();
    *graph = Graph::new();
}

fn keyboard_actions(graph: &mut Graph, path: &mut Vec<usize>) {
    if is_key_down(KeyCode::R) {
        reload(graph, path);
    }
}

fn mouse_events(graph: &mut Graph, path: &mut Vec<usize>) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        let multiplier = screen_height() / GRID_SIZE as f32;
        let (x, y) = (
            ((mouse_x - NODE_SIZE) / multiplier).round() as isize,
            ((mouse_y - NODE_SIZE) / multiplier).round() as isize,
        );
        if let Some(end_node) = graph.get_node(x, y) {
            *path = graph.get_path(graph.current_player_position.clone(), end_node);
        }
    }
}

fn move_player(graph: &mut Graph, last_run: &mut Instant, index: usize, path: &mut Vec<usize>) {
    graph.nodes[graph.current_player_position].value = Tile::Empty;
    if graph.nodes[index].value == Tile::Goal {
        reload(graph, path);
    }
    graph.nodes[index].value = Tile::Player;
    graph.current_player_position = index;
    *last_run = Instant::now();
}

fn get_distance(node_1: &Node, node_2: &Node) -> f32 {
    (((node_1.x - node_2.x).pow(2) + (node_1.y - node_2.y).pow(2)) as f32).sqrt()
}

#[macroquad::main("MapMaker")]
async fn main() {
    let mut graph = Graph::new();

    let mut path: Vec<usize> = Vec::new();
    let mut last_run = Instant::now();
    let movement_speed = 0.01;
    loop {
        keyboard_actions(&mut graph, &mut path);
        mouse_events(&mut graph, &mut path);
        if path.len() > 0 {
            let distance = get_distance(
                &graph.nodes[graph.current_player_position],
                &graph.nodes[path[path.len() - 1]],
            );
            if last_run.elapsed()
                > Duration::from_millis((distance / movement_speed).round() as u64)
            {
                move_player(&mut graph, &mut last_run, path.pop().unwrap(), &mut path);
            }
        }
        clear_background(WHITE);
        draw_graph(&graph);
        next_frame().await
    }
}

pub mod map;
use crate::map::*;
pub mod combat;
use crate::combat::*;

use macroquad::prelude::*;
use std::time::{Duration, Instant};

enum GameState {
    MainMap,
    EnterCombat,
    Combat,
    ExitCombat,
}

impl GameState {
    fn new() -> GameState {
        GameState::MainMap
    }
}

fn move_player(
    graph: &mut Graph,
    last_move: &mut Instant,
    game_state: &mut GameState,
    entered_combat: &mut Option<Instant>,
) {
    let movement_speed = 0.01;
    if graph.player_path.len() > 0 {
        let distance = graph.distance(
            graph.current_player_position.unwrap(),
            *graph.player_path.last().unwrap(),
        );
        let travel_time = Duration::from_millis((distance / movement_speed).round() as u64);
        if last_move.elapsed() >= travel_time {
            let next_pos = graph.player_path.pop().unwrap();
            graph.move_player(next_pos);
            *last_move = Instant::now();

            match graph.nodes[graph.current_player_position.unwrap()].value {
                Tile::Empty => (),
                Tile::Enemy(_) => {
                    *game_state = GameState::EnterCombat;
                    entered_combat.replace(Instant::now());
                }
                Tile::Treasure(_) => (),
            }
        }
    }
}

struct Player {
    _stamina: i32,
    _health: i32,
    _defense: i32,
    sentence: Vec<char>,
}

impl Player {
    fn new() -> Player {
        Player {
            _stamina: 50,
            _health: 100,
            _defense: 100,
            sentence: vec![],
        }
    }
}

#[macroquad::main("MapMaker")]
async fn main() {
    let mut player = Player::new();
    let mut game_state = GameState::new();
    let mut graph = Graph::new();
    let mut last_move = Instant::now();
    let mut entered_combat = None;
    let mut num_iterations: usize = 0;
    let sentence: Vec<char> = "Hello, world! This is a sentence to type!"
        .chars()
        .collect();

    loop {
        num_iterations += 1;
        clear_background(WHITE);
        match game_state {
            GameState::MainMap => {
                keyboard_actions(&mut graph);
                mouse_events(&mut graph);
                move_player(
                    &mut graph,
                    &mut last_move,
                    &mut game_state,
                    &mut entered_combat,
                );
                graph.draw_graph();
            }
            GameState::EnterCombat => match enter_combat_animation((0., 0.), &mut entered_combat) {
                State::Playing => (),
                State::Finished => game_state = GameState::Combat,
            },
            GameState::Combat => {
                num_iterations += 1;
                typing(&mut player.sentence, &num_iterations);
                match draw_combat(&sentence, &mut player.sentence) {
                    State::Playing => (),
                    State::Finished => {
                        game_state = GameState::ExitCombat;
                        player.sentence = Vec::new();
                    }
                }
            }
            GameState::ExitCombat => match enter_combat_animation((0., 0.), &mut entered_combat) {
                State::Playing => (),
                State::Finished => {
                    graph.nodes[graph.current_player_position.unwrap()].value = Tile::Empty;
                    game_state = GameState::MainMap;
                    last_move = Instant::now();
                }
            },
        }
        next_frame().await
    }
}

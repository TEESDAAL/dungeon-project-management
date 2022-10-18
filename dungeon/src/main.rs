pub mod map;
use crate::map::{keyboard_actions, load_map_textures, mouse_events, Graph, Tile};
pub mod combat;
use crate::combat::{
    draw_combat, enemy_attack, enter_combat_animation, load_combat_textures, typing, DeletionState,
    Player, State, SENTENCE_LOWER_BOUND, SENTENCE_UPPER_BOUND,
};
pub mod sentences;
use crate::sentences::{load_sentences, return_sentence};
use crate::treasure::{card_select, load_treasure_images, CardType, CARDS};
use ::rand::seq::SliceRandom;
use macroquad::prelude::*;
use std::time::{Duration, Instant};
pub mod treasure;

enum RewardType {
    Treasure,
    EndOfLevel,
}
enum GameState {
    LoadTextures,
    MainMap,
    EnterCombat,
    Combat,
    ExitCombat,
    Rewarded(RewardType),
}

impl GameState {
    fn new() -> GameState {
        GameState::LoadTextures
    }
}

fn move_player(
    graph: &mut Graph,
    last_move: &mut Instant,
    game_state: &mut GameState,
    entered_combat: &mut Option<Instant>,
    current_background: &mut usize,
) {
    let movement_speed = 0.01;
    if !graph.player_path.is_empty() {
        let distance = graph.distance(
            graph.current_player_position.unwrap(),
            *graph.player_path.last().unwrap(),
        );
        let travel_time = Duration::from_millis((distance / movement_speed).round() as u64);
        if last_move.elapsed() >= travel_time {
            let next_pos = graph.player_path.pop().unwrap();
            if next_pos == graph.goal_position.unwrap() {
                *game_state = GameState::Rewarded(RewardType::EndOfLevel);
                *current_background += 1;
                // println!("{}", graph.current_background);
            }
            graph.move_player(next_pos);
            *last_move = Instant::now();

            match graph.nodes[graph.current_player_position.unwrap()].value {
                Tile::Empty => (),
                Tile::Enemy(_) => {
                    *game_state = GameState::EnterCombat;
                    entered_combat.replace(Instant::now());
                }
                Tile::Treasure => *game_state = GameState::Rewarded(RewardType::Treasure),
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Dungeon Explorer".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new();
    let mut game_state = GameState::new();
    let mut graph = Graph::new();
    let mut last_move = Instant::now();
    let mut entered_combat = None;
    let mut sentence: Option<Vec<char>> = None;
    let mut time_since_last_delete = Instant::now();
    let mut deletion_state = DeletionState::FirstCharacter;
    let mut last_attack = Instant::now();
    let mut temp_damage_reduction = 0.0;
    let mut current_background = 0;
    let perm_damage_reduction = 0.0;

    let mut temp_words_reduction = 0;
    let perm_word_reduction = 0;

    while player.health > 0.0 {
        clear_background(WHITE);
        match game_state {
            GameState::LoadTextures => {
                load_map_textures();
                    load_sentences();
                    load_combat_textures();
                    load_treasure_images();
                game_state = GameState::MainMap;
            }
            GameState::MainMap => {
                get_char_pressed();
                keyboard_actions(&mut graph);
                mouse_events(&mut graph);
                move_player(
                    &mut graph,
                    &mut last_move,
                    &mut game_state,
                    &mut entered_combat,
                    &mut current_background,
                );
                graph.draw_graph(&player.armoured, &current_background);
            }
            GameState::EnterCombat => match enter_combat_animation((0., 0.), &mut entered_combat) {
                State::Playing => {
                    get_char_pressed();
                }
                State::Finished => {
                    sentence = None;
                    let word_reduction = temp_words_reduction + perm_word_reduction;
                    while sentence.is_none() {
                        let sentence_length = match ((SENTENCE_LOWER_BOUND - word_reduction)
                            ..(SENTENCE_UPPER_BOUND - word_reduction))
                            .collect::<Vec<usize>>()
                            .choose(&mut ::rand::thread_rng())
                        {
                            Some(length) => *length,
                            None => continue,
                        };
                        sentence = Some(match return_sentence(sentence_length) {
                            Some(sentence) => sentence.chars().collect(),
                            None => continue,
                        });
                    }
                    last_attack = Instant::now();
                    game_state = GameState::Combat;
                }
            },
            GameState::Combat => {
                let damage_reduction = perm_damage_reduction + temp_damage_reduction;
                enemy_attack(&mut player, &mut last_attack, damage_reduction);
                let test = sentence.clone();
                typing(
                    &mut player.sentence,
                    &mut deletion_state,
                    &mut time_since_last_delete,
                );
                match {
                    let level_info = &graph.background_order[current_background];
                    draw_combat(
                        &test.unwrap(),
                        &mut player,
                        &level_info.sky_color,
                        &level_info.ground_color,
                    )
                } {
                    State::Playing => (),
                    State::Finished => {
                        game_state = GameState::ExitCombat;
                        player.sentence = Vec::new();
                    }
                }
            }
            GameState::ExitCombat => {
                temp_damage_reduction = 0.0;
                player.armoured = false;
                match enter_combat_animation((0., 0.), &mut entered_combat) {
                    State::Playing => (),
                    State::Finished => {
                        graph.nodes[graph.current_player_position.unwrap()].value = Tile::Empty;
                        game_state = GameState::MainMap;
                        last_move = Instant::now();
                    }
                }
            }
            GameState::Rewarded(_) => {
                let cards_and_coords = vec![
                    (
                        CARDS[0].clone(),
                        (
                            screen_width() / 2.
                                - CARDS[0].card_width * 1.2
                                - CARDS[0].card_width / 2.,
                            screen_height() / 2. - CARDS[0].card_height / 2.,
                        ),
                    ),
                    (
                        CARDS[1].clone(),
                        (
                            screen_width() / 2. - CARDS[0].card_width / 2.,
                            screen_height() / 2. - CARDS[0].card_height / 2.,
                        ),
                    ),
                    (
                        CARDS[2].clone(),
                        (
                            screen_width() / 2. + CARDS[0].card_width * 1.2
                                - CARDS[0].card_width / 2.,
                            screen_height() / 2. - CARDS[0].card_height / 2.,
                        ),
                    ),
                ];
                graph.draw_graph(&player.armoured, &current_background);
                for (card, (x, y)) in &cards_and_coords {
                    card.draw_card(*x, *y);
                }

                if let Some(card) = card_select(&cards_and_coords) {
                    match card.card_type {
                        CardType::TempHeal => {
                            player.health += 40.;
                            if player.health > 100.0 {
                                player.health = 100.0;
                            }
                        }
                        CardType::TempDamageReduction => {
                            player.armoured = true;
                            temp_damage_reduction += 1.;
                        }
                        CardType::TempWordsReduce => temp_words_reduction += 10,
                    };
                    if graph.nodes[graph.current_player_position.unwrap()].value == Tile::Treasure {
                        graph.nodes[graph.current_player_position.unwrap()].value = Tile::Empty;
                    }
                    game_state = GameState::MainMap;
                }
            }
        }
        next_frame().await;
    }
}

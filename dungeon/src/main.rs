pub mod map;
use crate::map::*;
pub mod combat;
use crate::combat::*;
pub mod sentences;
use crate::sentences::*;
use crate::treasure::*;
pub mod end;
use crate::end::*;
use ::rand::seq::SliceRandom;
use futures::join;
use macroquad::prelude::*;
use std::time::{Duration, Instant};
pub mod treasure;
#[derive(Copy, Clone, PartialEq)]
enum RewardType {
    Treasure,
    EndOfLevel,
}

#[derive(Copy, Clone, PartialEq)]
enum EndCondition {
    Death,
    Success,
}

#[derive(PartialEq)]
enum GameState {
    LoadTextures,
    MainMap,
    EnterCombat,
    Combat,
    ExitCombat,
    Rewarded(RewardType),
    EndOfGame(EndCondition),
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

            // End the level (or the game) and load the next game state
            if next_pos == graph.goal_position.unwrap() {
                *current_background += 1;
                if *current_background == graph.background_order.len() {
                    *game_state = GameState::EndOfGame(EndCondition::Success);
                    return;
                }
                *game_state = GameState::Rewarded(RewardType::EndOfLevel);
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
struct Variables {
    last_move: Instant,
    entered_combat: Option<Instant>,
    sentence: Option<Vec<char>>,
    time_since_last_delete: Instant,
    deletion_state: DeletionState,
    last_attack: Instant,
    temp_damage_reduction: f32,
    current_background: usize,
    temp_words_reduction: f32,
    perm_word_reduction: f32,
    perm_damage_reduction: f32,
    num_enemies_defeated: usize,
}
impl Variables {
    fn new() -> Self {
        Variables {
            last_move: Instant::now(),
            entered_combat: None,
            sentence: None,
            time_since_last_delete: Instant::now(),
            deletion_state: DeletionState::FirstCharacter,
            last_attack: Instant::now(),
            temp_damage_reduction: 0.0,
            current_background: 0,
            temp_words_reduction: 1.,
            perm_word_reduction: 1.,
            perm_damage_reduction: 1.,
            num_enemies_defeated: 0,
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new();
    let mut game_state = GameState::new();
    let mut graph = Graph::new();
    let mut variables = Variables::new();

    loop {
        if player.health <= 0.0 {
            game_state = GameState::EndOfGame(EndCondition::Death);
        }
        clear_background(WHITE);
        match game_state {
            GameState::LoadTextures => {
                join!(
                    load_sentences(),
                    load_combat_textures(),
                    load_map_textures(),
                    load_treasure_images(),
                );
                game_state = GameState::MainMap;
            }
            GameState::MainMap => {
                get_char_pressed();
                mouse_events(&mut graph);
                move_player(
                    &mut graph,
                    &mut variables.last_move,
                    &mut game_state,
                    &mut variables.entered_combat,
                    &mut variables.current_background,
                );
                if game_state == GameState::MainMap {
                    graph.draw_graph(&player.armoured, &variables.current_background);
                }
            }
            GameState::EnterCombat => {
                match enter_combat_animation((0., 0.), &mut variables.entered_combat) {
                    State::Playing => {
                        get_char_pressed();
                    }
                    State::Finished => {
                        variables.sentence = None;
                        while variables.sentence == None {
                            let sentence_length = match (SENTENCE_LOWER_BOUND..SENTENCE_UPPER_BOUND)
                                .collect::<Vec<usize>>()
                                .choose(&mut ::rand::thread_rng())
                            {
                                Some(length) => (*length as f32 * variables.perm_word_reduction
                                    - variables.temp_words_reduction)
                                    .floor()
                                    as usize,
                                None => continue,
                            };
                            variables.sentence = Some(match return_sentence(sentence_length) {
                                Some(sentence) => sentence.chars().collect(),
                                None => continue,
                            });
                        }
                        variables.last_attack = Instant::now();
                        game_state = GameState::Combat
                    }
                }
            }
            GameState::Combat => {
                enemy_attack(
                    &mut player,
                    &mut variables.last_attack,
                    &variables.temp_damage_reduction,
                    &variables.perm_damage_reduction,
                );
                let test = variables.sentence.clone();
                typing(
                    &mut player.sentence,
                    &mut variables.deletion_state,
                    &mut variables.time_since_last_delete,
                );
                match {
                    let level_info = &graph.background_order[variables.current_background];
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
                variables.temp_damage_reduction = 0.0;
                player.armoured = false;
                variables.num_enemies_defeated += 1;
                match enter_combat_animation((0., 0.), &mut variables.entered_combat) {
                    State::Playing => (),
                    State::Finished => {
                        graph.nodes[graph.current_player_position.unwrap()].value = Tile::Empty;
                        game_state = GameState::MainMap;
                        variables.last_move = Instant::now();
                    }
                }
            }
            GameState::Rewarded(reward_type) => {
                let cards = match reward_type {
                    RewardType::Treasure => TEMP_CARDS.clone(),
                    RewardType::EndOfLevel => PERM_CARDS.clone(),
                };
                let cards_and_coords = vec![
                    (
                        cards[0].clone(),
                        (
                            screen_width() / 2.
                                - cards[0].card_width * 1.2
                                - cards[0].card_width / 2.,
                            screen_height() / 2. - cards[0].card_height / 2.,
                        ),
                    ),
                    (
                        cards[1].clone(),
                        (
                            screen_width() / 2. - cards[0].card_width / 2.,
                            screen_height() / 2. - cards[0].card_height / 2.,
                        ),
                    ),
                    (
                        cards[2].clone(),
                        (
                            screen_width() / 2. + cards[0].card_width * 1.2
                                - cards[0].card_width / 2.,
                            screen_height() / 2. - cards[0].card_height / 2.,
                        ),
                    ),
                ];
                graph.draw_graph(&player.armoured, &variables.current_background);
                for (card, (x, y)) in &cards_and_coords {
                    card.draw_card(*x, *y);
                }

                match card_select(&cards_and_coords) {
                    Some(card) => {
                        match card.card_type {
                            CardType::TempHeal => {
                                player.health += 40.;
                                if player.health > player.max_health {
                                    player.health = player.max_health;
                                }
                            }
                            CardType::TempDamageReduction => {
                                player.armoured = true;
                                variables.temp_damage_reduction += 1.
                            }
                            CardType::TempWordsReduce => variables.temp_words_reduction *= 0.90,
                            CardType::PermHeal => player.max_health *= 1.10,
                            CardType::PermDamageReduction => {
                                variables.perm_damage_reduction *= 0.95
                            }
                            CardType::PermWordsReduce => variables.perm_word_reduction *= 0.90,
                        };
                        if graph.nodes[graph.current_player_position.unwrap()].value
                            == Tile::Treasure
                        {
                            graph.nodes[graph.current_player_position.unwrap()].value = Tile::Empty
                        }
                        game_state = GameState::MainMap;
                    }
                    None => (),
                }
            }
            GameState::EndOfGame(end_type) => {
                match end_type {
                    EndCondition::Death => draw_death_screen(
                        &(variables.current_background + 1),
                        &variables.num_enemies_defeated,
                    ),
                    EndCondition::Success => draw_victory_screen(&variables.num_enemies_defeated),
                }
                if restart() {
                    player = Player::new();
                    game_state = GameState::new();
                    graph = Graph::new();
                    variables = Variables::new();
                }
            }
        }
        next_frame().await
    }
}

use lazy_static::lazy_static;
use macroquad::prelude::*;
use std::time::{Duration, Instant};

const PLAYER_SIZE: f32 = 200.0;

pub struct Player {
    pub _stamina: i32,
    pub health: i32,
    pub _defense: i32,
    pub sentence: Vec<char>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            _stamina: 50,
            health: 100,
            _defense: 100,
            sentence: vec![],
        }
    }
}

lazy_static! {
    pub static ref PLAYER_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/ferris-back.png"),
        Some(ImageFormat::Png),
    );
    pub static ref ENEMY_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/go-gopher.png"),
        Some(ImageFormat::Png),
    );
    pub static ref BASE_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/base.png"),
        Some(ImageFormat::Png),
    );
}
pub enum DeletionState {
    FirstCharacter,
    SecondCharacter,
    ThirdCharacter,
    EverythingElse,
}

pub enum State {
    Playing,
    Finished,
}
pub async fn load_combat_textures() {
    let _ = *PLAYER_TEXTURE;
    println!("Player texture loaded");
    let _ = *ENEMY_TEXTURE;
    println!("Enemy texture loaded");
    let _ = *BASE_TEXTURE;
    println!("Base texture loaded");
}
pub fn enter_combat_animation(_coords: (f32, f32), time: &mut Option<Instant>) -> State {
    if time.unwrap().elapsed() < Duration::from_millis(1000) {
        draw_rectangle(0., 0., screen_width(), screen_height(), RED);
        State::Playing
    } else {
        State::Finished
    }
}

pub fn draw_combat(sentence: &Vec<char>, player: &mut Player) -> State {
    let player_sentence = &player.sentence;
    draw_text(
        &format!("Player Health: {}", player.health)[..],
        screen_width() / 10.,
        screen_height() * 0.9,
        60.,
        BLACK,
    );
    draw_texture(
        *BASE_TEXTURE,
        screen_width() / 4. - BASE_TEXTURE.width() / 2.,
        screen_height() / 4. - BASE_TEXTURE.height() / 2.,
        WHITE,
    );
    draw_texture(
        *BASE_TEXTURE,
        screen_width() - (screen_width() / 4. - BASE_TEXTURE.width() / 2.),
        screen_height() - (screen_height() / 4. - BASE_TEXTURE.height() / 2.),
        WHITE,
    );
    draw_texture_ex(
        *ENEMY_TEXTURE,
        screen_width() / 4. - 100. / 2.,
        screen_height() / 4. - 100. / 1.5,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::from([100., 100.])),
            ..Default::default()
        },
    );

    let player_shrink_factor = PLAYER_SIZE / PLAYER_TEXTURE.width();
    draw_texture_ex(
        *PLAYER_TEXTURE,
        screen_width() - (screen_width() / 4. - BASE_TEXTURE.width() / 2.)
            + PLAYER_TEXTURE.width() * player_shrink_factor / 2.,
        screen_height()
            - (screen_height() / 4. - BASE_TEXTURE.height() / 2.)
            - PLAYER_SIZE * player_shrink_factor / 2.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::from([
                PLAYER_TEXTURE.width() * player_shrink_factor,
                PLAYER_TEXTURE.height() * player_shrink_factor,
            ])),
            ..Default::default()
        },
    );

    draw_sentence(sentence, player_sentence);

    if player_sentence == sentence {
        State::Finished
    } else {
        State::Playing
    }
}

fn draw_sentence(sentence: &Vec<char>, user_sentence: &Vec<char>) {
    let mut char_pairs: Vec<(Option<&char>, Option<&char>)> = Vec::new();
    let mut i = 0;
    loop {
        let char_pair = (user_sentence.get(i), sentence.get(i));
        match char_pair {
            (None, None) => break,
            _ => char_pairs.push(char_pair),
        }
        i += 1;
    }

    let x_upper_bound = screen_width() - 20.;
    let spacing = 25;
    let font_size = 50.;
    let mut shift = 0.;
    let mut y_pos = 50. - font_size / 2.;
    for (i, char_pair) in char_pairs.into_iter().enumerate() {
        let mut x_pos = (spacing * i) as f32 - shift;
        if x_pos >= x_upper_bound {
            shift += x_upper_bound;
            x_pos = 50. - font_size / 2. - shift;
            if x_pos < 0. {
                x_pos = 0.;
            }
            y_pos += font_size;
        }
        x_pos += 10.;
        match char_pair {
            (Some(c), Some(s)) => {
                if c == s {
                    draw_text(&c.to_string(), x_pos, y_pos, font_size, GREEN);
                    if c == &' ' {
                        draw_rectangle(
                            x_pos,
                            y_pos - font_size / 2.,
                            spacing as f32 - 5.,
                            font_size / 2.,
                            Color::from_rgba(0, 200, 0, 100),
                        );
                    }
                } else {
                    draw_text(&c.to_string(), x_pos, y_pos, font_size, RED);
                    if c == &' ' {
                        draw_rectangle(
                            x_pos,
                            y_pos - font_size / 2.,
                            spacing as f32 - 5.,
                            font_size / 2.,
                            Color::from_rgba(200, 0, 0, 100),
                        );
                    }
                }
            }
            (Some(c), None) => {
                draw_text(&c.to_string(), x_pos, y_pos, font_size, RED);
                if c == &' ' {
                    draw_rectangle(
                        x_pos,
                        y_pos - font_size / 2.,
                        spacing as f32 - 5.,
                        font_size / 2.,
                        Color::from_rgba(200, 0, 0, 100),
                    );
                }
            }
            (None, Some(s)) => draw_text(&s.to_string(), x_pos, y_pos, font_size, GRAY),
            (None, None) => break,
        }
    }
}

pub fn enemy_attack(player: &mut Player, last_attack: &mut Instant) {
    let enemy_attack_time = Duration::from_millis(2000);
    if last_attack.elapsed() < Duration::from_millis(200) {
        // *enemy_size /= 1.1;
    }
    if last_attack.elapsed() >= enemy_attack_time {
        // *enemy_size *= 1.1;
        player.health -= 3;
        *last_attack = Instant::now();
    }
}

pub fn typing(
    user_sentence: &mut Vec<char>,
    deletion_state: &mut DeletionState,
    time_since_last_delete: &mut Instant,
) {
    if let Some(c) = get_char_pressed() {
        user_sentence.push(c);
    }
    if is_key_released(KeyCode::Backspace) {
        *deletion_state = DeletionState::FirstCharacter;
    }
    if is_key_down(KeyCode::Backspace) {
        match deletion_state {
            DeletionState::FirstCharacter => {
                user_sentence.pop();
                *deletion_state = DeletionState::SecondCharacter;
                *time_since_last_delete = Instant::now();
            }
            DeletionState::SecondCharacter => {
                if time_since_last_delete.elapsed() > Duration::from_millis(400) {
                    user_sentence.pop();
                    *deletion_state = DeletionState::ThirdCharacter;
                    *time_since_last_delete = Instant::now();
                }
            }
            DeletionState::ThirdCharacter => {
                if time_since_last_delete.elapsed() > Duration::from_millis(200) {
                    user_sentence.pop();
                    *deletion_state = DeletionState::EverythingElse;
                    *time_since_last_delete = Instant::now();
                }
            }
            DeletionState::EverythingElse => {
                if time_since_last_delete.elapsed() > Duration::from_millis(50) {
                    user_sentence.pop();
                    *time_since_last_delete = Instant::now();
                }
            }
        }
    }
}

pub fn exit_combat_animation(_coords: (f32, f32), time: &mut Option<Instant>) -> State {
    if time.unwrap().elapsed() < Duration::from_millis(1000) {
        draw_rectangle(0., 0., screen_width(), screen_height(), RED);
        State::Playing
    } else {
        State::Finished
    }
}

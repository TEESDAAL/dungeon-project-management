use lazy_static::lazy_static;
use macroquad::prelude::*;
use std::time::{Duration, Instant};

pub const SENTENCE_UPPER_BOUND: usize = 100;
pub const SENTENCE_LOWER_BOUND: usize = 90;

pub struct Player {
    pub _stamina: i32,
    pub health: f32,
    pub _defense: i32,
    pub sentence: Vec<char>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            _stamina: 50,
            health: 100.0,
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
    // Draw Enemy and base
    let base_size = 90. + screen_width() / 6.;
    let base_shrink_factor = base_size / BASE_TEXTURE.width();
    let (enemy_x_pos, enemy_y_pos) = (screen_width() / 4., screen_height() / 2.5);
    let (base_width, base_height) = (
        BASE_TEXTURE.width() * base_shrink_factor,
        BASE_TEXTURE.height() * base_shrink_factor,
    );
    draw_texture_ex(
        *BASE_TEXTURE,
        enemy_x_pos - base_width / 2.,
        enemy_y_pos - base_height / 2.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::from([base_width, base_height])),
            ..Default::default()
        },
    );

    let enemy_size = screen_width() / 6.;
    let enemy_shrink_factor = enemy_size / ENEMY_TEXTURE.width();
    let (enemy_width, enemy_height) = (
        ENEMY_TEXTURE.width() * enemy_shrink_factor,
        ENEMY_TEXTURE.height() * enemy_shrink_factor,
    );
    draw_texture_ex(
        *ENEMY_TEXTURE,
        enemy_x_pos - enemy_width / 2.,
        enemy_y_pos - enemy_height / 1.2,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::from([enemy_width, enemy_height])),
            flip_x: true,
            ..Default::default()
        },
    );

    let base_size = 100. + screen_width() / 6.;
    let base_shrink_factor = base_size / BASE_TEXTURE.width();
    let (player_x_pos, player_y_pos) = (screen_width() * 4. / 5., screen_height() * 3. / 4.);
    let (base_width, base_height) = (
        BASE_TEXTURE.width() * base_shrink_factor,
        BASE_TEXTURE.height() * base_shrink_factor,
    );
    draw_texture_ex(
        *BASE_TEXTURE,
        player_x_pos - base_width / 2.,
        player_y_pos - base_height / 2.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::from([base_width, base_height])),
            ..Default::default()
        },
    );

    let player_size = screen_width() / 6.;
    let player_shrink_factor = player_size / PLAYER_TEXTURE.width();
    draw_texture_ex(
        *PLAYER_TEXTURE,
        player_x_pos - PLAYER_TEXTURE.width() * player_shrink_factor / 2.,
        player_y_pos - PLAYER_TEXTURE.height() * player_shrink_factor / 1.2,
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

pub fn return_lines(sentence: &Vec<char>, width: f32, font_size: u16) -> Vec<String> {
    let string_sentence = sentence.iter().collect::<String>();
    let words: Vec<&str> = string_sentence.split(' ').collect();
    let mut line: Vec<&str> = Vec::new();
    let mut temp_line = line.clone();
    let mut lines: Vec<Vec<&str>> = Vec::new();
    for word in words {
        temp_line.push(word);
        if measure_text(&temp_line.join(" ")[..], None, font_size, 1.).width > width {
            lines.push(line);
            line = vec![word];
            temp_line = line.clone();
        } else {
            line.push(word);
        }
    }
    if line.concat() != "" {
        lines.push(line);
    }

    lines.iter().map(|line| line.join(" ")).collect()
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
    let font_size = 50.;

    let x_upper_bound = screen_width() - 20.;
    let mut line_lengths: Vec<usize> = return_lines(&sentence, x_upper_bound, font_size as u16)
        .iter()
        .map(|line| line.len())
        .collect();

    let last_index = line_lengths.len() - 1;
    line_lengths[last_index] = 82;

    let spacing = 22;
    let mut y_pos = font_size as f32 / 2. + 10.;
    let mut num_lines = 0;

    let mut num_chars = 0;
    for char_pair in char_pairs.iter() {
        let x_pos = 10. + (spacing * num_chars) as f32;
        let line_length = match line_lengths.get(num_lines) {
            Some(length) => *length,
            None => 82,
        };

        let (c, color) = match *char_pair {
            (Some(c), Some(s)) => {
                let character = if *c == ' ' { '⊔' } else { *c };
                if c == s {
                    (character, Color::from_rgba(0, 200, 0, 100))
                } else {
                    (character, Color::from_rgba(200, 0, 0, 100))
                }
            }
            (Some(c), None) => {
                let character = if *c == ' ' { '⊔' } else { *c };
                (character, Color::from_rgba(200, 0, 0, 100))
            }
            (None, Some(s)) => (*s, GRAY),
            (None, None) => break,
        };
        draw_text_ex(
            &c.to_string()[..],
            x_pos,
            y_pos,
            TextParams {
                font_size: font_size as u16,
                font_scale: 1.,
                color,
                ..Default::default()
            },
        );
        num_chars += 1;

        if num_chars > line_length {
            num_chars = 0;
            num_lines += 1;
            y_pos += font_size;
        }
    }
}

pub fn enemy_attack(player: &mut Player, last_attack: &mut Instant, damage_reduction: f32) {
    let enemy_attack_time = Duration::from_millis(2000);
    if last_attack.elapsed() >= enemy_attack_time {
        player.health -= 3. - damage_reduction;
        *last_attack = Instant::now();
    }
}

pub fn typing(
    user_sentence: &mut Vec<char>,
    deletion_state: &mut DeletionState,
    time_since_last_delete: &mut Instant,
) {
    if let Some(c) = get_char_pressed() {
        if c != '\u{8}' {
            user_sentence.push(c);
        }
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
                if time_since_last_delete.elapsed() > Duration::from_millis(150) {
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

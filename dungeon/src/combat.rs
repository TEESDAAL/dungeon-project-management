use lazy_static::{initialize, lazy_static};
use macroquad::prelude::*;
use regex::Regex;
use std::env::consts::OS;
use std::time::{Duration, Instant};

pub const SENTENCE_UPPER_BOUND: usize = 70;
pub const SENTENCE_LOWER_BOUND: usize = 60;
pub struct Player {
    pub health: f32,
    pub max_health: f32,
    pub sentence: Vec<char>,
    pub armoured: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Self {
        Player {
            health: 100.0,
            max_health: 100.0,
            sentence: vec![],
            armoured: false,
        }
    }
}

lazy_static! {
    pub static ref MAX_LINE_LENGTH: usize = if OS == "windows" { 55 } else { 65 };
    pub static ref FONT_SIZE: u16 = if OS == "windows" { 40 } else { 50 };
    pub static ref CHAR_SPACING: usize = if OS == "windows" { 18 } else { 22 };
    pub static ref PLAYER_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/ferris-back.png"),
        Some(ImageFormat::Png),
    );
    pub static ref ARMOURED_PLAYER_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/armored_ferris-back.png"),
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

pub enum CombatState {
    Playing,
    Finished,
}
pub async fn load_combat_textures() {
    initialize(&PLAYER_TEXTURE);
    println!("Player texture loaded");
    initialize(&ARMOURED_PLAYER_TEXTURE);
    println!("Armoured player texture loaded");
    initialize(&ENEMY_TEXTURE);
    println!("Enemy texture loaded");
    initialize(&BASE_TEXTURE);
    println!("Base texture loaded");
}
pub fn enter_combat_animation(_coords: (f32, f32), time: &mut Option<Instant>) -> CombatState {
    if time.unwrap().elapsed() < Duration::from_millis(1000) {
        draw_rectangle(0., 0., screen_width(), screen_height(), RED);
        CombatState::Playing
    } else {
        CombatState::Finished
    }
}

pub fn draw_combat_background(sky_color: &Color, ground_color: &Color) {
    let skyline = screen_height() / 3.;
    draw_rectangle(0., 0., screen_width(), skyline, *sky_color);
    draw_rectangle(
        0.,
        skyline,
        screen_width(),
        screen_height() - skyline,
        *ground_color,
    );
}
pub fn draw_combat(
    sentence: &[char],
    player: &mut Player,
    sky_color: &Color,
    ground_color: &Color,
) -> CombatState {
    draw_combat_background(sky_color, ground_color);
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
    let texture = if player.armoured {
        *ARMOURED_PLAYER_TEXTURE
    } else {
        *PLAYER_TEXTURE
    };
    let player_size = screen_width() / 6.;
    let player_shrink_factor = player_size / texture.width();
    draw_texture_ex(
        texture,
        player_x_pos - texture.width() * player_shrink_factor / 2.,
        player_y_pos - texture.height() * player_shrink_factor / 1.2,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::from([
                texture.width() * player_shrink_factor,
                texture.height() * player_shrink_factor,
            ])),
            ..Default::default()
        },
    );

    draw_sentence(sentence, player_sentence, sky_color);

    if player_sentence == sentence {
        CombatState::Finished
    } else {
        CombatState::Playing
    }
}

pub fn return_lines(sentence: &[char]) -> Vec<String> {
    let string_sentence = sentence.iter().collect::<String>();
    let words: Vec<&str> = string_sentence.split(' ').collect();
    let mut line: Vec<&str> = Vec::new();
    let mut temp_line = line.clone();
    let mut lines: Vec<Vec<&str>> = Vec::new();
    for word in words {
        temp_line.push(word);
        // if measure_text(&temp_line.join(" ")[..], None, *FONT_SIZE, 1.).width >= width {
        if temp_line.join(" ").len() >= *MAX_LINE_LENGTH {
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

fn draw_text_box(x: f32, y: f32, w: f32, h: f32, sky_color: &Color) {
    let color = if sky_color == &BLACK { WHITE } else { BLACK };
    draw_rectangle(x, y, w, h, WHITE);

    // Draw top two lines
    draw_line(x, y - 10., x + w, y - 10., 3., color);
    draw_line(x, y, x + w, y, 3., color);

    // Draw bottom two lines
    draw_line(x, y + h, x + w, y + h, 3., color);
    draw_line(x, y + h + 10., x + w, y + h + 10., 3., color);

    // Draw left two lines
    draw_line(x, y, x, y + h, 3., color);
    draw_line(x - 10., y, x - 10., y + h, 3., color);

    // Draw right two lines
    draw_line(x + w, y, x + w, y + h, 3., color);
    draw_line(x + w + 10., y, x + w + 10., y + h, 3., color);

    // Draw the top left diagonal lines
    draw_line(x + 10., y - 20., x - 20., y + 10., 3., color);
    draw_line(x + 10., y - 10., x - 10., y + 10., 3., color);

    // Draw the top right diagonal lines
    draw_line(x + w - 10., y - 20., x + w + 20., y + 10., 3., color);
    draw_line(x + w - 10., y - 10., x + w + 10., y + 10., 3., color);

    // Draw the bottom left diagonal lines
    draw_line(x - 20., y + h - 10., x + 10., y + h + 20., 3., color);
    draw_line(x - 10., y + h - 10., x + 10., y + h + 10., 3., color);

    // Draw the bottom right diagonal lines
    draw_line(
        x + w + 20.,
        y + h - 10.,
        x + w - 10.,
        y + h + 20.,
        3.,
        color,
    );
    draw_line(
        x + w + 10.,
        y + h - 10.,
        x + w - 10.,
        y + h + 10.,
        3.,
        color,
    );
}

fn draw_sentence(sentence: &[char], user_sentence: &[char], sky_color: &Color) {
    let mut char_pairs: Vec<(Option<&char>, Option<&char>)> = Vec::new();
    let mut i = 0;
    let text_box_width = *MAX_LINE_LENGTH as f32 * (*CHAR_SPACING as f32 + 0.5);
    loop {
        let char_pair = (user_sentence.get(i), sentence.get(i));
        match char_pair {
            (None, None) => break,
            _ => char_pairs.push(char_pair),
        }
        i += 1;
    }

    let mut line_lengths: Vec<usize> = return_lines(sentence)
        .iter()
        .map(std::string::String::len)
        .collect();

    let last_index = line_lengths.len() - 1;
    line_lengths[last_index] = *MAX_LINE_LENGTH;

    let mut y_pos = f32::from(*FONT_SIZE) / 2. + 40.;
    let mut num_lines = 0;

    let mut num_chars = 0;
    let mut base_x_pos = 40.;
    draw_text_box(
        base_x_pos,
        y_pos - f32::from(*FONT_SIZE) / 2.,
        text_box_width,
        f32::from(*FONT_SIZE) * line_lengths.len() as f32,
        &sky_color,
    );
    base_x_pos += 5.;
    y_pos += 7.;
    for char_pair in &char_pairs {
        let x_pos = base_x_pos + (*CHAR_SPACING * num_chars) as f32;
        let line_length = match line_lengths.get(num_lines) {
            Some(length) => *length,
            None => *MAX_LINE_LENGTH,
        };

        let (c, color) = match *char_pair {
            (Some(c), Some(s)) => {
                let character = if *c == ' ' { '⊔' } else { *c };
                if c == s {
                    (character, Color::from_rgba(0, 182, 0, 255))
                } else {
                    (character, Color::from_rgba(182, 0, 0, 255))
                }
            }
            (Some(c), None) => {
                let character = if *c == ' ' { '⊔' } else { *c };
                (character, Color::from_rgba(182, 0, 0, 255))
            }
            (None, Some(s)) => (*s, Color::from_rgba(0, 0, 0, 255)),
            (None, None) => break,
        };
        draw_text_ex(
            &c.to_string()[..],
            x_pos,
            y_pos,
            TextParams {
                font_size: *FONT_SIZE,
                font_scale: 1.,
                color,
                ..Default::default()
            },
        );
        num_chars += 1;

        if num_chars > line_length {
            num_chars = 0;
            num_lines += 1;
            y_pos += f32::from(*FONT_SIZE);
        }
    }
}

pub fn enemy_attack(
    player: &mut Player,
    last_attack: &mut Instant,
    damage_reduction: &f32,
    damage_percentage: &f32,
) {
    let enemy_attack_time = Duration::from_millis(2000);
    if last_attack.elapsed() >= enemy_attack_time {
        let damage = (3. * damage_percentage) - damage_reduction;
        if damage > 0. {
            player.health -= damage;
        }
        *last_attack = Instant::now();
    }
}

pub fn typing(
    user_sentence: &mut Vec<char>,
    deletion_state: &mut DeletionState,
    time_since_last_delete: &mut Instant,
) {
    if let Some(c) = get_char_pressed() {
        if Regex::new(r"[\x20-\x7e]")
            .unwrap()
            .is_match(&c.to_string()[..])
        {
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

pub fn exit_combat_animation(_coords: (f32, f32), time: &mut Option<Instant>) -> CombatState {
    if time.unwrap().elapsed() < Duration::from_millis(1000) {
        draw_rectangle(0., 0., screen_width(), screen_height(), RED);
        CombatState::Playing
    } else {
        CombatState::Finished
    }
}

use lazy_static::lazy_static;
use macroquad::prelude::*;
use std::time::{Duration, Instant};

lazy_static! {
    pub static ref PLAYER_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/ferris-back.png"),
        Some(ImageFormat::Png),
    );
    pub static ref ENEMY_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/go-gopher.png"),
        Some(ImageFormat::Png),
    );
}

pub enum State {
    Playing,
    Finished,
}

pub fn enter_combat_animation(_coords: (f32, f32), time: &mut Option<Instant>) -> State {
    if time.unwrap().elapsed() < Duration::from_millis(1000) {
        draw_rectangle(0., 0., screen_width(), screen_height(), RED);
        State::Playing
    } else {
        State::Finished
    }
}

pub fn draw_combat(sentence: &Vec<char>, player_sentence: &Vec<char>) -> State {
    let base_texture = Texture2D::from_file_with_format(include_bytes!("../assets/base.png"), None);

    draw_texture(
        base_texture,
        screen_width() / 4. - base_texture.width() / 2.,
        screen_height() / 4. - base_texture.height() / 2.,
        WHITE,
    );
    draw_texture(
        base_texture,
        screen_width() - (screen_width() / 4. - base_texture.width() / 2.),
        screen_height() - (screen_height() / 4. - base_texture.height() / 2.),
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

    draw_texture_ex(
        *PLAYER_TEXTURE,
        screen_width() - (screen_width() / 4. - base_texture.width() / 2.) - 100. / 2.,
        screen_height() - (screen_height() / 4. - base_texture.height() / 2.) - 100. / 1.5,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::from([100., 100.])),
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

    let spacing = 25;
    let font_size = 50.;
    for (i, char_pair) in char_pairs.into_iter().enumerate() {
        match char_pair {
            (Some(c), Some(s)) => {
                if c == s {
                    draw_text(&c.to_string(), (spacing * i) as f32, 50., font_size, GREEN);
                    if c == &' ' {
                        draw_rectangle(
                            (spacing * i) as f32,
                            50. - font_size / 2.,
                            spacing as f32 - 5.,
                            font_size / 2.,
                            Color::from_rgba(0, 200, 0, 100),
                        );
                    }
                } else {
                    draw_text(&c.to_string(), (spacing * i) as f32, 50., font_size, RED);
                    if c == &' ' {
                        draw_rectangle(
                            (spacing * i) as f32,
                            50. - font_size / 2.,
                            spacing as f32 - 5.,
                            font_size / 2.,
                            Color::from_rgba(200, 0, 0, 100),
                        );
                    }
                }
            }
            (Some(c), None) => {
                draw_text(&c.to_string(), (spacing * i) as f32, 50., font_size, RED);
                if c == &' ' {
                    draw_rectangle(
                        (spacing * i) as f32,
                        50. - font_size / 2.,
                        spacing as f32 - 5.,
                        font_size / 2.,
                        Color::from_rgba(200, 0, 0, 100),
                    );
                }
            }
            (None, Some(s)) => {
                draw_text(&s.to_string(), (spacing * i) as f32, 50., font_size, GRAY)
            }
            (None, None) => break,
        }
    }
}

pub fn typing(user_sentence: &mut Vec<char>, num_iterations: &usize) {
    if let Some(c) = get_char_pressed() {
        user_sentence.push(c);
        println!("{}", c);
    }

    if is_key_down(KeyCode::Backspace) {
        user_sentence.pop();
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

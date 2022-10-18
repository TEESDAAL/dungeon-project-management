use lazy_static::lazy_static;
use macroquad::prelude::*;
const FERRIS_SIZE: f32 = 500.;

lazy_static! {
    pub static ref DEAD_FERRIS_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/dead-ferris.png"),
        Some(ImageFormat::Png),
    );
    pub static ref VICTORIOUS_FERRIS_TEXTURE: Texture2D = Texture2D::from_file_with_format(
        include_bytes!("../assets/victorious-ferris.png"),
        Some(ImageFormat::Png),
    );
}

pub fn draw_death_screen(num_levels: &usize, num_enemies_defeated: &usize) {
    clear_background(BLACK);
    let ferris_shrink_factor = FERRIS_SIZE / DEAD_FERRIS_TEXTURE.width();
    let font_size = 40;
    draw_text_ex(
        "You died :(",
        screen_width() / 2. - measure_text("You died :(", None, font_size, 1.).width / 2.,
        screen_height() / 5. - f32::from(font_size),
        TextParams {
            font_size,
            font_scale: 1.0,
            color: WHITE,
            font_scale_aspect: 1.0,
            ..Default::default()
        },
    );
    let text_to_draw = format!(
        "You reached level {} and defeated {} enemies.",
        num_levels, num_enemies_defeated
    );
    draw_text_ex(
        &text_to_draw,
        screen_width() / 2. - measure_text(&text_to_draw, None, font_size, 1.).width / 2.,
        screen_height() / 5.,
        TextParams {
            font_size,
            font_scale: 1.0,
            color: WHITE,
            font_scale_aspect: 1.0,
            ..Default::default()
        },
    );
    draw_texture_ex(
        *DEAD_FERRIS_TEXTURE,
        screen_width() / 2. - DEAD_FERRIS_TEXTURE.width() * ferris_shrink_factor / 2.,
        screen_height() / 2. - DEAD_FERRIS_TEXTURE.height() * ferris_shrink_factor / 2.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::from([
                DEAD_FERRIS_TEXTURE.width() * ferris_shrink_factor,
                DEAD_FERRIS_TEXTURE.height() * ferris_shrink_factor,
            ])),
            ..Default::default()
        },
    );

    draw_text_ex(
        "Press 'r' to play again",
        screen_width() / 2.
            - measure_text("Press 'r' to play again", None, font_size, 1.).width / 2.,
        4. * screen_height() / 5.,
        TextParams {
            font_size,
            font_scale: 1.0,
            color: WHITE,
            font_scale_aspect: 1.0,
            ..Default::default()
        },
    );
}

pub fn draw_victory_screen(num_enemies_defeated: &usize) {
    clear_background(BLACK);
    let ferris_shrink_factor = FERRIS_SIZE / DEAD_FERRIS_TEXTURE.width();
    let font_size = 40;
    let text_to_draw = format!(
        "You reached the final level and defeated {} enemies.",
        num_enemies_defeated
    );
    draw_text_ex(
        &text_to_draw,
        screen_width() / 2. - measure_text(&text_to_draw, None, font_size, 1.).width / 2.,
        screen_height() / 5.,
        TextParams {
            font_size,
            font_scale: 1.0,
            color: WHITE,
            font_scale_aspect: 1.0,
            ..Default::default()
        },
    );
    draw_texture_ex(
        *VICTORIOUS_FERRIS_TEXTURE,
        screen_width() / 2. - VICTORIOUS_FERRIS_TEXTURE.width() * ferris_shrink_factor / 2.,
        screen_height() / 2. - VICTORIOUS_FERRIS_TEXTURE.height() * ferris_shrink_factor / 2.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::from([
                VICTORIOUS_FERRIS_TEXTURE.width() * ferris_shrink_factor,
                VICTORIOUS_FERRIS_TEXTURE.height() * ferris_shrink_factor,
            ])),
            ..Default::default()
        },
    );
    draw_text_ex(
        "Press 'r' to play again",
        screen_width() / 2.
            - measure_text("Press 'r' to play again", None, font_size, 1.).width / 2.,
        4. * screen_height() / 5.,
        TextParams {
            font_size,
            font_scale: 1.0,
            color: WHITE,
            font_scale_aspect: 1.0,
            ..Default::default()
        },
    );
}

#[must_use] pub fn restart() -> bool {
    is_key_pressed(KeyCode::R)
}

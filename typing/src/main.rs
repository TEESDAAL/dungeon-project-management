use macroquad::prelude::*;

fn draw_sentance(sentance: &Vec<char>, user_sentance: &mut Vec<char>) {
    let mut char_pairs: Vec<(Option<&char>, Option<&char>)> = Vec::new();
    let mut i = 0;
    loop {
        let char_pair = (user_sentance.get(i), sentance.get(i));
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

fn keyboard_events(user_sentance: &mut Vec<char>, num_iterations: &usize) {
    match get_char_pressed() {
        Some(c) => user_sentance.push(c),
        None => (),
    }
    if is_key_down(KeyCode::Backspace) && num_iterations % 5 == 0 {
        user_sentance.pop();
    }
}

#[macroquad::main("MapMaker")]
async fn main() {
    let sentance: Vec<char> = "Hello, world! This is a sentance to type".chars().collect();
    let mut user_sentance: Vec<char> = Vec::new();
    let mut num_iterations: usize = 0;
    loop {
        num_iterations += 1;
        keyboard_events(&mut user_sentance, &num_iterations);
        draw_sentance(&sentance, &mut user_sentance);
        next_frame().await
    }
}

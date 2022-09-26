use macroquad::{prelude::*, text};
pub struct Card {
    pub title: String,
    pub image: Texture2D,
    pub description: String,
    pub card_width: f32,
    pub card_height: f32,
}

impl Card {
    pub fn draw_card(&self, x: f32, y: f32) {
        let (base_x, base_y) = (x - self.card_width / 2., y - self.card_height / 2.);

        draw_rectangle(
            base_x,
            base_y,
            self.card_width,
            0.15 * self.card_height,
            WHITE,
        );
        draw_rectangle_lines(
            base_x,
            base_y,
            self.card_width,
            0.15 * self.card_height,
            5.,
            BLACK,
        );
        let font_size = 40.;
        let td = measure_text(&self.title[..], None, 1, font_size);
        draw_text(
            &self.title[..],
            x - self.card_width / 2. + td.width / 8.,
            y - self.card_height / 2.
                + (0.15 * self.card_height) / 2.
                + (td.height - td.offset_y) / 4.,
            font_size,
            BLACK,
        );

        draw_rectangle(
            base_x,
            base_y + 0.15 * self.card_height,
            self.card_width,
            self.card_height / 2.,
            GREEN,
        );
        let text_box_y = base_y + 0.15 * self.card_height + self.card_height / 2.;
        let text_box_height = self.card_height - 0.15 * self.card_height - self.card_height / 2.;

        draw_rectangle(base_x, text_box_y, self.card_width, text_box_height, WHITE);

        draw_rectangle_lines(
            base_x,
            text_box_y,
            self.card_width,
            text_box_height,
            5.,
            BLACK,
        );

        draw_rectangle_lines(base_x, base_y, self.card_width, self.card_height, 5., BLACK);
    }
}

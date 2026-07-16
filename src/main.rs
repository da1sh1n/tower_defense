use macroquad::prelude::*;

#[macroquad::main("td")]
async fn main() {
    loop {
        clear_background(DARKGRAY);
        draw_rectangle(screen_width() / 2.0 - 50.0, screen_height() / 2.0 - 50.0, 100.0, 100.0, BLUE);

        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 40.0, GOLD);
        draw_text("td - it works", 20.0, 20.0, 30.0, WHITE);
        

        next_frame().await;
    }
}


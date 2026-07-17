#![allow(non_snake_case)]

use macroquad::prelude::*;
use std::sync::atomic::Ordering;
mod globals;
use globals::{IS_FULLSCREEN, WINDOWED_DIM_POS};

fn window_conf() -> Conf {
    Conf {
        window_title: "td".to_owned(),
        window_resizable: true,
        fullscreen: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        let alt_enter_pressed = (is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt))
            && is_key_pressed(KeyCode::Enter);

        if is_key_pressed(KeyCode::F11) || alt_enter_pressed {
            let entering_fullscreen = !IS_FULLSCREEN.load(Ordering::Relaxed);

            if entering_fullscreen {
                let (x, y) = miniquad::window::get_window_position();
                let (width, height) = miniquad::window::screen_size();
                *WINDOWED_DIM_POS.lock().unwrap() = Rect::new(x as f32, y as f32, width, height);
            }

            set_fullscreen(entering_fullscreen);
            IS_FULLSCREEN.store(entering_fullscreen, Ordering::Relaxed);

            if !entering_fullscreen {
                let saved = WINDOWED_DIM_POS.lock().unwrap();
                miniquad::window::set_window_size(saved.w as u32, saved.h as u32);
                miniquad::window::set_window_position(saved.x as u32, saved.y as u32);
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(GRAY);

        next_frame().await;
    }
}


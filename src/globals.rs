use macroquad::prelude::Rect;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

// ========== Constants ==========
const DEFAULT_WINDOW_WIDTH: u32 = 1280;
const DEFAULT_WINDOW_HEIGHT: u32 = 720;
// ========== Mutable globals ==========

pub static IS_FULLSCREEN: AtomicBool = AtomicBool::new(true);

pub static WINDOWED_DIM_POS: Mutex<Rect> = Mutex::new(
    Rect::new(
        0.0, 
        0.0, 
        DEFAULT_WINDOW_WIDTH as f32, 
        DEFAULT_WINDOW_HEIGHT as f32
    )
);

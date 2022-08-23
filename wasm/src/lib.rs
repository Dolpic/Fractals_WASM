mod utils;
mod mandelbrot;

use wasm_bindgen::prelude::*;
use std::panic;
use crate::mandelbrot::Mandelbrot;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn initialize(){
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn compute(width:u32, height:u32, pos_x:f64, pos_y:f64, scale:u64, iterations:u32, colors:Vec<u8>) -> *const u8{
    let mandel = Mandelbrot::new(width as usize, height as usize, pos_x, pos_y, scale as u128, iterations, colors);
    return mandel.compute().as_ptr();
}

use wasm_bindgen::prelude::*;
use crate::mandelbrot::Mandelbrot;

mod mandelbrot;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn compute(function_nb:u32, width:u32, height:u32, pos_x:f64, pos_y:f64, scale:u64, iterations:u32, colors:Vec<u8>, precise:bool) -> *const u8{
    let mandel = Mandelbrot::new(function_nb, width as usize, height as usize, pos_x, pos_y, scale, iterations, colors);
    if precise {
            return mandel.compute_precise().as_ptr();
    }else{
            return mandel.compute().as_ptr();
    }
}

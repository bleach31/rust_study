mod utils;
mod logic;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::ImageData;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log{
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


macro_rules! measure_elasped_time{
    ($t:tt,$s:block) =>{{
        let window = web_sys::window().expect("should have a window in ths context");
        let performance = window
            .performance()
            .expect("performance should be available");
        let start = performance.now();
        let end = performance.now();
        console_log!("{}:{}[ms]",$t, end - start);
        result
    }}
}

#[wasm_bindgen]
pub fn generate_mandelbrot_set(
    canvas_w: usize,
    canvas_h: usize,
    x_min: f64,
    x_max: f64,
    y_mix: f64,
    y_max: f64,
    max_iter: usize,
) -> Vec<u8>{
    measure_elasped_time!("generate:wasm\telasped:",{
        
    })

}
#[macro_use]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

mod mandelbrot;
mod utils;
#[macro_use]
extern crate lazy_static;
extern crate js_sys;

use mandelbrot::{mandelbrot_set, calc_color, write_buffer};
use std::os::raw::c_double;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use num_complex::Complex64;
use js_sys::Uint32Array;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref DIMS: Mutex<(c_double, c_double)> = Mutex::new((640.0, 480.0));
    static ref ZOOM: Mutex<f64> = Mutex::new(0.009);
}

#[wasm_bindgen(module = "/www/draw.js")]
extern "C" {
    fn draw(x: c_double, y: c_double, r: c_double, g: c_double, b: c_double);
    fn draw_buffer();
}

#[wasm_bindgen]
pub fn resize(width: c_double, height: c_double) {
    utils::set_panic_hook();
    *DIMS.lock().unwrap() = (width, height);
}

#[wasm_bindgen]
pub fn update(_offset_x: c_double, _offset_y: c_double, data: &Uint32Array )
    -> c_double {
    let (width, height) = *DIMS.lock().unwrap();
    const MAX_ITERS : u32 = 1024;
    let mut zoom = 0.;
    if let Ok(ref mut mutex) = ZOOM.lock() {
        **mutex *= 0.98;
        zoom = **mutex;
    }

    let offset_y = -0.808 + _offset_y * zoom;
    let offset_x = -0.1998 + _offset_x * zoom;

    let mut c = Complex64{
        re: -1. * width / 2. * zoom + offset_x,
        im: -1. * height / 2. * zoom + offset_y
    };

    for x in 0..width as usize {
        c.re += zoom;
        let mut tmp = c;
        for y in 0..height as usize {
            tmp.im += zoom;
            let value = mandelbrot_set(tmp, MAX_ITERS);
            let (r,g,b) = calc_color(value);
            write_buffer(x,y,width as usize, r,g,b, &data);
        }
    }
    draw_buffer();
    zoom
}

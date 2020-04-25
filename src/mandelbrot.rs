extern crate num_complex;

use num_complex::Complex64;
use js_sys::Uint32Array;

// pub fn calc_color(iter: Option<u32>, iters: u32) -> (u32, u32, u32) {
//     let (mut r,mut g,mut b) = (0, 0, 0);
//     let iter : u32 = match iter {
//         Some(value) => value,
//         None => return(r, g, b)
//     };
//     let color = 255 * iter / (iters - 1);
//     let block = iters / 4;
//     if iter < block {
//         r = color;
//         b = color;
//     } else if iter >= block && iter < block * 2 {
//         b = color;
//         g = color;
//     } else if iter >= block * 2 && iter < block * 3 {
//         r = color;
//         g = color;
//     } else if iter >= block * 3 && iter < block * 4 {
//         r = color;
//     }
//     (r, g, b)
// }

pub fn calc_color(iter: Option<u32>) -> (u32, u32, u32) {
    match iter {
        Some(value) => (value, value, value),
        None => (0, 0, 0)
    }
}


pub fn mandelbrot_set(c: Complex64, iters: u32)
    -> Option<u32> {
    let mut z = Complex64 { re: 0.0, im: 0.0 };
    for i in 0..iters {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None

}


pub fn write_buffer(x: usize, y: usize, width: usize, red: u32, green: u32,
                    blue: u32, data: &Uint32Array) {
    data.set_index(
        (y * width + x) as u32,
        (255   << 24) |   // alpha
        (blue << 16) |    // blue
        (green <<  8) |   // green
        red               // red
    );
}
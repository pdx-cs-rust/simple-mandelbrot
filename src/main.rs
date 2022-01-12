//! Render a Mandelbrot set slowly using NetPGM
//!
//! Bart Massey 2022

use num_complex::Complex;

/// Run a Mandelbrot iteration.
/// Code adapted from *Programming Rust* 2nd ed, Blandy et al 2021, p. 25.
fn escape_time(c: Complex<f64>, limit: u64) -> u64 {
    let mut z: Complex<f64> = Complex::default();
    for count in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return count;
        }
    }
    limit
}

/// Linear-interpolate 0..`pos`..`end` to `xstart`..`xend`.
fn posn(pos: usize, end: usize, xstart: f64, xend: f64) -> f64 {
    assert!(xstart < xend);
    assert!(pos <= end);
    (pos as f64 / end as f64) / (xend - xstart) + xstart
}

#[test]
fn test_posn() {
    let approx = |x0: f64, x: f64| (x - x0).abs() < 0.001;
    let tests = &[(0, 1.0f64), (80, 2.0), (40, 1.5)];
    for (pos, out) in tests {
        assert!(approx(posn(*pos, 80, 1.0, 2.0), *out));
    }
}

#[test]
#[should_panic]
fn test_posn_flipped() {
    let _ = posn(12, 80, 1.0, -1.0);
}

/// Render the image.
fn main() {
    // XXX Hardwire everything for now.
    let ((left, top), (right, bottom)) = ((-1.0, 0.0), (0.0, 1.0));
    let (width, height) = (640, 480);
    let escape = 255;

    // http://netpbm.sourceforge.net/doc/pgm.html
    println!("P2");
    println!("{} {}", width, height);
    println!("{}", escape);

    for row in 0..height {
        for col in 0..width {
            let (x, y) = (
                posn(col, width, left, right),
                posn(row, height, top, bottom),
            );
            let offset = Complex { re: x, im: y };
            let pixel = escape - escape_time(offset, escape);
            println!("{}", pixel);
        }
    }
}

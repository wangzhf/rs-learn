extern crate rayon;

use rayon::prelude::*;

pub fn test_parallel() {
    let mut colors = [-20.0f32, 0.0, 20.0, 40.0,
        80.0, 100.0, 150.0, 180.0, 200.0, 250.0, 300.0];

    println!("original: {:?}", &colors);

    colors.par_iter_mut().for_each(|color| {
        let c: f32 = if *color < 0.0 {
            0.0
        } else if *color > 255.0 {
            255.0
        } else {
            *color
        };
        *color = c / 255.0;
    });

    println!("transformed: {:?}", &colors);
}

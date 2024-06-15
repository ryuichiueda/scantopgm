//SPDX-FileCopyrightText: 2024 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct Image {
    pub x_max: f64, //upper edge of image
    pub x_min: f64, //bottom edge of image
    pub y_max: f64, //left edge of image
    pub y_min: f64, //right edge of image
    pub height: usize,
    pub width: usize,
    pub x_step: f64,
    pub y_step: f64,
    pub data: HashMap<(i32, i32), i32>,
}

impl Image {
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64,
           width: usize, height: usize) -> Image {
        Image {
            x_max: x_max,
            x_min: x_min,
            y_max: y_max,
            y_min: y_min,
            height: height,
            width: width,
            x_step: (x_max - x_min)/(height as f64),
            y_step: (y_max - y_min)/(width as f64),
            data: HashMap::new(),
        }
    }

    pub fn normalize(&mut self) {
        let max = *self.data.values().max().unwrap();
        for e in &mut self.data {
            let val = (*e.1 as f64)/(max as f64) * 255.0;
            *e.1 = val as i32;
        }
    }

    pub fn binarize(&mut self, threshold: i32) {
        for e in &mut self.data {
            let val = if *e.1 > threshold {
                255
            }else{
                0
            };
            *e.1 = val;
        }
    }

    pub fn pos_to_pixel(&self, x: f64, y: f64) -> (i32, i32) {
        let px = (self.x_max - x)/(self.x_max - self.x_min)*(self.height as f64);
        let py = (self.y_max - y)/(self.y_max - self.y_min)*(self.width as f64);

        (px as i32, py as i32)
    }

    pub fn pgm_out(&self, filename: &str) {
        let mut file = BufWriter::new(File::create(&filename).unwrap());

        file.write(format!("P2\n{} {}\n255\n", self.width, self.height).as_bytes()).unwrap();

        for x in 0..self.height {
            for y in 0..self.width {
                let v = match self.data.get( &(x as i32, y as i32) ) {
                    Some(v) => *v,
                    None    => 0,
                };

                file.write(format!("{} ", v).as_bytes()).unwrap();
            }
            file.write("\n".as_bytes()).unwrap();
        }
    }
}

use core::panic;
use std::path::Path;

use crate::util::{dump_grid, get_lines};

struct Image {
    image: Vec<Vec<Vec<u8>>>,
}

impl Image {
    fn new(input: &[u8], width: usize, height: usize) {}
}

fn p2(lines: Vec<String>) {
    let image_data: Vec<u8> = lines[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let width = 25;
    let height = 6;

    let mut final_image: Vec<Vec<u8>> = Vec::with_capacity(height);
    for _ in 0..height {
        final_image.push(vec![2; width]);
    }

    let layer_size = width * height;
    let n_layers = image_data.len() / layer_size;

    for layer_i in 0..n_layers {
        for height_i in 0..height {
            let row_i = height_i * width + layer_size * layer_i;

            for pixel_i in row_i..row_i + width {
                let pixel_value = image_data[pixel_i];

                if final_image[height_i][pixel_i % width] == 2 {
                    final_image[height_i][pixel_i % width] = pixel_value;
                }
            }
        }
    }

    for row in &final_image {
        for c in row {
            let to_print = {
                if *c == 1 {
                    '1'
                } else {
                    ' '
                }
            };

            print!("{} ", to_print);
        }
        println!();
    }
    println!();
}

fn p1(lines: Vec<String>) {
    let image_data: Vec<u8> = lines[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let width = 25;
    let height = 6;

    let layer_size = width * height;
    let n_layers = image_data.len() / layer_size;

    let mut fewest_zeroes = u32::MAX;
    let mut fewest_zeroes_layer_ans = 0;

    for layer_i in 0..n_layers {
        let mut num_counts: [u32; 3] = [0, 0, 0];

        for height_i in 0..height {
            let row_index = height_i * width + layer_size * layer_i;

            for pixel_i in row_index..row_index + width {
                let pixel_value = image_data[pixel_i];

                if pixel_value <= 2 {
                    num_counts[pixel_value as usize] += 1;
                }
            }
        }

        if num_counts[0] < fewest_zeroes {
            fewest_zeroes = num_counts[0];
            fewest_zeroes_layer_ans = num_counts[1] * num_counts[2];
        }
    }

    println!("{}", fewest_zeroes_layer_ans);
}

pub fn d08() {
    let input_file = Path::new("src/d08/in.txt");

    // p1(get_lines(input_file));
    p2(get_lines(input_file));
}

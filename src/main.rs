#![warn(clippy::pedantic, clippy::nursery)]

use std::env::args;

use image::{imageops::FilterType, io::Reader};

#[allow(clippy::cast_precision_loss, clippy::cast_lossless)]
fn main() {
    let squares = [
        (":black_large_square:", "31373D"),
        (":red_square:", "DD2E44"),
        (":orange_square:", "F49019"),
        (":yellow_square:", "FDCB59"),
        (":green_square:", "78B159"),
        (":blue_square:", "56ACEE"),
        (":purple_square:", "AB8ED6"),
        (":brown_square:", "C1694F"),
        (":white_large_square:", "E6E7E8"),
    ].map(|(text, colour)| {
        (
            text,
            colour
                .chars()
                .collect::<Vec<_>>()
                .chunks_exact(2)
                .map(|chunk| i32::from_str_radix(chunk.iter().collect::<String>().as_str(), 16).unwrap())
                .collect::<Vec<_>>(),
        )
    });
    let image = Reader::open(args().nth(1).unwrap())
        .unwrap()
        .decode()
        .unwrap()
        .resize(
            args().nth(2).unwrap().parse().unwrap(),
            Reader::open(args().nth(1).unwrap())
                .unwrap()
                .decode()
                .unwrap()
                .height(),
            FilterType::Lanczos3,
        )
        .to_rgb8();
    let pixels: Vec<_> = image.pixels().collect();
    let pixels = pixels.chunks_exact(image.width() as usize).map(|row| {
        row.iter().map(|pixel| {
            squares.iter().map(|(text, colour)| {
                (
                    text,
                    colour
                        .iter()
                        .zip(pixel.0.iter())
                        .map(|(a, b): (&i32, &u8)| a.abs_diff(*b as i32))
                        .sum::<u32>(),
                )
            }).min_by_key(|(_, diff)| *diff).unwrap().0
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    for row in pixels {
        for text in row {
            print!("{text}");
        }
        println!();
    }
}

/**
 * Flood Fill
 * https://leetcode.com/problems/flood-fill/
 */

pub fn flood_fill(image: &mut Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    if image[sr as usize][sc as usize] != color {
        image[sr as usize][sc as usize] = color;
        if sr > 0 {
            flood_fill(image, sr - 1, sc, color);
        }
        if ((sr as usize) + 1) < image.len() {
            flood_fill(image, sr + 1, sc, color);
        }
        if sc > 0 {
            flood_fill(image, sr, sc - 1, color);
        }
        if ((sc as usize) + 1) < image[sr as usize].len() {
            flood_fill(image, sr, sc + 1, color);
        }
    }

    return image;
}

fn main() {
    println!("Hello, world!");
    let pixels: &mut Vec<Vec<i32>> = Vec::from([
        Vec::from([1, 1, 1]),
        Vec::from([1, 1, 0]),
        Vec::from([1, 0, 1]),
    ]);

    println!("{:?}", pixels);
    println!("Flood filled Pixels = {:?}", flood_fill(pixels, 1, 1, 2));
}

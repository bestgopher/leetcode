#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut image = image;
        for i in 0..image.len() {
            let (mut s, mut m) = (0, image[0].len() - 1);

            while s <= m {
                if s == m {
                    image[i][s] ^= 1;
                    break;
                }

                image[i].swap(s, m);
                image[i][s] ^= 1;
                image[i][m] ^= 1;
                s += 1;
                m -= 1;
            }
        }

        image
    }
}

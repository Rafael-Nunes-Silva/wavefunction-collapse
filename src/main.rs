// struct Tile<T>(pub T);

use std::{collections::HashMap, io::Cursor};

use image::{io::Reader, GenericImageView};
use rand::Rng;

#[derive(Clone)]
struct Rule<T> {
    tile: T,
    up: Vec<T>,
    right: Vec<T>,
    down: Vec<T>,
    left: Vec<T>,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum TileTypes {
    Blank,
    Up,
    Right,
    Down,
    Left,
}

fn main() {
    // let blank = Tile(TileTypes::Blank);
    // let up = Tile(TileTypes::Up);
    // let right = Tile(TileTypes::Right);
    // let down = Tile(TileTypes::Down);
    // let left = Tile(TileTypes::Left);

    // define tiles
    let blank = TileTypes::Blank;
    let up = TileTypes::Up;
    let right = TileTypes::Right;
    let down = TileTypes::Down;
    let left = TileTypes::Left;

    // define rules
    let blank_rule = Rule {
        tile: blank,
        up: vec![blank, up],
        right: vec![blank, right],
        down: vec![blank, down],
        left: vec![blank, left],
    };
    let up_rule = Rule {
        tile: up,
        up: vec![right, down, left],
        right: vec![up, down, left],
        down: vec![blank, down],
        left: vec![up, right, down],
    };
    let right_rule = Rule {
        tile: right,
        up: vec![right, down, left],
        right: vec![up, down, left],
        down: vec![up, right, left],
        left: vec![blank, left],
    };
    let down_rule = Rule {
        tile: down,
        up: vec![blank, up],
        right: vec![up, down, left],
        down: vec![up, right, left],
        left: vec![up, right, down],
    };
    let left_rule = Rule {
        tile: left,
        up: vec![right, down, left],
        right: vec![blank, right],
        down: vec![up, right, left],
        left: vec![up, right, down],
    };

    // map tiles to it's rules
    let mut rules_map = HashMap::with_capacity(5);
    rules_map.insert(blank, blank_rule.clone());
    rules_map.insert(up, up_rule.clone());
    rules_map.insert(right, right_rule.clone());
    rules_map.insert(down, down_rule.clone());
    rules_map.insert(left, left_rule.clone());

    // create the array to hold the tiles
    let width = 16;
    let height = 16;
    let mut tiles: Vec<Option<TileTypes>> = Vec::with_capacity(width * height);

    // initialize the array of tiles with Nones
    for _y in 0..height {
        for _x in 0..width {
            tiles.push(None);
        }
    }

    let mut collapse_stack: Vec<(usize, usize)> = Vec::new();

    let possible_tiles = [blank, up, right, down, left];

    let mut rng = rand::thread_rng();
    let start_x = rng.gen::<usize>() % width;
    let start_y = rng.gen::<usize>() % height;
    let start_tile_index = rng.gen::<usize>() % possible_tiles.len();

    collapse_stack.push((start_x, start_y));

    let mut first_tile = true;
    while let Some((x, y)) = collapse_stack.pop() {
        let i = width * y + x;

        let mut top_possibilities: Vec<TileTypes> = Vec::new();
        if y > 0 {
            let i_up = i - width;
            if let Some(tile) = tiles[i_up] {
                top_possibilities = rules_map.get(&tile).unwrap().clone().down;
            } else {
                collapse_stack.push((x, y - 1));
            }
        }

        let mut right_possibilities: Vec<TileTypes> = Vec::new();
        if x < width - 1 {
            let i_right = i + 1;
            if let Some(tile) = tiles[i_right] {
                right_possibilities = rules_map.get(&tile).unwrap().clone().left;
            } else {
                collapse_stack.push((x + 1, y));
            }
        }

        let mut down_possibilities: Vec<TileTypes> = Vec::new();
        if y < height - 1 {
            let i_down = i + width;
            if let Some(tile) = tiles[i_down] {
                down_possibilities = rules_map.get(&tile).unwrap().clone().up;
            } else {
                collapse_stack.push((x, y + 1));
            }
        }

        let mut left_possibilities: Vec<TileTypes> = Vec::new();
        if x > 0 {
            let i_left = i - 1;
            if let Some(tile) = tiles[i_left] {
                left_possibilities = rules_map.get(&tile).unwrap().clone().right
            } else {
                collapse_stack.push((x - 1, y));
            }
        }

        let possibilities: Vec<TileTypes> = [
            top_possibilities.clone(),
            right_possibilities.clone(),
            down_possibilities.clone(),
            left_possibilities.clone(),
        ]
        .concat()
        .iter()
        .filter(|p| {
            if top_possibilities.len() > 0 && !top_possibilities.contains(p) {
                return false;
            }
            if right_possibilities.len() > 0 && !right_possibilities.contains(p) {
                return false;
            }
            if down_possibilities.len() > 0 && !down_possibilities.contains(p) {
                return false;
            }
            if left_possibilities.len() > 0 && !left_possibilities.contains(p) {
                return false;
            }

            true
        })
        .cloned()
        .collect();

        if possibilities.len() > 0 {
            let tile_index = rng.gen::<usize>() % possibilities.len();
            tiles[i] = Some(possibilities[tile_index]);
        } else if first_tile {
            first_tile = false;
            tiles[i] = Some(possible_tiles[start_tile_index]);
        }
    }

    let blank_img = Reader::open("images/blank.png")
        .unwrap()
        .decode()
        .unwrap();
    let up_img = Reader::open("images/up.png")
        .unwrap()
        .decode()
        .unwrap();
    let right_img = Reader::open("images/right.png")
        .unwrap()
        .decode()
        .unwrap();
    let down_img = Reader::open("images/down.png")
        .unwrap()
        .decode()
        .unwrap();
    let left_img = Reader::open("images/left.png")
        .unwrap()
        .decode()
        .unwrap();

    let tile_width = blank_img.width() as usize;
    let tile_height = blank_img.height() as usize;

    let bytes_len = width * tile_width * height * tile_height * 4;
    let mut bytes: Vec<u8> = Vec::with_capacity(bytes_len);
    unsafe {
        bytes.set_len(bytes_len);
    }
    bytes.fill(0 as u8);

    for y in 0..height {
        for x in 0..width {
            let x_offset = x * tile_width;
            let y_offset = y * tile_height;

            let tile_i = width * y + x;
            for tile_y in 0..tile_height {
                for tile_x in 0..tile_width {
                    let real_y = y_offset + tile_y;
                    let real_x = x_offset + tile_x;

                    let byte_i = (width * tile_width * real_y + real_x) * 4;
                    if let Some(tile) = tiles[tile_i] {
                        match tile {
                            TileTypes::Blank => {
                                let pixel = blank_img.get_pixel(tile_x as u32, tile_y as u32);
                                bytes[byte_i] = pixel[0];
                                bytes[byte_i + 1] = pixel[1];
                                bytes[byte_i + 2] = pixel[2];
                                bytes[byte_i + 3] = pixel[3];
                            }
                            TileTypes::Up => {
                                let pixel = up_img.get_pixel(tile_x as u32, tile_y as u32);
                                bytes[byte_i] = pixel[0];
                                bytes[byte_i + 1] = pixel[1];
                                bytes[byte_i + 2] = pixel[2];
                                bytes[byte_i + 3] = pixel[3];
                            }
                            TileTypes::Right => {
                                let pixel = right_img.get_pixel(tile_x as u32, tile_y as u32);
                                bytes[byte_i] = pixel[0];
                                bytes[byte_i + 1] = pixel[1];
                                bytes[byte_i + 2] = pixel[2];
                                bytes[byte_i + 3] = pixel[3];
                            }
                            TileTypes::Down => {
                                let pixel = down_img.get_pixel(tile_x as u32, tile_y as u32);
                                bytes[byte_i] = pixel[0];
                                bytes[byte_i + 1] = pixel[1];
                                bytes[byte_i + 2] = pixel[2];
                                bytes[byte_i + 3] = pixel[3];
                            }
                            TileTypes::Left => {
                                let pixel = left_img.get_pixel(tile_x as u32, tile_y as u32);
                                bytes[byte_i] = pixel[0];
                                bytes[byte_i + 1] = pixel[1];
                                bytes[byte_i + 2] = pixel[2];
                                bytes[byte_i + 3] = pixel[3];
                            }
                        }
                    }
                }
            }
        }
    }

    image::save_buffer(
        "new_output.png",
        &bytes,
        (width * tile_width) as u32,
        (height * tile_height) as u32,
        image::ColorType::Rgba8,
    )
    .unwrap();
}

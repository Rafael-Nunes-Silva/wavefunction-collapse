use std::collections::HashMap;

use image::{io::Reader, GenericImageView};
use wavefunction_collapse::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
enum TileTypes {
    Blank,
    URL,
    URB,
    RBL,
    UBL,
}

fn main() {
    let blank = Tile::new(TileTypes::Blank, vec![0], vec![0], vec![0], vec![0]);
    let url = Tile::new(TileTypes::URL, vec![1], vec![1], vec![0], vec![1]);
    let urb = url.rotate_90(TileTypes::URB);
    let rbl = url.rotate_180(TileTypes::RBL);
    let ubl = url.rotate_270(TileTypes::UBL);

    let model = TileModel::<TileTypes, u8>::new(vec![blank, url, urb, rbl, ubl]);

    let width = 4;
    let height = 4;
    let tileset = model.generate(width, height, 52838673823);

    let blank_img = Reader::open("images/blank.png").unwrap().decode().unwrap();
    let up_img = Reader::open("images/up.png").unwrap().decode().unwrap();
    let right_img = Reader::open("images/right.png").unwrap().decode().unwrap();
    let down_img = Reader::open("images/down.png").unwrap().decode().unwrap();
    let left_img = Reader::open("images/left.png").unwrap().decode().unwrap();

    let tile_width = blank_img.width() as usize;
    let tile_height = blank_img.height() as usize;

    let mut img_map = HashMap::with_capacity(5);
    img_map.insert(TileTypes::Blank, blank_img);
    img_map.insert(TileTypes::URL, up_img);
    img_map.insert(TileTypes::URB, right_img);
    img_map.insert(TileTypes::RBL, down_img);
    img_map.insert(TileTypes::UBL, left_img);

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
                    if let Some(tile) = &tileset[tile_i] {
                        let pixel = img_map
                            .get(&tile.tile)
                            .unwrap()
                            .get_pixel(tile_x as u32, tile_y as u32);

                        bytes[byte_i] = pixel[0];
                        bytes[byte_i + 1] = pixel[1];
                        bytes[byte_i + 2] = pixel[2];
                        bytes[byte_i + 3] = pixel[3];
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

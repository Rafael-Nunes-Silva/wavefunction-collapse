use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Clone)]
pub struct Tile<T, U>
where
    T: Clone,
    U: Clone + PartialEq,
{
    pub tile: T,
    up: Vec<U>,
    right: Vec<U>,
    down: Vec<U>,
    left: Vec<U>,
}
impl<T: Clone, U: Clone + PartialEq> Tile<T, U> {
    pub fn new(tile: T, up: Vec<U>, right: Vec<U>, down: Vec<U>, left: Vec<U>) -> Self {
        Self {
            tile,
            up,
            right,
            down,
            left,
        }
    }

    pub fn rotate_90(&self, tile: T) -> Self {
        Self {
            tile: tile,
            up: self.left.clone(),
            right: self.up.clone(),
            down: self.right.clone(),
            left: self.down.clone(),
        }
    }

    pub fn rotate_180(&self, tile: T) -> Self {
        Self {
            tile: tile,
            up: self.down.clone(),
            right: self.left.clone(),
            down: self.up.clone(),
            left: self.right.clone(),
        }
    }

    pub fn rotate_270(&self, tile: T) -> Self {
        Self {
            tile: tile,
            up: self.right.clone(),
            right: self.down.clone(),
            down: self.left.clone(),
            left: self.up.clone(),
        }
    }
}

pub struct TileModel<T, U>
where
    T: Clone,
    U: Clone + PartialEq,
{
    // rules_map: HashMap<T, Rule<T>>,
    tiles: Vec<Tile<T, U>>,
}
impl<T: Clone, U: Clone + PartialEq> TileModel<T, U> {
    pub fn new(tiles: Vec<Tile<T, U>>) -> Self {
        Self { tiles }
    }

    pub fn generate(&self, width: usize, height: usize, seed: u64) -> Vec<Option<Tile<T, U>>> {
        let mut tileset: Vec<Option<Tile<T, U>>> = Vec::with_capacity(width * height);
        for _y in 0..height {
            for _x in 0..width {
                tileset.push(None);
            }
        }

        let mut collapse_stack: Vec<(usize, usize)> = Vec::with_capacity(width * height);

        let mut rng = StdRng::seed_from_u64(seed);
        let start_x = rng.gen::<usize>() % width;
        let start_y = rng.gen::<usize>() % height;
        let start_tile_index = rng.gen::<usize>() % self.tiles.len();

        collapse_stack.push((start_x, start_y));

        let mut first_tile = true;
        while let Some((x, y)) = collapse_stack.pop() {
            let i = width * y + x;

            if let Some(_) = &tileset[i] {
                continue;
            }

            let mut up_tile = None;
            if y > 0 {
                let i_up = i - width;
                if let Some(tile) = &tileset[i_up] {
                    up_tile = Some(tile.clone());
                } else {
                    collapse_stack.push((x, y - 1));
                }
            }

            let mut right_tile = None;
            if x < width - 1 {
                let i_right = i + 1;
                if let Some(tile) = &tileset[i_right] {
                    right_tile = Some(tile.clone());
                } else {
                    collapse_stack.push((x + 1, y));
                }
            }

            let mut down_tile = None;
            if y < height - 1 {
                let i_down = i + width;
                if let Some(tile) = &tileset[i_down] {
                    down_tile = Some(tile.clone());
                } else {
                    collapse_stack.push((x, y + 1));
                }
            }

            let mut left_tile = None;
            if x > 0 {
                let i_left = i - 1;
                if let Some(tile) = &tileset[i_left] {
                    left_tile = Some(tile.clone());
                } else {
                    collapse_stack.push((x - 1, y));
                }
            }

            let possibilities: Vec<Tile<T, U>> = self
                .tiles
                .iter()
                .filter(|tile| {
                    let mut match_up = false;
                    if let Some(up) = &up_tile {
                        tile.up.iter().for_each(|m| {
                            if up.down.contains(m) {
                                match_up = true;
                            }
                        });
                    } else {
                        match_up = true;
                    }

                    let mut match_right = false;
                    if let Some(right) = &right_tile {
                        tile.right.iter().for_each(|m| {
                            if right.left.contains(m) {
                                match_right = true;
                            }
                        });
                    } else {
                        match_right = true;
                    }

                    let mut match_down = false;
                    if let Some(down) = &down_tile {
                        tile.down.iter().for_each(|m| {
                            if down.up.contains(m) {
                                match_down = true;
                            }
                        });
                    } else {
                        match_down = true;
                    }

                    let mut match_left = false;
                    if let Some(left) = &left_tile {
                        tile.left.iter().for_each(|m| {
                            if left.right.contains(m) {
                                match_left = true;
                            }
                        });
                    } else {
                        match_left = true;
                    }

                    match_up && match_right && match_down && match_left
                })
                .cloned()
                .collect();

            if possibilities.len() > 0 {
                let tile_index = rng.gen::<usize>() % possibilities.len();
                tileset[i] = Some(possibilities[tile_index].clone());
            } else if first_tile {
                first_tile = false;
                tileset[i] = Some(self.tiles[start_tile_index].clone());
            }
        }

        println!("collapse_stack capacity: {}", collapse_stack.capacity());

        tileset
    }
}

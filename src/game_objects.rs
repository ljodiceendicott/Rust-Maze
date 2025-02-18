// use bracket_lib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
    pub projectiles: Vec<Proj>,
}

pub struct Proj {
    pub x: i32,
    pub y: i32,
    pub movement: Direction,
    pub nextx: i32,
    pub nexty: i32,
}

impl Proj {
    pub fn new(x: i32, y: i32, dir: Direction) -> Self {
        Self {
            x,
            y,
            movement: dir,
            nextx: x,
            nexty: y,
        }
    }

    pub fn update(&mut self) {
        self.x = self.nextx;
        self.y = self.nexty;

        match self.movement {
            Direction::Left => self.nextx -= 1,
            Direction::Right => self.nextx += 1,
            Direction::Up => self.nexty -= 1,
            Direction::Down => self.nexty += 1,
        }
    }
}

// Map tile types
#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    OuterWall,
    Wall,
    Floor,
    Proj,
}

// Game map
pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let mut tiles = vec![vec![TileType::Floor; height as usize]; width as usize];
        
        // Add walls around the map borders
        for x in 0..width {
            for y in 0..height {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    tiles[x as usize][y as usize] = TileType::OuterWall;
                }
            }
        }
        
        // Add some inner walls
        tiles[5][5] = TileType::Wall;
        tiles[5][6] = TileType::Wall;
        tiles[5][7] = TileType::Wall;
        
        Map {
            tiles,
            width,
            height,
        }
    }

    pub fn can_enter_tile(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return false;
        }
        self.tiles[x as usize][y as usize] != TileType::Wall
    }
} 
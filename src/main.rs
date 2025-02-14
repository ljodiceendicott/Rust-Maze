use std::usize;

use bracket_lib::prelude::*;

// Player struct
struct Player {
    x: i32,
    y: i32,
}

enum Direction{
    Up,
    Down,
    Left,
    Right,
}

struct Proj {
    x: i32,
    y: i32,
    movment: Direction,
}

// Map tile types
#[derive(Clone, Copy, PartialEq)]
enum TileType {
    OuterWall,
    Wall,
    Floor,
    Proj,
}

// Game map
struct Map {
    tiles: Vec<Vec<TileType>>,
    width: i32,
    height: i32,
}

impl Map {
    // Create a simple map with walls around the edges
    fn new(width: i32, height: i32) -> Self {
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

    // Check if a tile is walkable
    fn can_enter_tile(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return false;
        }
        self.tiles[x as usize][y as usize] != TileType::Wall
    }
}

// Game state
struct State {
    player: Player,
    map: Map,
}

impl State {
    fn new() -> Self {
        State {
            player: Player { x: 10, y: 10 },
            map: Map::new(30, 30),
        }
    }
}

// Implement the GameState trait for our state
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Handle input
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::A => {
                    if self.map.can_enter_tile(self.player.x - 1, self.player.y) {
                        self.player.x -= 1;
                    }
                }
                VirtualKeyCode::D => {
                    if self.map.can_enter_tile(self.player.x + 1, self.player.y) {
                        self.player.x += 1;
                    }
                }
                VirtualKeyCode::W => {
                    if self.map.can_enter_tile(self.player.x, self.player.y - 1) {
                        self.player.y -= 1;
                    }
                }
                VirtualKeyCode::S => {
                    if self.map.can_enter_tile(self.player.x, self.player.y + 1) {
                        self.player.y += 1;
                    }
                }
                //arrow keys for projectile movement
                VirtualKeyCode::Up => {
                  self.map.tiles[self.player.x as usize][(self.player.y-1) as usize] = TileType::Proj;
                }
                _ => {}
            }
        }

        // Clear the screen
        ctx.cls();

        // Draw the map
        for x in 0..self.map.width {
            for y in 0..self.map.height {
                let glyph = match self.map.tiles[x as usize][y as usize] {
                    TileType::Wall => '#',
                    TileType::Floor => '.',
                    TileType::OuterWall =>'.',
                    TileType::Proj => '°',
                };
                ctx.set(x, y, RGB::named(WHITE), RGB::named(BLACK), glyph);
            }
        }

        // Draw the player
        ctx.set(
            self.player.x,
            self.player.y,
            RGB::named(GREEN),
            RGB::named(BLACK),
            '@',
        );

        // Add text at the bottom of the screen
        let bottom_text = format!("Player Position: ({}, {})", self.player.x, self.player.y);
        ctx.print_color(
            1, // X position (left-aligned)
            self.map.height, // Y position (bottom of the screen)
            RGB::named(WHITE),
            RGB::named(BLACK),
            bottom_text,
        );

        let movementup: String = format!("Move Forward => W");
        ctx.print_color(
            1, // X position (left-aligned)
            self.map.height+1, // Y position (bottom of the screen)
            RGB::named(WHITE),
            RGB::named(BLACK),
            movementup,
        );
        let movement2: String = format!("Move Backward => S");
        ctx.print_color(
            1, // X position (left-aligned)
            self.map.height+3, // Y position (bottom of the screen)
            RGB::named(WHITE),
            RGB::named(BLACK),
            movement2,
        );
        let movement3: String = format!("Move Left => A");
        ctx.print_color(
            1, // X position (left-aligned)
            self.map.height+5, // Y position (bottom of the screen)
            RGB::named(WHITE),
            RGB::named(BLACK),
            movement3,
        );
        let movement4: String = format!("Move Right => D");
        ctx.print_color(
            1, // X position (left-aligned)
            self.map.height+7, // Y position (bottom of the screen)
            RGB::named(WHITE),
            RGB::named(BLACK),
            movement4,
        );
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(30, 40)?
        .with_title("Rusty Rust ")
        .with_fps_cap(45.0)
        .build()?;
    

    main_loop(context, State::new())
}
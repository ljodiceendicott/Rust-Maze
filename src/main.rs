use std::usize;

use bracket_lib::prelude::*;
mod game_objects;
use game_objects::*;

// Game state
struct State {
    player: Player,
    map: Map,  // This will now use Map from game_objects
}

impl State {
    fn new() -> Self {
        State {
            player: Player { 
                x: 10, 
                y: 10, 
                facing: Direction::Up,
                projectiles: Vec::new(),
            },
            map: Map::new(30, 30),  // This will now use the wall initialization from game_objects.rs
        }
    }
}

// Implement the GameState trait for our state
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Handle input
        if let Some(key) = ctx.key {
            match key {
                // Movement controls (WASD)
                VirtualKeyCode::A => {
                    if self.map.can_enter_tile(self.player.x - 1, self.player.y) {
                        self.player.x -= 1;
                    }
                    self.player.facing = Direction::Left;
                }
                VirtualKeyCode::D => {
                    if self.map.can_enter_tile(self.player.x + 1, self.player.y) {
                        self.player.x += 1;
                    }
                    self.player.facing = Direction::Right;
                }
                VirtualKeyCode::W => {
                    if self.map.can_enter_tile(self.player.x, self.player.y - 1) {
                        self.player.y -= 1;
                    }
                    self.player.facing = Direction::Up;
                }
                VirtualKeyCode::S => {
                    if self.map.can_enter_tile(self.player.x, self.player.y + 1) {
                        self.player.y += 1;
                    }
                    self.player.facing = Direction::Down;
                }
                // Shooting controls (Arrow keys)
                VirtualKeyCode::Left => {
                    let proj = Proj::new(self.player.x, self.player.y, Direction::Left);
                    if self.map.can_enter_tile(proj.nextx, proj.nexty) {
                        self.map.tiles[proj.nextx as usize][proj.nexty as usize] = TileType::Proj;
                        self.player.projectiles.push(proj);
                    }
                }
                VirtualKeyCode::Right => {
                    let proj = Proj::new(self.player.x, self.player.y, Direction::Right);
                    if self.map.can_enter_tile(proj.nextx, proj.nexty) {
                        self.map.tiles[proj.nextx as usize][proj.nexty as usize] = TileType::Proj;
                        self.player.projectiles.push(proj);
                    }
                }
                VirtualKeyCode::Up => {
                    let proj = Proj::new(self.player.x, self.player.y, Direction::Up);
                    if self.map.can_enter_tile(proj.nextx, proj.nexty) {
                        self.map.tiles[proj.nextx as usize][proj.nexty as usize] = TileType::Proj;
                        self.player.projectiles.push(proj);
                    }
                }
                VirtualKeyCode::Down => {
                    let proj = Proj::new(self.player.x, self.player.y, Direction::Down);
                    if self.map.can_enter_tile(proj.nextx, proj.nexty) {
                        self.map.tiles[proj.nextx as usize][proj.nexty as usize] = TileType::Proj;
                        self.player.projectiles.push(proj);
                    }
                }
                _ => {}
            }
        }

        // Update all projectiles
        let mut i = 0;
        while i < self.player.projectiles.len() {
            let proj = &mut self.player.projectiles[i];
            
            // Clear the current projectile position
            self.map.tiles[proj.x as usize][proj.y as usize] = TileType::Floor;
            
            // Update the projectile position
            proj.update();
            
            // Collision check
                if proj.nextx < 0 || proj.nextx >= self.map.width || 
                   proj.nexty < 0 || proj.nexty >= self.map.height ||
                   self.map.tiles[proj.nextx as usize][proj.nexty as usize] == TileType::Wall {
                // Clear the current position before removing
                self.map.tiles[proj.x as usize][proj.y as usize] = TileType::Floor;
                // Drop the mutable reference before removing
                let _ = proj;
                // Remove the projectile
                self.player.projectiles.remove(i);
            } 
            //
            else {
                // Set the new projectile position in the map
                self.map.tiles[proj.nextx as usize][proj.nexty as usize] = TileType::Proj;
                i += 1;
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
                 //a projectile is in the map
                
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
            self.map.width+1, // X position (left-aligned)
            1, // Y position (bottom of the screen)
            RGB::named(WHITE),
            RGB::named(BLACK),
            bottom_text,
        );

        let controls_text = [
            "Movement Controls:",
            "W - Move Up",
            "S - Move Down",
            "A - Move Left",
            "D - Move Right",
            "",
            "Shooting Controls:",
            "↑ - Shoot Up",
            "↓ - Shoot Down",
            "← - Shoot Left",
            "→ - Shoot Right",
        ];

        for (i, text) in controls_text.iter().enumerate() {
            ctx.print_color(
                self.map.width + 1,
                3 + i as i32,
                RGB::named(WHITE),
                RGB::named(BLACK),
                text,
            );
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(80, 50)?
        .with_title("Rusty Rust ")
        .with_fps_cap(45.0)
        .build()?;
    

    main_loop(context, State::new())
}
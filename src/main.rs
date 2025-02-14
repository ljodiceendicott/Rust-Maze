use std::io::{self, Write};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Wall,
    Path,
}

struct Maze {
    grid: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let mut grid = vec![vec![Tile::Wall; width]; height];

        // Simple random maze generation (can be customized later)
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                // if rng.gen_bool(0.3) {
                grid[y][x] = Tile::Path;
                //}
            }
        }

        // Ensure starting and ending points are paths
        grid[1][1] = Tile::Path;
        grid[height - 2][width - 2] = Tile::Path;

        Maze {
            grid,
            width,
            height,
        }
    }

    //fn display(&self) {
    //  println!("Maze (Player 'P' represents your position):");
    //   for row in &self.grid {
    //     for tile in row {
    //         let symbol = match tile {
    //           Tile::Wall => "#",
    //         Tile::Path => " ",
    //   };
    // print!("{}", symbol);
    //  }
    //  println!();
    //  }
    // }
}

struct Game {
    maze: Maze,
    player_x: usize,
    player_y: usize,
}

impl Game {
    fn new(maze: Maze) -> Self {
        Game {
            maze,
            player_x: 1,
            player_y: 1,
        }
    }

    fn render(&self) {
        println!();
        println!();
        println!();
        println!();
        println!();
        println!();
        println!();
        println!();

        println!();
        for y in 0..self.maze.height {
            for x in 0..self.maze.width {
                if self.player_x == x && self.player_y == y {
                    print!("P"); // Player's position
                } else {
                    match self.maze.grid[y][x] {
                        Tile::Wall => print!("#"),
                        Tile::Path => print!(" "),
                    }
                }
            }
            println!();
        }
        println!();
    }

    fn move_player(&mut self, dx: isize, dy: isize) {
        let new_x = self.player_x as isize + dx;
        let new_y = self.player_y as isize + dy;

        if new_x >= 0
            && new_x < self.maze.width as isize
            && new_y >= 0
            && new_y < self.maze.height as isize
            && self.maze.grid[new_y as usize][new_x as usize] == Tile::Path
        {
            self.player_x = new_x as usize;
            self.player_y = new_y as usize;
        }
    }

    fn play(&mut self) {
        loop {
            self.render();
            println!();
            println!();
            println!();
            println!();
            println!("Use WASD to move (Q to quit):");
            let mut input = String::new();
            io::stdout().flush().unwrap(); // Flush to ensure prompt is displayed

            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            match input.as_str() {
                "q" => break,                   // Quit the game
                "w" => self.move_player(0, -1), // Up
                "a" => self.move_player(-1, 0), // Left
                "s" => self.move_player(0, 1),  // Down
                "d" => self.move_player(1, 0),  // Right
                _ => println!("Invalid input. Please press W, A, S, D, or Q."),
            }
        }
    }
}

fn main() {
    let maze = Maze::new(30, 15); // Customize maze size here
    let mut game = Game::new(maze);

    game.play();
}

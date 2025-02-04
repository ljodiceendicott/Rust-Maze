use std::{
    io::{self, Write},
    usize::MIN,
};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tile {
    Wall,
    Empty,
    Start,
    End,
}
//Write code so that both are coming from the same object of Maze then have 2 values, being
//Fixed-maze & Custom-Maze
//Could also seperate this into another file maybe later
struct Maze {
    grid: Vec<Vec<Tile>>,
    start: (usize, usize),
    end: (usize, usize),
}

struct CustomMaze {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
}
impl Maze {
    fn new() -> Self {
        let grid = vec![
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
            vec![
                Tile::Wall,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Wall,
            ],
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Start, Tile::Empty, Tile::End, Tile::Wall],
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
        ];

        let start = (3, 1);
        let end = (3, 3);

        Maze { grid, start, end }
    }
    fn print(&self) {
        for row in &self.grid {
            for tile in row {
                let symbol = match tile {
                    Tile::Wall => "#",
                    Tile::Empty => " ",
                    Tile::Start => "S",
                    Tile::End => "E",
                };
                print!("{}", symbol);
            }
            println!();
        }
    }

    fn is_valid_move(&self, x: usize, y: usize) -> bool {
        x < self.grid.len() && y < self.grid[0].len() && self.grid[x][y] != Tile::Wall
    }
}

impl CustomMaze {
    fn new() -> Self {
        let grid = vec![
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', 'S', ' ', ' ', '#', ' ', ' ', ' ', '#'],
            vec!['#', ' ', '#', ' ', '#', ' ', '#', ' ', '#'],
            vec!['#', ' ', '#', ' ', '#', ' ', '#', ' ', '#'],
            vec!['#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'],
            vec!['#', ' ', '#', '#', '#', '#', '#', ' ', '#'],
            vec!['#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'],
            vec!['#', ' ', '#', '#', ' ', '#', '#', ' ', '#'],
            vec!['#', ' ', ' ', ' ', ' ', ' ', ' ', 'E', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
        ];

        let mut start = (MIN, MIN);
        let mut end = (MIN, MIN);

        //This is so that when new maps are added, you can find the start and end
        for (i, row) in grid.iter().enumerate() {
            for (y, &cell) in row.iter().enumerate() {
                if cell == 'S' {
                    start = (i, y);
                } else if cell == 'E' {
                    end = (i, y);
                }
            }
        }

        CustomMaze { grid, start, end }
    }

    fn printalt(&self) {
        for row in &self.grid {
            for tile in row {
                let symbol = tile;
                print!("{}", symbol);
            }
            println!();
        }
    }
}
struct Player {
    x: usize,
    y: usize,
}

impl Player {
    fn new(start_x: usize, start_y: usize) -> Self {
        Player {
            x: start_x,
            y: start_y,
        }
    }

    fn move_player(&mut self, maze: &Maze, direction: &str) {
        let (dx, dy) = match direction {
            "w" => (-1, 0), // up
            "s" => (1, 0),  // down
            "a" => (0, -1), // left
            "d" => (0, 1),  // right
            _ => (0, 0),
        };

        let new_x = (self.x as isize + dx) as usize;
        let new_y = (self.y as isize + dy) as usize;

        if maze.is_valid_move(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        } else {
            print!("Last Move was invalid, so you were not moved");
        }
    }
}

//Using a method to get the maze instead and returning a maze depending on result
fn get_maze(x: usize) {}

fn main() {
    println!("Hello! Would you like the easy or the hard maze?");
    println!("1.)EZ\n2.HARD");
    io::stdout().flush().unwrap();

    let mut player = Player::new(1, 1); // Starting position
    let mut move_input = String::new();
    io::stdin().read_line(&mut move_input).unwrap();
    let move_input = move_input.trim();

    if move_input == "q" {
        print!("Have a good day!");
    } else if move_input == "1" {
        let maze = Maze::new();
    } else if move_input == "2" {
        let maze = CustomMaze::new();
    }

    loop {
        println!("\nMove the player (w = up, s = down, a = left, d = right, q = quit): ");
        println!("Player Coords {},{}", player.x, player.y);
        print!("Your choice: ");
        io::stdout().flush().unwrap();

        let mut move_input = String::new();
        io::stdin().read_line(&mut move_input).unwrap();
        let move_input = move_input.trim();

        if move_input == "q" {
            break;
        }

        player.move_player(&maze, move_input);

        if (player.x, player.y) == maze.end {
            println!("Congratulations! You've reached the end!");
            break;
        }
        println!("Maze:");
        maze.print();
    }
}

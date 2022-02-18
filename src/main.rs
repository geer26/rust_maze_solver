use uuid::Uuid;
use std::io;
use std::fs;

enum Type{
    Visited,
    Wall,
    Floor,
    Node,
    Entrance,
}

enum Parent{
    None,
    parent(Box<Point>),
}

struct Point{
    x: u32,
    y: u32,
    t: Type,
    id: String,
    parent: Parent,
}

impl Point{
}

struct Maze{
    points: Vec<Point>,
}

impl Maze{
    pub fn read_file(&self){

    }
}

fn main() {
    println!("Maze solver - V0.1");
    println!("Please enter filename to solve!");
    let mut maze = String::new();

    io::stdin()
        .read_line(&mut maze)
        .expect("Failed to read line");

    let maze = maze.to_string();

    let contents = fs::read_to_string(maze)
        .expect("Something went wrong reading the file");
    println!("MAZE:\n{}", contents)
}

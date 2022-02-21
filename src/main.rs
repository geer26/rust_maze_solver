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

    //let maze = maze.;

    //let contents = fs::read_to_string(maze)
    let contents = fs::read_to_string("to_text.txt")
        .expect("Something went wrong reading the file");
    println!("MAZE:\n{}", contents);

    let mut rows = contents.trim().split('\n');
    for (r_index,row) in rows.into_iter().enumerate(){
        /*
        items.iter().enumerate().for_each(|(i, x)| {
            println!("Item {} = {}", i, x);
        })
        */

        for (c_index, column) in row.split("").into_iter().enumerate(){
            println!("ROW - {} : COLUMN - {} :: VALUE - {}", r_index, c_index, column);
        }
        //println!("ROW:");
        //println!("{}", row);
    }
}

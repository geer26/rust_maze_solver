extern crate core;

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
    Parent(String),
}

struct Point{
    x: usize,
    y: usize,
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
    let mut points : Vec<Point> = Vec::new();

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
        for (c_index, column) in row.chars().into_iter().enumerate(){
            println!("ROW - {} : COLUMN - {} :: VALUE - {}", r_index, c_index, column);
            let mut ty: Type;
            if column == '#'{ ty = Type::Wall} else { ty = Type::Floor }
            let p = Point{
                x: r_index,
                y: c_index,
                t: ty,
                id: "123456".to_string(),
                parent: Parent::None
            };
            &points.push(p);
        }
    }

    for p in &points{
        println!("id:{}, x:{}, y:{}, type:{}", &p.id, &p.x, &p.y, match &p.t{
            Wall => "WALL",
            Floor => "FLOOR",
        });
    }

}

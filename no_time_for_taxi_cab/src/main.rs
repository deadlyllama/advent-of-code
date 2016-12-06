use std::env;
use std::process;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(Eq, PartialEq, Debug)]
enum Rotation {
    Left,
    Right
}

#[derive(Eq, PartialEq, Debug)]
struct Movement {
    rotation: Rotation,
    distance: i32,
}

#[derive(Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq)]
struct State {
    dir: Direction,
    pos: Position
}

const INITIAL_STATE: State = State { dir: Direction::North, pos: Position { x: 0, y: 0}};

#[test]
fn test_read_instruction() {
    assert_eq!(read_instruction(String::from("L42")), Some(Movement{rotation: Rotation::Left, distance: 42i32}));
    assert_eq!(read_instruction(String::from("R1")), Some(Movement{rotation: Rotation::Right, distance: 1i32}));
    assert_eq!(read_instruction(String::from("L")), None);
    assert_eq!(read_instruction(String::from("42")), None);
    assert_eq!(read_instruction(String::from("L42,")), None);
}

fn read_instruction(instruction: String) -> Option<Movement> {
    let (rot_string, dist_string) = instruction.split_at(1);
    let chars = instruction.chars();
    let rot = rot_string.chars().next().and_then(|c: char| match c {
        'R' => Some(Rotation::Right),
        'L' => Some(Rotation::Left),
        _   => None,
    });
    let dist: Option<i32> = dist_string.parse::<i32>().ok();
    return match (rot, dist) {
        (Some(r), Some(d)) => Some(Movement{ rotation: r, distance: d}),
        (_, _) => None
    };
}

// fn read_instructions(contents: String) -> [Movement] {
//    // split contents on ", "
//    let split = contents.split_whitespace;
//
//}

fn main() {
    use std::io::{Error, ErrorKind};
    let mut args = env::args();

    let filename = args.nth(1);
    let no_file_error = Error::new(ErrorKind::Other, "No filename supplied");
    let contents = filename.ok_or(no_file_error).and_then(|filename| {
        let path = Path::new(&filename);
        File::open(path)
    }).and_then(|mut file| {
        let mut content_string = String::new();
        file.read_to_string(&mut content_string).map(|x| content_string)
    });

    match contents {
        Ok(contents) => {
            println!("File contents: {}", contents);
        }
        Err(msg) => {
            println!("{}", msg);
        }
    }
}

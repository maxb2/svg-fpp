use geo::GeometryCollection;

use svg::node::element::path::{Command, Data};
use svg::node::element::tag::{Path, Rectangle};
use svg::parser::Event;

struct PaperPieceCollection {
    geoms: GeometryCollection,
    adjacency: Vec<Vec<usize>>,
}

fn main() {
    // let svg =
    // String::from(r#"<path d="M0 0L0 60L60 60L60 0L0 0M10 10L40 1L40 40L10.5 40L10 10"/>"#);

    let path = "drawing.svg";
    let mut content = String::new();
    for event in svg::open(path, &mut content).unwrap() {
        // println!("Event: {:?}", event);
        match event {
            Event::Tag(Path, _, attributes) => {
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();
                for command in data.iter() {
                    match command {
                        Command::Move(_pos, _params) => {
                            println!("Move: {:?} {:?}", _pos, _params);
                        }
                        Command::Line(_pos, _params) => {
                            println!("Line: {:?} {:?}", _pos, _params);
                        }
                        Command::Close => {
                            println!("Close")
                        }
                        _ => {}
                    }
                }
            }
            Event::Tag(Rectangle, _, attributes) => {
                let x: f64 = attributes.get("x").unwrap().parse().unwrap();
                let y = attributes.get("y").unwrap();
                let h = attributes.get("width").unwrap();
                let w = attributes.get("height").unwrap();
                println!("Rect: {:?} {:?} {:?} {:?}", x, y, h, w);
            }
            _ => {}
        }
    }

    println!("Hello, world!");
}

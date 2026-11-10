use geo::{ConvexHull, LineString, MultiPolygon, Polygon};

use svg::node::element::path::{Command, Data};
use svg::node::element::tag::{Path, Rectangle};
use svg::parser::Event;

struct PaperPieceCollection {
    polys: MultiPolygon,
    adjacency: Vec<Vec<usize>>,
}

fn main() {
    // let svg =
    // String::from(r#"<path d="M0 0L0 60L60 60L60 0L0 0M10 10L40 1L40 40L10.5 40L10 10"/>"#);

    let path = "drawing.svg";
    let mut content = String::new();
    let mut polygons = Vec::<Polygon<f32>>::new();
    for event in svg::open(path, &mut content).unwrap() {
        // println!("Event: {:?}", event);
        match event {
            Event::Tag(Path, _, attributes) => {
                let mut _poly = Vec::<(f32, f32)>::new();
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();
                let mut is_closed = true;
                for command in data.iter() {
                    match command {
                        Command::Move(_pos, _params) => {
                            println!("Move: {:?} {:?}", _pos, _params);
                            if !is_closed {
                                panic!("Started a new Move without previous being closed.");
                            };
                            is_closed = false;
                            // TODO: write this in a better way
                            // for e in _params.chunks(2).cycle().take(_params.len()/2+1) {
                            //    _poly.push((e[0], e[1]));
                            // };
                            // polygons.push(Polygon::<f32>::new(
                            //     geo::LineString::<f32>::from(_poly.clone()),
                            //     vec![]
                            // ));
                            polygons.push(Polygon::<f32>::new(
                                geo::LineString::<f32>::from(
                                    _params
                                        .chunks_exact(2)
                                        .chain(_params.get(..2))
                                        .map(|c| (c[0], c[1]))
                                        .collect::<Vec<(f32, f32)>>(),
                                ),
                                vec![],
                            ));
                        }
                        Command::Line(_pos, _params) => {
                            println!("Line: {:?} {:?}", _pos, _params);
                            if !is_closed {
                                panic!("Started a new Line without previous being closed.");
                            };
                            is_closed = false;
                        }
                        Command::Close => {
                            println!("Close");
                            is_closed = true;
                        }
                        _ => {}
                    }
                }
                if !is_closed {
                    panic!("Finished path without being closed.")
                };
            }
            Event::Tag(Rectangle, _, attributes) => {
                let x: f32 = attributes.get("x").unwrap().parse().unwrap();
                let y: f32 = attributes.get("y").unwrap().parse().unwrap();
                let h: f32 = attributes.get("width").unwrap().parse().unwrap();
                let w: f32 = attributes.get("height").unwrap().parse().unwrap();
                println!("Rect: {:?} {:?} {:?} {:?}", x, y, h, w);

                polygons.push(Polygon::new(
                    LineString::from(vec![(x, y), (x, y + h), (x + w, y + h), (x + w, y), (x, y)]),
                    vec![],
                ));
            }
            _ => {}
        }
    }

    println!("{:?}", &polygons);

    let _polygons = MultiPolygon::from(polygons);

    println!("{:?}", _polygons);

    println!("{:?}", _polygons.convex_hull());

    println!("Hello, world!");
}

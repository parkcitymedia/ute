use std::io::{stdin,stdout,Write};

use crate::{Tile};

/*
/// generate a polygon of n (`side_input`) sides
pub fn custom_polygon() {
    let mut side_input: i32;
    println!("number of sides:");
    stdin().read_line(&mut side_input).expect("unicode is pretty diverse so you have to have entered in something REALLY out-of-context here.");

    

}
*/

/// ### triangle()
/// ```rust
/// // example:
/// let my_triangle: ute::Tile = generate::triangle();
/// ```
/// outputs a ute-style json "triangle"
/// polygon.
pub fn gen_triangle() -> Tile {
    let mut tri_tile = Tile{
        tile_center: String::from(""),
        tile_id: Some(String::from("")),
        tile_edges:Vec::new()
    };
    tri_tile
}

pub fn gen_quadagon() -> Tile {
    unimplemented!()
}

pub fn gen_pentagon() -> Tile {
    unimplemented!();
}

pub fn gen_hexagon() -> Tile {
    unimplemented!();
}

pub fn gen_septagon() -> Tile {
    unimplemented!();
}

pub fn gen_octagon() -> Tile {
    unimplemented!();
}

pub fn gen_cutsom() -> Tile {
    unimplemented!();
}
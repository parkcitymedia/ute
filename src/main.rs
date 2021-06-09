extern crate serde_json;
extern crate tokio;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TagPosition {
    tag_position: String,
    tag_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EdgeValue {
    is_open: bool,
    edge_tag: Vec<TagPosition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Edge {
    tile_edge: EdgeValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Tile {
    tile_id: String,
    tile_center: String,
    tile_edges: Vec<Edge>,
}

/// takes in a tile ref (tile: &Tile) and  returns a (Tile) with a 
/// new "tile_id" value based on the tile field information.
fn identify_tile(tile: &Tile) -> Result<Tile, Box<dyn std::error::Error>> {
    println!("identifying tile...");

    // set up vector to hold characters
    let mut id_vec: Vec<char> = Vec::new();
    let center_value: Vec<char> = tile.tile_center.chars().collect();
    println!("id_value vec: {:#?}", &center_value[0]);

    for i in new_tile.tile_edges.clone() {
        if i.tile_edge.is_open.clone() == true {

            // if is_open is true, append a 1 to vec
            id_vec.push(1 as char);

        } else {

            // else, send a 0
            id_vec.push(0 as char);
        }

        // loop through the TagPosition values 
        for j in i.tile_edge.edge_tag.clone() {

            //collect 
            let edge_tag_map: Vec<char> = j.tag_value.clone().chars().collect();
            let edge_value_char: char = edge_tag_map[0].clone();
            id_vec.push(edge_value_char);

        }
    }

    new_tile.tile_id = id_vec.into_iter().collect();

    // result out the new tile (now with id, $2.99
    // call now for your free consultation!)
    Ok(new_tile)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // where's the shit?
    let raw_tile_path = "tile.json";

    // read that shit
    let tile_r: String = std::fs::read_to_string(raw_tile_path).unwrap();
    let tile_r: &str = &tile_r;

    // make shit usable
    let tile: Tile = serde_json::from_str(tile_r)?;
    let tile_id: Tile = identify_tile(&tile)?;

    println!("{:#?}", tile.tile_id);

    Ok(())
}

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

/// Tile is a struct of Strings "tile_id" and "tile_center", as well as a "tile_edges" Vec<Edge>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Tile {
    /// Retrieved with global function identify_tile()
    tile_id: String,
    tile_center: String,
    tile_edges: Vec<Edge>,
}

/// takes in a mutable Tile ref (tile: &Tile) and  returns a (String) with a 
/// new "tile_id" value based on the tile field information.\n
/// EXAMPLE:
/// identify_tile(tile)
fn identify_tile(tile: &mut Tile) -> Result<Tile, Box<dyn std::error::Error>> {
    
    println!("identifying tile...");

    //let tile_id_string: String;
    let mut id_vec: Vec<char> = Vec::new();
    let char_1: String = tile.tile_center.chars().into_iter().collect();
    let char_1: Vec<char> = char_1.chars().collect();
    let char_1 = char_1[0];
    // send the "center_value" character to the vec
    id_vec.push(char_1);
    println!("center_value vec: {:#?}", &id_vec[0]);

    // result out the new tile (now with id, $2.99
    // call now for your free consultation!)
    let tile_id_string: String = id_vec.into_iter().collect();
    println!("id string: {}", tile_id_string);
    tile.tile_id = tile_id_string;

    let tile: Tile = tile.clone();

    Ok(tile)
}

/// raw_tile_path
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // where's the shit?
    let raw_tile_path = "tile.json";

    // read that shit
    let tile_r: String = std::fs::read_to_string(raw_tile_path).unwrap();
    let tile_r: &str = &tile_r;

    // make shit usable
    let mut tile: Tile = serde_json::from_str(tile_r)?;
    
    /* GIVE TILE IDENTIFIER. */
    tile = identify_tile(&mut tile)?;

    println!("{:#?}", tile.tile_id);

    Ok(())
}

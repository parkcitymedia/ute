extern crate serde_json;
extern crate tokio;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct TagPosition {
    tag_position: String,
    tag_value: String
}

#[derive(Serialize, Deserialize, Debug)]
struct EdgeValue {
    is_open: bool,
    edge_tag: Vec<TagPosition>
}

#[derive(Serialize, Deserialize, Debug)]
struct Edge {
    tile_edge:EdgeValue,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tile {
    tile_id: String,
    tile_center: String,
    tile_edges: Vec<Edge>
}

// #[derive(Serialize, Deserialize, Debug)]
// struct TileMap {
//     map_id: String,
//     tiles: Vec<Tile>
// }


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // where's the shit?
    let raw_tile_path = "tile.json";
    let _raw_map_path = "tilemap.json";

    // read that shit
    let tile_r: String = std::fs::read_to_string(raw_tile_path).unwrap();
    let tile_r: &str = &tile_r;
    // let map_r: String = std::fs::read_to_string(raw_map_path).unwrap();
    // let map_r: &str = &map_r;
    println!("{}",tile_r);

    // make shit usable
    let tile: Tile = serde_json::from_str(tile_r)?;
    println!("{:#?}",tile.tile_edges[0 as usize].tile_edge.is_open);
    //let struct_tilemap: TileMap = serde_json::from_str(map_r)?;

    // pointlessly echo shit
    //println!("identifying tiles...");
    
    Ok(())
}

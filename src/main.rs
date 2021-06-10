extern crate serde_json;
extern crate tokio;


use serde::{Deserialize, Serialize};

/// holds a tag_position: String, and a tag_value: String
#[derive(Serialize, Deserialize, Debug, Clone)]
struct TagPosition {
    tag_position: String,
    tag_value: String,
}

/// holds a is_open: bool, and an edge_tag: Vec\<TagPosition>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct EdgeValue {
    /// checks whether or not a side is open for linking
    is_open: bool,
    /// a vector of TagPosition{}s
    edge_tag: Vec<TagPosition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Edge {
    tile_edge: EdgeValue,
}

/// struct holds `tile_id: String`, `tile_center: string`, `tile_edges: Vec<Edge>` 
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Tile {
    /// Retrieved with global function identify_tile()
    tile_id: String,
    /// center value of a tile
    tile_center: String,
    /// Vec\<Edge>
    tile_edges: Vec<Edge>,
}


///  takes in a mutable Tile ref (tile: &Tile) and  returns a (String) with a 
///  new "tile_id" value based on the tile field information. EXAMPLE:
///  identify_tile(tile) -> tile (it now has a tile_id)
fn identify_tile(tile: &mut Tile) -> Result<Tile, Box<dyn std::error::Error>> {
    
    println!("identifying tile...");

    // send the "center_value" character to the vec
    let mut id_vec: Vec<char> = Vec::new();
    let char_1: char = tile.tile_center.chars().into_iter().next().expect("string's empty...");
    id_vec.push(char_1);

    for edge in tile.tile_edges.iter() {
        
        let is_open_bit: char = if edge.tile_edge.is_open == false {'0'} else {'1'};
        id_vec.push(is_open_bit);

        for tag_position in edge.tile_edge.edge_tag.iter() {

            let tag_value_char: char = tag_position.tag_value.chars().next().expect("idk wtf happened here but rustc dosen't like it");
            id_vec.push(tag_value_char);

        }
    }

    // result out the new tile (now with id, $2.99
    // call now for your free consultation!)
    let tile_id_string: String = id_vec.into_iter().collect();
    tile.tile_id = tile_id_string;

    Ok(tile.clone())
}

/// # main()
/// if you don't know when this
/// runs, you shouldn't be working
/// on this rn lmao
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // where's the shit?
    let raw_tile_path: &str = "tile.json";

    // read that shit
    let tile_r: String = std::fs::read_to_string(raw_tile_path).unwrap();
    let tile_r: &str = &tile_r;

    // make shit usable
    let mut tile: Tile = serde_json::from_str(tile_r)?;
    
    // GIVE TILE IDENTIFIER.
    tile = identify_tile(&mut tile)?;

    println!("id: {:#?}", tile.tile_id);

    Ok(())
}

use serde::{Deserialize, Serialize};
/// holds a tag_position: String, and a tag_value: String
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagPosition {
    pub tag_position: String,
    pub tag_value: String,
}

/// holds a `is_open: bool`, and an `edge_tag: Vec<TagPosition>`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EdgeValue {
    /// checks whether or not a side is open for linking
    /// (useful in pathfinding or maze creation)
    pub is_open: bool,
    /// a vector of TagPosition{}s
    pub edge_tag: Vec<TagPosition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edge {
    /// holds an `is_open
    pub tile_edge: EdgeValue,
}

/// map this struct with serde_json::from_str()
/// | struct holds `tile_id: String`, `tile_center: string`, `tile_edges: Vec<Edge>`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tile {
    /// Retrieved with global function identify_tile()
    pub tile_id: String,
    /// center value of a tile
    pub tile_center: String,
    /// Vec\<Edge>
    pub tile_edges: Vec<Edge>,
}

///  takes in a mutable Tile ref (tile: &Tile) and  returns a (String) with a
///  new "tile_id" value based on the tile field information. EXAMPLE:
///  identify_tile(tile) -> tile (it now has a tile_id)
pub fn identify_tile(tile: &mut Tile) -> Result<Tile, Box<dyn std::error::Error>> {

    // this vec stores the string that we mutate `tile.tile_id` with
    let mut id_vec: Vec<char> = Vec::new();

    // send the "center_value" character to the vec
    let char_1: char = tile
        .tile_center
        .chars()
        .into_iter()
        .next()
        .expect("string's empty...");
    id_vec.push(char_1);

    for edge in tile.tile_edges.iter() {
        // push the bit for the `is_open` field of every `tile_edge`
        let is_open_bit: char = if edge.tile_edge.is_open == false {
            // if edge is not open, push a 0
            '0'
        } else {
            // otherwise, push a 1
            '1'
        };
        id_vec.push(is_open_bit);

        // loop through every character in a `tile_edge.tag` and push
        // to `id_vec`
        for tag_position in edge.tile_edge.edge_tag.iter() {
            for tag_value_char in tag_position.tag_value.chars() {
                id_vec.push(tag_value_char);
            }
        }
    }

    // result out the new tile (now with id, $2.99
    // call now for your free consultation!)
    let tile_id_string: String = id_vec.into_iter().collect();
    tile.tile_id = tile_id_string;

    Ok(tile.clone())
}

/// tile generation tools.
#[cfg(feature="generate")]
pub mod generate;
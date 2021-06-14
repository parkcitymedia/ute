# UTE <Sub><i>( yoot )</i></sub>

ute - polygons of arbitrary number of tagged sides engine for matching and manipulation (in rust)

## navigation
 <sub>[[you're here]](#navigation)</sub>

[using ute](#using-ute)
- [including in your project](#including-in-your-project)

[examples](#examples)
- [using handmade tiles](#using-handmade-tiles)
- [single tile input](#single-tile-input)

[features](#features)

## using pantsemm
#### including in your project
 <sub>[[top]](#navigation)</sub>

- add this repo* to your `Cargo.toml`:
```toml
[dependencies]

# ... other deps are probably here ...

ute = {git = "https://github.com/parkcitymedia/ute", branch="main"}
```
<sub>*cargo searches by default for a "master" branch, so branch specification may/may not be necessary. </sub>


## examples
#### using handmade or generated tiles
 <sub>[[top]](#navigation)</sub>

assuming tile path "`tile.json`" [has been made](#single-tile-input)/exists:
- ##### map json to a tile (serde)
 <sub>[[top]](#navigation)</sub>
```rust
// likely your main.rs:
use ute::{Tile, identify_tile};
use serde_json::{from_str};
use std::fs::read_to_string;

#[tokio::main] // requires tokio = {features = ["full"]}
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // where does the tile live?
    let raw_tile_path: &str = "tile.json";
    
    // read the tile info into a String (re-borrow
    // into &str for serde_json compatibility)
    let tile_r: String = read_to_string(raw_tile_path).unwrap();
    let tile_r: &str = &tile_r;

    // use serde_json::from_str() to
    // map the json string to a pantsemm::Tile
    // note: this tile is mutable - while they don't
    // need to be, this is good for when
    // you want to change tile field values later
    let mut tile: Tile = from_str(tile_r)?;
    
    // generate a unique tile-data-based tile name
    // with pantsemm's pantsemm::identify_tile()
    // (returns a Tile!!! common use: mutation)
    tile = identify_tile(&mut tile)?;

    // print the tile_id out after generating it!
    println!("my new tile id: {:#?}", tile.tile_id);

    Ok(())
}
```

#### single tile input
 <sub>[[top]](#navigation)</sub>

example can be found in `tiles/tile_example.json`[[view]](tiles/tile_example.json) for data input.
`tile_example.json`
```json
{
    "tile_id": "",
    "tile_center": "5",
    "tile_edges": [
        {
            "tile_edge": {
                "is_open": true,
                "name": "a",
                "edge_tag": [
                    {
                        "tag_position": "0",
                        "tag_value": "r"
                    },
                    {
                        "tag_position": "1",
                        "tag_value": "q"
                    },
                    {
                        "tag_position": "2",
                        "tag_value": "p"
                    }
                ]
            }
        },
        {
            "tile_edge": {
                "is_open": true,
                "name": "a",
                "edge_tag": [
                    {
                        "tag_position": "0",
                        "tag_value": "o"
                    },
                    {
                        "tag_position": "1",
                        "tag_value": "n"
                    },
                    {
                        "tag_position": "2",
                        "tag_value": "m"
                    }
                ]
            }
        },
        {
            "tile_edge": {
                "is_open": true,
                "name": "a",
                "edge_tag": [
                    {
                        "tag_position": "0",
                        "tag_value": "l"
                    },
                    {
                        "tag_position": "1",
                        "tag_value": "k"
                    },
                    {
                        "tag_position": "2",
                        "tag_value": "j"
                    }
                ]
            }
        },
        {
            "tile_edge": {
                "is_open": true,
                "name": "a",
                "edge_tag": [
                    {
                        "tag_position": "0",
                        "tag_value": "i"
                    },
                    {
                        "tag_position": "1",
                        "tag_value": "h"
                    },
                    {
                        "tag_position": "2",
                        "tag_value": "g"
                    }
                ]
            }
        },
        {
            "tile_edge": {
                "is_open": true,
                "name": "a",
                "edge_tag": [
                    {
                        "tag_position": "0",
                        "tag_value": "f"
                    },
                    {
                        "tag_position": "1",
                        "tag_value": "e"
                    },
                    {
                        "tag_position": "2",
                        "tag_value": "d"
                    }
                ]
            }
        },
        {
            "tile_edge": {
                "is_open": true,
                "name": "a",
                "edge_tag": [
                    {
                        "tag_position": "0",
                        "tag_value": "c"
                    },
                    {
                        "tag_position": "1",
                        "tag_value": "b"
                    },
                    {
                        "tag_position": "2",
                        "tag_value": "a"
                    }
                ]
            }
        }
    ]
}
```
## references
- [Rust By Example: Struct Visibility](https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html)
- [Rust Forum: how to get user input](https://users.rust-lang.org/t/how-to-get-user-input/5176)
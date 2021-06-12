# PANTSEMM <Sub><i>( pænts · em )</i></sub>

pantsemm - polygons of arbitrary number of tagged sides engine for matching and manipulation (in rust)

## navigation
 <sub>[[you're here]](#navigation)</sub>

[using pantsemm](#using-pantsemm)
- [including in your project](#including-in-your-project)

[examples](#examples)
- [single tile input](#single-tile-input)

[features](#features)

## using pantsemm
 <sub>[[top]](#navigation)</sub>


#### including in your project
 <sub>[[top]](#navigation)</sub>

- add this repo* to your `Cargo.toml`:
```toml
[dependencies]

# ... other deps are probably here ...

pantsemm = {git = "https://github.com/parkcitymedia/pantsemm", branch="main"}
```
<sub>*cargo searches by default for a "master" branch, so branch specification may/may not be necessary. </sub>


## examples
 <sub>[[top]](#navigation)</sub>

#### single tile input
 <sub>[[top]](#navigation)</sub>

- example can be found in `tiles/tile_example.json`[[view]] for data input.
- `tile_example.json`
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
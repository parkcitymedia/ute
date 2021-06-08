# PANTSEMM
<h6><Sub><i>( pænts · em )</i></sub></h6>
pantsemm_rs - polygons of arbitrary number of tagged sides engine for matching and manipulation (in rust)

## navigation
[installation](#installation)
- [dependencies](#dependencies)
- [building from source](#building-from-source)

[examples](#examples)
- [single tile input](#single-tile-input)

## installation

#### dependencies
- rust
- cargo (comes with rust)

#### building from source
- clone repo
- change dir into repo
- execute a `cargo run` in the directory
  - default output binary gets tossed into target/debug/\<binary>

## examples

#### single tile input
- example can be found in `tile.json` for data input.
- `tile.json`
    ```json
    {
        "tile_id":"",
        "tile_center": "5",
        "tile_edges": {
            "edge_a": {
                "is_open": true,
                "tag": {
                    "tag_1": "a",
                    "tag_2": "b",
                    "tag_3": "c"
                }
            },
            "edge_b": {
                "is_open": false,
                "tag": {
                    "tag_1": "a",
                    "tag_2": "d",
                    "tag_3": "b"
                }
            },
            "edge_c": {
                "is_open": false,
                "tag": {
                    "tag_1": "c",
                    "tag_2": "b",
                    "tag_3": "b"
                }
            },
            "edge_d": {
                "is_open": true,
                "tag": {
                    "tag_1": "c",
                    "tag_2": "d",
                    "tag_3": "c"
                }
            },
            "edge_e": {
                "is_open": false,
                "tag": {
                    "tag_1": "b",
                    "tag_2": "f",
                    "tag_3": "f"
                }
            },
            "edge_f": {
                "is_open": false,
                "tag": {
                    "tag_1": "a",
                    "tag_2": "f",
                    "tag_3": "c"
                }
            }
        }
    }
    ```
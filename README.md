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
- example can be found in `tile_example.json` for data input.
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
                            "tag_value": ""
                        },
                        {
                            "tag_position": "1",
                            "tag_value": ""
                        },
                        {
                            "tag_position": "2",
                            "tag_value": ""
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
                            "tag_value": ""
                        },
                        {
                            "tag_position": "1",
                            "tag_value": ""
                        },
                        {
                            "tag_position": "2",
                            "tag_value": ""
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
                            "tag_value": ""
                        },
                        {
                            "tag_position": "1",
                            "tag_value": ""
                        },
                        {
                            "tag_position": "2",
                            "tag_value": ""
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
                            "tag_value": ""
                        },
                        {
                            "tag_position": "1",
                            "tag_value": ""
                        },
                        {
                            "tag_position": "2",
                            "tag_value": ""
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
                            "tag_value": ""
                        },
                        {
                            "tag_position": "1",
                            "tag_value": ""
                        },
                        {
                            "tag_position": "2",
                            "tag_value": ""
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
                            "tag_value": ""
                        },
                        {
                            "tag_position": "1",
                            "tag_value": ""
                        },
                        {
                            "tag_position": "2",
                            "tag_value": ""
                        }
                    ]
                }
            }
        ]
    }
    ```
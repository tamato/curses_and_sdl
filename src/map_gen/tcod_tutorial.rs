
/**********************************************************
    Generates a map in the style from the TCOD tutorial
***********************************************************/
use map_gen::{MapTileType, MapData};

pub fn map_generation(width:u32, height:u32) -> MapData {
    MapData {
        width: width,
        height: height,
        data: vec![MapTileType::Wall],
    }
}

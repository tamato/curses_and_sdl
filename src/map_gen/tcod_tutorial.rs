
/**********************************************************
    Generates a map in the style from the TCOD tutorial
***********************************************************/
use map_gen::{MapTileType, MapData};

pub fn map_generation(width:usize, height:usize) -> MapData {

    let mut map_data = vec![MapTileType::Floor; (width * height)];
    for idx in 0..(width * height) {
        let col_count = width;
        let row = idx / col_count;
        let col = idx % col_count;

        let mut tile = MapTileType::Floor;
        if col == 0 || col == (width - 1) {
            tile = MapTileType::Wall;
        }
        if row == 0 || row == (height - 1) {
            tile = MapTileType::Wall;
        }
        map_data[idx] = tile;
    }

    MapData {
        width: width as u32,
        height: height as u32,
        data: map_data,
    }
}

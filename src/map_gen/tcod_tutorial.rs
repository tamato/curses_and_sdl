
/**********************************************************
    Generates a map in the style from the TCOD tutorial
***********************************************************/
use map_gen::{MapTileType, MapData};

extern crate sdl2;
use self::sdl2::rect::Rect;

pub fn map_generation(width:usize, height:usize) -> MapData {
    let mut map = MapData {
        width: width as u32,
        height: height as u32,
        data: vec![MapTileType::Empty; (width * height)],
    };

    create_room(Rect::new(0,0,5,5), &mut map);
    create_room(Rect::new(5,5,5,8), &mut map);

    map
}

fn create_room(room:Rect, map: &mut MapData) {
    let width = room.x+room.w;
    let height = room.y+room.h;

    let map_width = map.width as i32;

    // fill out the walls of the room
    for x in room.x..width { 
        let index = room.y * map_width + x;
        map[index] = MapTileType::Wall;

        let index = (height-1) * map_width + x;
        map[index] = MapTileType::Wall;
    }
    for y in room.y..height { 
        let index = y * map_width + room.x;
        map[index] = MapTileType::Wall;

        let index = y * map_width + (width-1);
        map[index] = MapTileType::Wall;
    }

    // fill in the room
    for x in (room.x+1)..(width-1) {
        for y in (room.y+1)..(height-1) {
            let index = x + y * map_width;
            map[index] = MapTileType::Floor;
        }
    }
}
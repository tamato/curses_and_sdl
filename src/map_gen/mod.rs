
#[derive(Clone, Copy)]
pub enum MapTileType {
    Empty,
    Floor,
    Wall,
    Chasm,
    Water,
}

pub struct MapData {
    width: u32,
    height: u32,
    data: Vec<MapTileType>,
}

impl MapData {
    pub fn render<T>(&self, mut renderer: T)
        where T: FnMut(&u32, &u32, &Vec<MapTileType>)
    {
        renderer(&self.width, &self.height, &self.data);
    }
}

use std::ops::Index;
impl Index<i32> for MapData {
    type Output = MapTileType;
    fn index(&self, index: i32) -> &Self::Output {
        &self.data[index as usize]
    }
}

use std::ops::IndexMut;
impl IndexMut<i32> for MapData {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

pub mod tcod_tutorial;
// pub use self::tcod_tutorial::map_generation;


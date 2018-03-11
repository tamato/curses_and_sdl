
#[derive(Clone, Copy)]
pub enum MapTileType {
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

pub mod tcod_tutorial;
// pub use self::tcod_tutorial::map_generation;


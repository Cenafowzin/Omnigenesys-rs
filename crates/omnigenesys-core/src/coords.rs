#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct GridSize {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl Coord {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    
    pub fn is_2d(&self) -> bool {
        self.z == 0
    }
}

impl GridSize {

    pub fn is_2d(&self) -> bool {
        self.depth == 1
    }

    pub fn volume(&self) -> usize {
        (self.width * self.height * self.depth) as usize
    }
}
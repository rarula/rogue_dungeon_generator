pub type Vec2<T> = Vec<Vec<T>>;

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Size {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Rectangle {
    pub pos: Position,
    pub size: Size,
}

#[derive(Debug)]
pub struct Path {
    pub rect: Rectangle,
    pub is_horizontal: bool,
}

#[derive(Debug)]
pub struct DividedArea {
    pub rect: Rectangle,
    pub paths: Vec<Path>,
}

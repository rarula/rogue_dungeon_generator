use std::cell::RefCell;
use std::rc::Rc;

pub type Vec2<T> = Vec<Vec<T>>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Size {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Rectangle {
    pub pos: Position,
    pub size: Size,
}

impl Rectangle {
    pub fn divide(&self, divider: &Path) -> (Rectangle, Rectangle) {
        if divider.is_horizontal {
            let a = Rectangle {
                pos: Position {
                    x: self.pos.x,
                    y: self.pos.y,
                },
                size: Size {
                    x: self.size.x,
                    y: divider.rect.pos.y - self.pos.y,
                },
            };
            let b = Rectangle {
                pos: Position {
                    x: self.pos.x,
                    y: divider.rect.pos.y + divider.rect.size.y,
                },
                size: Size {
                    x: self.size.x,
                    y: self.size.y - (divider.rect.pos.y + divider.rect.size.y - self.pos.y),
                },
            };

            (a, b)
        } else {
            let a = Rectangle {
                pos: Position {
                    x: self.pos.x,
                    y: self.pos.y,
                },
                size: Size {
                    x: divider.rect.pos.x - self.pos.x,
                    y: self.size.y,
                },
            };
            let b = Rectangle {
                pos: Position {
                    x: divider.rect.pos.x + divider.rect.size.x,
                    y: self.pos.y,
                },
                size: Size {
                    x: self.size.x - (divider.rect.pos.x + divider.rect.size.x - self.pos.x),
                    y: self.size.y,
                },
            };

            (a, b)
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.intersects_x(other) && self.intersects_y(other)
    }

    pub fn intersects_x(&self, other: &Self) -> bool {
        self.pos.x < other.pos.x + other.size.x && other.pos.x < self.pos.x + self.size.x
    }

    pub fn intersects_y(&self, other: &Self) -> bool {
        self.pos.y < other.pos.y + other.size.y && other.pos.y < self.pos.y + self.size.y
    }
}

#[derive(Debug)]
pub struct Path {
    pub rect: Rectangle,
    pub is_horizontal: bool,
}

#[derive(Debug)]
pub struct DividedArea {
    pub rect: Rectangle,
    pub path: Path,
    pub sub_paths: Vec<Path>,
    pub positioned_nodes: Vec<PositionedNode>,
    pub horizontal_edges: Vec<Rc<RefCell<Edge>>>,
    pub vertical_edges: Vec<Rc<RefCell<Edge>>>,
}

impl DividedArea {
    pub fn region(&self, quadrant: Quadrant) -> Region {
        let (top_or_left, bottom_or_right) = self.rect.divide(&self.path);

        match quadrant {
            Quadrant::LeftTop => {
                for sub_path in &self.sub_paths {
                    if top_or_left.intersects(&sub_path.rect) {
                        return Region::LeftTop(top_or_left.divide(sub_path).0);
                    }
                }
                if self.path.is_horizontal {
                    Region::Top(top_or_left)
                } else {
                    Region::Left(top_or_left)
                }
            }
            Quadrant::LeftBottom => {
                if self.path.is_horizontal {
                    for sub_path in &self.sub_paths {
                        if bottom_or_right.intersects(&sub_path.rect) {
                            return Region::LeftBottom(bottom_or_right.divide(sub_path).0);
                        }
                    }
                    Region::Bottom(bottom_or_right)
                } else {
                    for sub_path in &self.sub_paths {
                        if top_or_left.intersects(&sub_path.rect) {
                            return Region::LeftBottom(top_or_left.divide(sub_path).1);
                        }
                    }
                    Region::Left(top_or_left)
                }
            }
            Quadrant::RightTop => {
                if self.path.is_horizontal {
                    for sub_path in &self.sub_paths {
                        if top_or_left.intersects(&sub_path.rect) {
                            return Region::RightTop(top_or_left.divide(sub_path).1);
                        }
                    }
                    Region::Top(top_or_left)
                } else {
                    for sub_path in &self.sub_paths {
                        if bottom_or_right.intersects(&sub_path.rect) {
                            return Region::RightTop(bottom_or_right.divide(sub_path).0);
                        }
                    }
                    Region::Right(bottom_or_right)
                }
            }
            Quadrant::RightBottom => {
                for sub_path in &self.sub_paths {
                    if bottom_or_right.intersects(&sub_path.rect) {
                        return Region::RightBottom(bottom_or_right.divide(sub_path).1);
                    }
                }
                if self.path.is_horizontal {
                    Region::Bottom(bottom_or_right)
                } else {
                    Region::Right(bottom_or_right)
                }
            }
        }
    }
}

pub enum Quadrant {
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
}

pub enum Region {
    Left(Rectangle),
    LeftTop(Rectangle),
    LeftBottom(Rectangle),
    Right(Rectangle),
    RightTop(Rectangle),
    RightBottom(Rectangle),
    Top(Rectangle),
    Bottom(Rectangle),
}

#[derive(Debug)]
pub struct CombinedRegion {
    pub rect: Rectangle,
    pub side_edges_x: Vec<Rc<RefCell<Edge>>>,
    pub side_edges_y: Vec<Rc<RefCell<Edge>>>,
    pub room: Option<Room>,
}

#[derive(Debug)]
pub struct Room {
    pub rect: Rectangle,
    pub is_horizontal: bool,
}

#[derive(Debug)]
pub struct Node {
    pub rect: Rectangle,
}

#[derive(Debug)]
pub struct Edge {
    pub a: Rc<Node>,
    pub b: Rc<Node>,
    pub is_protected: bool,
    pub is_enabled: bool,
}

impl Edge {
    pub fn to_rect(&self) -> Rectangle {
        let mut a = &self.a.rect;
        let mut b = &self.b.rect;

        if a.pos.x < b.pos.x || b.pos.x < a.pos.x {
            if b.pos.x < a.pos.x {
                (a, b) = (&self.b.rect, &self.a.rect);
            }
            Rectangle {
                pos: Position {
                    x: a.pos.x + a.size.x,
                    y: a.pos.y,
                },
                size: Size {
                    x: b.pos.x - (a.pos.x + a.size.x),
                    y: a.size.y,
                },
            }
        } else if a.pos.y < b.pos.y || b.pos.y < a.pos.y {
            if b.pos.y < a.pos.y {
                (a, b) = (&self.b.rect, &self.a.rect);
            }
            Rectangle {
                pos: Position {
                    x: a.pos.x,
                    y: a.pos.y + a.size.y,
                },
                size: Size {
                    x: a.size.x,
                    y: b.pos.y - (a.pos.y + a.size.y),
                },
            }
        } else {
            Rectangle {
                pos: Position {
                    x: 0,
                    y: 0,
                },
                size: Size {
                    x: 0,
                    y: 0,
                },
            }
        }
    }
}

#[derive(Debug)]
pub struct PositionedNode {
    pub node: Rc<Node>,
    pub loc: Location,
}

#[derive(PartialEq, Debug)]
pub enum Location {
    Left,
    Right,
    Top,
    Bottom,
    LeftRight,
    TopBottom,
    Border,
}

impl Location {
    pub fn is_border(&self) -> bool {
        if let Location::Border = self { true } else { false }
    }
}

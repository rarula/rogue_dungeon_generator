use crate::error::GenerationError;
use crate::generator::Args;
use crate::utils::*;
use rand::Rng;
use rand::seq::IndexedRandom;

pub fn create_field(args: &Args) -> Vec2<Rectangle> {
    let mut vec: Vec2<Rectangle> = Vec::new();

    for y in 0..args.area_count_y {
        let mut vec2: Vec<Rectangle> = Vec::new();

        for x in 0..args.area_count_x {
            let pos = Position {
                x: args.area_size * x,
                y: args.area_size * y,
            };
            let size = Size {
                x: args.area_size,
                y: args.area_size,
            };
            let rect = Rectangle { pos, size };

            vec2.push(rect);
        }

        vec.push(vec2);
    }

    vec
}

pub fn create_paths(
    args: &mut Args,
    field: Vec2<Rectangle>,
) -> Result<Vec2<DividedArea>, GenerationError> {
    let mut vec: Vec2<DividedArea> = Vec::new();
    let mut is_horizontal_source = args.rng.random_bool(0.5);

    for row in field {
        let mut vec2: Vec<DividedArea> = Vec::new();
        let mut is_horizontal = is_horizontal_source;

        let mut create_path =
            |rect: Rectangle, is_horizontal: bool| -> Result<DividedArea, GenerationError> {
                const PER: f64 = 0.3;

                if is_horizontal {
                    let range = rect.size.y - args.path_size - 1;

                    if 0 < range {
                        let diff = (range as f64 * PER).floor() as i32;
                        let path_offset = args.rng.random_range(0 + diff..range - diff);
                        let path_pos = path_offset + rect.pos.y + 1;

                        let path_rect = Rectangle {
                            pos: Position {
                                x: rect.pos.x,
                                y: path_pos,
                            },
                            size: Size {
                                x: rect.size.x,
                                y: args.path_size,
                            },
                        };
                        let path = Path {
                            rect: path_rect,
                            is_horizontal,
                        };
                        let area = DividedArea {
                            rect,
                            path,
                            sub_paths: Vec::new(),
                        };

                        Ok(area)
                    } else {
                        Err(GenerationError::GE0001)
                    }
                } else {
                    let range = rect.size.x - args.path_size - 1;

                    if 0 < range {
                        let diff = (range as f64 * PER).floor() as i32;
                        let path_offset = args.rng.random_range(0 + diff..range - diff);
                        let path_pos = path_offset + rect.pos.x + 1;

                        let path_rect = Rectangle {
                            pos: Position {
                                x: path_pos,
                                y: rect.pos.y,
                            },
                            size: Size {
                                x: args.path_size,
                                y: rect.size.y,
                            },
                        };
                        let path = Path {
                            rect: path_rect,
                            is_horizontal,
                        };
                        let area = DividedArea {
                            rect,
                            path,
                            sub_paths: Vec::new(),
                        };

                        Ok(area)
                    } else {
                        Err(GenerationError::GE0001)
                    }
                }
            };

        for rect in row {
            let result = create_path(rect, is_horizontal);
            match result {
                Ok(area) => vec2.push(area),
                Err(e) => return Err(e),
            };

            is_horizontal = !is_horizontal;
        }

        vec.push(vec2);
        is_horizontal_source = !is_horizontal_source;
    }

    Ok(vec)
}

pub fn extend_paths(args: &Args, mut field: Vec2<DividedArea>) -> Vec2<DividedArea> {
    for y in 0..field.len() {
        for x in 0..field[y].len() {
            let is_horizontal = field[y][x].path.is_horizontal;

            if is_horizontal {
                let pos_y = field[y][x].path.rect.pos.y;

                // Extend the path to the left
                if 0 < x {
                    let target_area = &mut field[y][x - 1];
                    let target_path = &target_area.path;

                    let pos = target_path.rect.pos.x + target_path.rect.size.x;
                    let size = target_area.rect.pos.x + target_area.rect.size.x - pos;

                    let rect = Rectangle {
                        pos: Position { x: pos, y: pos_y },
                        size: Size {
                            x: size,
                            y: args.path_size,
                        },
                    };
                    let path = Path {
                        rect,
                        is_horizontal,
                    };

                    target_area.sub_paths.push(path);
                }
                // Extend the path to the right
                if x < field[y].len() - 1 {
                    let target_area = &mut field[y][x + 1];
                    let target_path = &target_area.path;

                    let pos = target_area.rect.pos.x;
                    let size = target_path.rect.pos.x - pos;

                    let rect = Rectangle {
                        pos: Position { x: pos, y: pos_y },
                        size: Size {
                            x: size,
                            y: args.path_size,
                        },
                    };
                    let path = Path {
                        rect,
                        is_horizontal,
                    };

                    target_area.sub_paths.push(path);
                }
            } else {
                let pos_x = field[y][x].path.rect.pos.x;

                // Extend the path to the top
                if 0 < y {
                    let target_area = &mut field[y - 1][x];
                    let target_path = &target_area.path;

                    let pos = target_path.rect.pos.y + target_path.rect.size.y;
                    let size = target_area.rect.pos.y + target_area.rect.size.y - pos;

                    let rect = Rectangle {
                        pos: Position { x: pos_x, y: pos },
                        size: Size {
                            x: args.path_size,
                            y: size,
                        },
                    };
                    let path = Path {
                        rect,
                        is_horizontal,
                    };

                    target_area.sub_paths.push(path);
                }
                // Extend the path to the bottom
                if y < field.len() - 1 {
                    let target_area = &mut field[y + 1][x];
                    let target_path = &target_area.path;

                    let pos = target_area.rect.pos.y;
                    let size = target_path.rect.pos.y - pos;

                    let rect = Rectangle {
                        pos: Position { x: pos_x, y: pos },
                        size: Size {
                            x: args.path_size,
                            y: size,
                        },
                    };
                    let path = Path {
                        rect,
                        is_horizontal,
                    };

                    target_area.sub_paths.push(path);
                }
            }
        }
    }

    field
}

pub fn combine_regions(field: &Vec2<DividedArea>) -> Vec<Rectangle> {
    let mut vec: Vec<Rectangle> = Vec::new();

    for y in 0..field.len() {
        for x in 0..field[y].len() {
            if y == 0 {
                match field[y][x].region(Quadrant::RightTop) {
                    Region::Top(sr) => {
                        if x == 0 {
                            if let Region::LeftTop(r) = field[y][x + 1].region(Quadrant::LeftTop) {
                                vec.push(combine_x(sr, r));
                            }
                        }
                    }
                    Region::RightTop(sr) => {
                        if let Region::Top(r) = field[y][x + 1].region(Quadrant::LeftTop) {
                            if x < field[y].len() - 2 {
                                let sr = combine_x(sr, r);
                                if let Region::LeftTop(r) =
                                    field[y][x + 2].region(Quadrant::LeftTop)
                                {
                                    vec.push(combine_x(sr, r));
                                }
                            } else {
                                vec.push(combine_x(sr, r));
                            }
                        }
                    }
                    _ => (),
                }
            }
            if y == field.len() - 1 {
                match field[y][x].region(Quadrant::RightBottom) {
                    Region::Bottom(sr) => {
                        if x == 0 {
                            if let Region::LeftBottom(r) =
                                field[y][x + 1].region(Quadrant::LeftBottom)
                            {
                                vec.push(combine_x(sr, r));
                            }
                        }
                    }
                    Region::RightBottom(sr) => {
                        if let Region::Bottom(r) = field[y][x + 1].region(Quadrant::LeftBottom) {
                            if x < field[y].len() - 2 {
                                let sr = combine_x(sr, r);
                                if let Region::LeftBottom(r) =
                                    field[y][x + 2].region(Quadrant::LeftBottom)
                                {
                                    vec.push(combine_x(sr, r));
                                }
                            } else {
                                vec.push(combine_x(sr, r));
                            }
                        }
                    }
                    _ => (),
                }
            }
            if x == 0 {
                match field[y][x].region(Quadrant::LeftBottom) {
                    Region::Left(sr) => {
                        if y == 0 {
                            if let Region::LeftTop(r) = field[y + 1][x].region(Quadrant::LeftTop) {
                                vec.push(combine_y(sr, r));
                            }
                        }
                    }
                    Region::LeftBottom(sr) => {
                        if let Region::Left(r) = field[y + 1][x].region(Quadrant::LeftTop) {
                            if y < field.len() - 2 {
                                let sr = combine_y(sr, r);
                                if let Region::LeftTop(r) =
                                    field[y + 2][x].region(Quadrant::LeftTop)
                                {
                                    vec.push(combine_y(sr, r));
                                }
                            } else {
                                vec.push(combine_y(sr, r));
                            }
                        }
                    }
                    _ => (),
                }
            }
            if x == field[y].len() - 1 {
                match field[y][x].region(Quadrant::RightBottom) {
                    Region::Right(sr) => {
                        if y == 0 {
                            if let Region::RightTop(r) = field[y + 1][x].region(Quadrant::RightTop)
                            {
                                vec.push(combine_y(sr, r));
                            }
                        }
                    }
                    Region::RightBottom(sr) => {
                        if let Region::Right(r) = field[y + 1][x].region(Quadrant::RightTop) {
                            if y < field.len() - 2 {
                                let sr = combine_y(sr, r);
                                if let Region::RightTop(r) =
                                    field[y + 2][x].region(Quadrant::RightTop)
                                {
                                    vec.push(combine_y(sr, r));
                                }
                            } else {
                                vec.push(combine_y(sr, r));
                            }
                        }
                    }
                    _ => (),
                }
            }
            if y < field.len() - 1 && x < field[y].len() - 1 {
                if let Region::RightBottom(sr) = field[y][x].region(Quadrant::RightBottom) {
                    if let Region::LeftBottom(r0) = field[y][x + 1].region(Quadrant::LeftBottom) {
                        if let Region::RightTop(r1) = field[y + 1][x].region(Quadrant::RightTop) {
                            let x_rect = combine_x(sr, r0);
                            let xy_rect = combine_y(x_rect, r1);
                            vec.push(xy_rect);
                        }
                    }
                }
            }

            fn combine_x(source_rect: Rectangle, rect: Rectangle) -> Rectangle {
                Rectangle {
                    pos: source_rect.pos,
                    size: Size {
                        x: source_rect.size.x + rect.size.x,
                        y: source_rect.size.y,
                    },
                }
            }
            fn combine_y(source_rect: Rectangle, rect: Rectangle) -> Rectangle {
                Rectangle {
                    pos: source_rect.pos,
                    size: Size {
                        x: source_rect.size.x,
                        y: source_rect.size.y + rect.size.y,
                    },
                }
            }
        }
    }

    vec
}

pub fn create_subareas(
    args: &mut Args,
    regions: Vec<Rectangle>,
) -> Result<Vec<Subarea>, GenerationError> {
    let mut candidates: Vec<Rectangle> = Vec::new();

    for rect in regions {
        if 0 < rect.size.x - 2 && 0 < rect.size.y - 2 {
            candidates.push(rect);
        }
    }
    if args.room_count <= candidates.len() as i32 {
        let mut subareas: Vec<Subarea> = Vec::new();

        for candidate in candidates.choose_multiple(&mut args.rng, args.room_count as usize) {
            let size = Size {
                x: args
                    .rng
                    .random_range(candidate.size.x / 2..candidate.size.x - 1),
                y: args
                    .rng
                    .random_range(candidate.size.y / 2..candidate.size.y - 1),
            };
            let pos = Position {
                x: candidate.pos.x + 1 + args.rng.random_range(0..candidate.size.x - 1 - size.x),
                y: candidate.pos.y + 1 + args.rng.random_range(0..candidate.size.y - 1 - size.y),
            };
            let room = Room {
                rect: Rectangle { pos, size },
                is_horizontal: args.rng.random_bool(0.5),
            };
            let subarea = Subarea {
                rect: candidate.clone(),
                room,
            };

            subareas.push(subarea);
        }

        Ok(subareas)
    } else {
        if args.area_count_x * args.area_count_y + 1 < args.room_count {
            Err(GenerationError::GE0002(args.room_count, candidates.len()))
        } else {
            Err(GenerationError::GE0003(args.room_count, candidates.len()))
        }
    }
}

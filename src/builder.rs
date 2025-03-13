use crate::error::GenerationError;
use crate::generator::Args;
use crate::utils::*;
use rand::Rng;

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
                if is_horizontal {
                    let range = rect.size.y - args.path_size - 1;

                    if 0 < range {
                        let path_offset = args.rng.random_range(0..range);
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
                        let path_offset = args.rng.random_range(0..range);
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

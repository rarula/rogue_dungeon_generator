use crate::generator::Args;
use crate::utils::*;

pub fn visualizer_3(args: &Args, regions: &Vec<CombinedRegion>) -> String {
    let mut str = String::new();

    for y in 0..args.area_size * args.area_count_y as i32 {
        'outer: for x in 0..args.area_size * args.area_count_x as i32 {
            let point = Rectangle {
                pos: Position {
                    x,
                    y,
                },
                size: Size {
                    x: 1,
                    y: 1,
                },
            };
            for region in regions {
                if region.rect.intersects(&point) {
                    str.push('.');
                    continue 'outer;
                }
            }

            str.push('#');
        }

        str.push('\n');
    }

    str
}

pub fn visualizer_4(args: &Args, regions: &Vec<CombinedRegion>) -> String {
    let mut str = String::new();

    for y in 0..args.area_size * args.area_count_y as i32 {
        'outer: for x in 0..args.area_size * args.area_count_x as i32 {
            let point = Rectangle {
                pos: Position {
                    x,
                    y,
                },
                size: Size {
                    x: 1,
                    y: 1,
                },
            };
            for region in regions {
                if let Some(room) = &region.room {
                    if room.rect.intersects(&point) {
                        str.push('.');
                        continue 'outer;
                    }
                }
                if region.rect.intersects(&point) {
                    str.push(' ');
                    continue 'outer;
                }
            }

            str.push('#');
        }

        str.push('\n');
    }

    str
}

pub fn visualizer_5(args: &Args, field: &Vec2<DividedArea>, regions: &Vec<CombinedRegion>) -> String {
    let area_size = args.area_size as usize;
    let size_x = area_size * args.area_count_x;
    let size_y = area_size * args.area_count_y;

    let mut chars_vec = vec![vec![' '; size_x]; size_y];
    let mut fill = |rect: &Rectangle, val: char| {
        if 0 <= rect.pos.x && 0 <= rect.pos.y && rect.pos.x < size_x as i32 && rect.pos.y < size_y as i32 {
            for y in rect.pos.y..rect.pos.y + rect.size.y {
                for x in rect.pos.x..rect.pos.x + rect.size.x {
                    chars_vec[y as usize][x as usize] = val;
                }
            }
        }
    };
    for areas in field {
        for area in areas {
            for edge in &area.horizontal_edges {
                if edge.borrow().is_enabled {
                    fill(&edge.borrow().a.rect, '@');
                    fill(&edge.borrow().b.rect, '@');
                    fill(&edge.borrow().to_rect(), '#');
                }
            }
            for edge in &area.vertical_edges {
                if edge.borrow().is_enabled {
                    fill(&edge.borrow().a.rect, '@');
                    fill(&edge.borrow().b.rect, '@');
                    fill(&edge.borrow().to_rect(), '#');
                }
            }
        }
    }
    for region in regions {
        if let Some(room) = &region.room {
            if room.is_horizontal {
                fill(&room.rect, '-');
            } else {
                fill(&room.rect, '|');
            }
            if room.is_horizontal {
                {
                    let rect = Rectangle {
                        pos: Position {
                            x: region.rect.pos.x,
                            y: room.rect.pos.y,
                        },
                        size: Size {
                            x: room.rect.pos.x - region.rect.pos.x,
                            y: room.rect.size.y,
                        },
                    };
                    fill(&rect, '-');
                }
                {
                    let rect = Rectangle {
                        pos: Position {
                            x: room.rect.pos.x + room.rect.size.x,
                            y: room.rect.pos.y,
                        },
                        size: Size {
                            x: (region.rect.pos.x + region.rect.size.x) - (room.rect.pos.x + room.rect.size.x),
                            y: room.rect.size.y,
                        },
                    };
                    fill(&rect, '-');
                }
            } else {
                {
                    let rect = Rectangle {
                        pos: Position {
                            x: room.rect.pos.x,
                            y: region.rect.pos.y,
                        },
                        size: Size {
                            x: room.rect.size.x,
                            y: room.rect.pos.y - region.rect.pos.y,
                        },
                    };
                    fill(&rect, '|');
                }
                {
                    let rect = Rectangle {
                        pos: Position {
                            x: room.rect.pos.x,
                            y: room.rect.pos.y + room.rect.size.y,
                        },
                        size: Size {
                            x: room.rect.size.x,
                            y: (region.rect.pos.y + region.rect.size.y) - (room.rect.pos.y + room.rect.size.y),
                        },
                    };
                    fill(&rect, '|');
                }
            }
        }
    }

    chars_vec.iter().map(|chars| chars.iter().collect::<String>() + "\n").collect()
}

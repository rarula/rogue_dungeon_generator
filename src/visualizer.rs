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

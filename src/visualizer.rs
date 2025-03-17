use crate::generator::Args;
use crate::utils::*;

pub fn visualizer_3(args: &Args, regions: &Vec<Rectangle>) -> String {
    let mut str = String::new();

    for y in 0..args.area_size * args.area_count_y {
        for x in 0..args.area_size * args.area_count_x {
            let mut found = false;

            for region in regions {
                let point = Rectangle {
                    pos: Position { x, y },
                    size: Size { x: 1, y: 1 },
                };

                if region.intersects(&point) {
                    str.push(' ');
                    found = true;
                }
            }

            if !found {
                str.push('#');
            }
        }

        str.push('\n');
    }

    str
}

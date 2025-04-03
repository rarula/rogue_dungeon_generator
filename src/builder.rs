use crate::error::GenerationError;
use crate::generator::Args;
use crate::utils::*;
use rand::Rng;
use rand::seq::IndexedRandom;
use std::rc::Rc;

pub fn create_field(args: &Args) -> Vec2<Rectangle> {
    let mut vec: Vec2<Rectangle> = Vec::new();

    for y in 0..args.area_count_y {
        let mut vec2: Vec<Rectangle> = Vec::new();

        for x in 0..args.area_count_x {
            let pos = Position {
                x: args.area_size * x as i32,
                y: args.area_size * y as i32,
            };
            let size = Size {
                x: args.area_size,
                y: args.area_size,
            };
            let rect = Rectangle {
                pos,
                size,
            };

            vec2.push(rect);
        }

        vec.push(vec2);
    }

    vec
}

pub fn create_paths(args: &mut Args, field: Vec2<Rectangle>) -> Result<Vec2<DividedArea>, GenerationError> {
    let mut vec: Vec2<DividedArea> = Vec::new();
    let mut is_horizontal_source = args.rng.random_bool(0.5);

    for row in field {
        let mut vec2: Vec<DividedArea> = Vec::new();
        let mut is_horizontal = is_horizontal_source;

        let mut create_path = |rect: Rectangle, is_horizontal: bool| -> Result<DividedArea, GenerationError> {
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
                        positioned_nodes: Vec::new(),
                        horizontal_edges: Vec::new(),
                        vertical_edges: Vec::new(),
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
                        positioned_nodes: Vec::new(),
                        horizontal_edges: Vec::new(),
                        vertical_edges: Vec::new(),
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

pub fn extend_paths(args: &Args, field: &mut Vec2<DividedArea>) {
    for y in 0..args.area_count_y {
        let (y_before, y_middle) = field.split_at_mut(y);
        let (y_current, y_after) = y_middle.split_at_mut(1);
        let y_current = &mut y_current[0];

        for x in 0..args.area_count_x {
            let (x_before, x_middle) = y_current.split_at_mut(x);
            let (x_current, x_after) = x_middle.split_at_mut(1);
            let x_current = &mut x_current[0];

            if x_current.path.is_horizontal {
                // Extend the path to the left
                if 0 < x {
                    if let Some(left) = x_before.last_mut() {
                        let rect = {
                            let pos = left.path.rect.pos.x + left.path.rect.size.x;
                            let size = left.rect.pos.x + left.rect.size.x - pos;
                            Rectangle {
                                pos: Position {
                                    x: pos,
                                    y: x_current.path.rect.pos.y,
                                },
                                size: Size {
                                    x: size,
                                    y: args.path_size,
                                },
                            }
                        };
                        let path = Path {
                            rect,
                            is_horizontal: x_current.path.is_horizontal,
                        };
                        left.sub_paths.push(path);
                    }
                }
                // Extend the path to the right
                if x < args.area_count_x - 1 {
                    if let Some(right) = x_after.first_mut() {
                        let rect = {
                            let pos = right.rect.pos.x;
                            let size = right.path.rect.pos.x - pos;
                            Rectangle {
                                pos: Position {
                                    x: pos,
                                    y: x_current.path.rect.pos.y,
                                },
                                size: Size {
                                    x: size,
                                    y: args.path_size,
                                },
                            }
                        };
                        let path = Path {
                            rect,
                            is_horizontal: x_current.path.is_horizontal,
                        };
                        right.sub_paths.push(path);
                    }
                }
            } else {
                // Extend the path to the top
                if 0 < y {
                    if let Some(top) = y_before.last_mut() {
                        let top = &mut top[x];
                        let rect = {
                            let pos = top.path.rect.pos.y + top.path.rect.size.y;
                            let size = top.rect.pos.y + top.rect.size.y - pos;
                            Rectangle {
                                pos: Position {
                                    x: x_current.path.rect.pos.x,
                                    y: pos,
                                },
                                size: Size {
                                    x: args.path_size,
                                    y: size,
                                },
                            }
                        };
                        let path = Path {
                            rect,
                            is_horizontal: x_current.path.is_horizontal,
                        };
                        top.sub_paths.push(path);
                    }
                }
                // Extend the path to the bottom
                if y < args.area_count_y - 1 {
                    if let Some(bottom) = y_after.first_mut() {
                        let bottom = &mut bottom[x];
                        let rect = {
                            let pos = bottom.rect.pos.y;
                            let size = bottom.path.rect.pos.y - pos;
                            Rectangle {
                                pos: Position {
                                    x: x_current.path.rect.pos.x,
                                    y: pos,
                                },
                                size: Size {
                                    x: args.path_size,
                                    y: size,
                                },
                            }
                        };
                        let path = Path {
                            rect,
                            is_horizontal: x_current.path.is_horizontal,
                        };
                        bottom.sub_paths.push(path);
                    }
                }
            }
        }
    }
}

pub fn create_nodes(args: &Args, field: &mut Vec2<DividedArea>) {
    // TODO: split_at_mutを使用しない形にリファクタリングしてもよい（未使用の変数があるため）
    for y in 0..args.area_count_y {
        let (_y_before, y_middle) = field.split_at_mut(y);
        let (y_current, _y_after) = y_middle.split_at_mut(1);
        let y_current = &mut y_current[0];

        for x in 0..args.area_count_x {
            let (_x_before, x_middle) = y_current.split_at_mut(x);
            let (x_current, _x_after) = x_middle.split_at_mut(1);
            let x_current = &mut x_current[0];

            // Create border nodes in the area outside the field
            {
                if x_current.path.is_horizontal {
                    // Create a border node on the left
                    if x == 0 {
                        let rect = Rectangle {
                            pos: Position {
                                x: x_current.path.rect.pos.x - 1,
                                y: x_current.path.rect.pos.y,
                            },
                            size: Size {
                                x: 1,
                                y: x_current.path.rect.size.y,
                            },
                        };
                        let node = Node {
                            rect,
                        };
                        let node = PositionedNode {
                            loc: Location::Border,
                            node: Rc::new(node),
                        };
                        x_current.positioned_nodes.push(node);
                    }
                    // Create a border node on the right
                    if x == args.area_count_x - 1 {
                        let rect = Rectangle {
                            pos: Position {
                                x: x_current.path.rect.pos.x + x_current.path.rect.size.x,
                                y: x_current.path.rect.pos.y,
                            },
                            size: Size {
                                x: 1,
                                y: x_current.path.rect.size.y,
                            },
                        };
                        let node = Node {
                            rect,
                        };
                        let node = PositionedNode {
                            loc: Location::Border,
                            node: Rc::new(node),
                        };
                        x_current.positioned_nodes.push(node);
                    }
                } else {
                    // Create a border node on the top
                    if y == 0 {
                        let rect = Rectangle {
                            pos: Position {
                                x: x_current.path.rect.pos.x,
                                y: x_current.path.rect.pos.y - 1,
                            },
                            size: Size {
                                x: x_current.path.rect.size.x,
                                y: 1,
                            },
                        };
                        let node = Node {
                            rect,
                        };
                        let node = PositionedNode {
                            loc: Location::Border,
                            node: Rc::new(node),
                        };
                        x_current.positioned_nodes.push(node);
                    }
                    // Create a border node on the bottom
                    if y == args.area_count_y - 1 {
                        let rect = Rectangle {
                            pos: Position {
                                x: x_current.path.rect.pos.x,
                                y: x_current.path.rect.pos.y + x_current.path.rect.size.y,
                            },
                            size: Size {
                                x: x_current.path.rect.size.x,
                                y: 1,
                            },
                        };
                        let node = Node {
                            rect,
                        };
                        let node = PositionedNode {
                            loc: Location::Border,
                            node: Rc::new(node),
                        };
                        x_current.positioned_nodes.push(node);
                    }
                }
            }

            // Create a node in this area
            {
                if x_current.path.is_horizontal {
                    let (top, bottom) = get_sub_paths_x(&x_current.path, &x_current.sub_paths);

                    if let Some(top) = top {
                        let rect = Rectangle {
                            pos: Position {
                                x: top.rect.pos.x,
                                y: x_current.path.rect.pos.y,
                            },
                            size: Size {
                                x: args.path_size,
                                y: args.path_size,
                            },
                        };
                        let node = Node {
                            rect,
                        };
                        let node = PositionedNode {
                            loc: Location::Top,
                            node: Rc::new(node),
                        };
                        x_current.positioned_nodes.push(node);
                    }
                    if let Some(bottom) = bottom {
                        let rect = Rectangle {
                            pos: Position {
                                x: bottom.rect.pos.x,
                                y: x_current.path.rect.pos.y,
                            },
                            size: Size {
                                x: args.path_size,
                                y: args.path_size,
                            },
                        };
                        match get_matched_node(&mut x_current.positioned_nodes, &rect) {
                            Some(node) => {
                                node.loc = Location::TopBottom;
                            }
                            None => {
                                let node = Node {
                                    rect,
                                };
                                let node = PositionedNode {
                                    loc: Location::Bottom,
                                    node: Rc::new(node),
                                };
                                x_current.positioned_nodes.push(node);
                            }
                        }
                    }
                } else {
                    let (left, right) = get_sub_paths_y(&x_current.path, &x_current.sub_paths);

                    if let Some(left) = left {
                        let rect = Rectangle {
                            pos: Position {
                                x: x_current.path.rect.pos.x,
                                y: left.rect.pos.y,
                            },
                            size: Size {
                                x: args.path_size,
                                y: args.path_size,
                            },
                        };
                        let node = Node {
                            rect,
                        };
                        let node = PositionedNode {
                            loc: Location::Left,
                            node: Rc::new(node),
                        };
                        x_current.positioned_nodes.push(node);
                    }
                    if let Some(right) = right {
                        let rect = Rectangle {
                            pos: Position {
                                x: x_current.path.rect.pos.x,
                                y: right.rect.pos.y,
                            },
                            size: Size {
                                x: args.path_size,
                                y: args.path_size,
                            },
                        };
                        match get_matched_node(&mut x_current.positioned_nodes, &rect) {
                            Some(node) => {
                                node.loc = Location::LeftRight;
                            }
                            None => {
                                let node = Node {
                                    rect,
                                };
                                let node = PositionedNode {
                                    loc: Location::Right,
                                    node: Rc::new(node),
                                };
                                x_current.positioned_nodes.push(node);
                            }
                        }
                    }
                }

                fn get_sub_paths_x<'a>(path: &'a Path, sub_paths: &'a Vec<Path>) -> (Option<&'a Path>, Option<&'a Path>) {
                    let mut top: Option<&Path> = None;
                    let mut bottom: Option<&Path> = None;

                    for sub_path in sub_paths {
                        if sub_path.rect.pos.y < path.rect.pos.y {
                            top = Some(sub_path);
                        }
                        if path.rect.pos.y < sub_path.rect.pos.y {
                            bottom = Some(sub_path);
                        }
                    }
                    (top, bottom)
                }
                fn get_sub_paths_y<'a>(path: &'a Path, sub_paths: &'a Vec<Path>) -> (Option<&'a Path>, Option<&'a Path>) {
                    let mut left: Option<&Path> = None;
                    let mut right: Option<&Path> = None;

                    for sub_path in sub_paths {
                        if sub_path.rect.pos.x < path.rect.pos.x {
                            left = Some(sub_path);
                        }
                        if path.rect.pos.x < sub_path.rect.pos.x {
                            right = Some(sub_path);
                        }
                    }
                    (left, right)
                }
                fn get_matched_node<'a>(positioned_node: &'a mut Vec<PositionedNode>, rect: &Rectangle) -> Option<&'a mut PositionedNode> {
                    for node in positioned_node {
                        if &node.node.rect == rect {
                            return Some(node);
                        }
                    }
                    None
                }
            }
        }
    }
}

pub fn create_edges(args: &Args, field: &mut Vec2<DividedArea>) {
    for y in 0..args.area_count_y {
        let (y_before, y_middle) = field.split_at_mut(y);
        let (y_current, y_after) = y_middle.split_at_mut(1);
        let y_current = &mut y_current[0];

        for x in 0..args.area_count_x {
            let (x_before, x_middle) = y_current.split_at_mut(x);
            let (x_current, x_after) = x_middle.split_at_mut(1);
            let x_current = &mut x_current[0];

            if 0 < x_current.positioned_nodes.len() {
                // Create edges in areas outside of the field
                {
                    if x_current.path.is_horizontal {
                        // Create an edge on the left
                        if x == 0 {
                            if let Some(b) = nearest_x_without_border(&x_current.positioned_nodes, true) {
                                let a = nearest_x(&x_current.positioned_nodes, true);

                                x_current.horizontal_edges.push(Rc::new(Edge {
                                    a: Rc::clone(&a.node),
                                    b: Rc::clone(&b.node),
                                }));
                            }
                        }
                        // Create an edge on the right
                        if x == args.area_count_x - 1 {
                            if let Some(b) = nearest_x_without_border(&x_current.positioned_nodes, false) {
                                let a = nearest_x(&x_current.positioned_nodes, false);

                                x_current.horizontal_edges.push(Rc::new(Edge {
                                    a: Rc::clone(&a.node),
                                    b: Rc::clone(&b.node),
                                }));
                            }
                        }
                    } else {
                        // Create an edge on the top
                        if y == 0 {
                            if let Some(b) = nearest_y_without_border(&x_current.positioned_nodes, true) {
                                let a = nearest_y(&x_current.positioned_nodes, true);

                                x_current.vertical_edges.push(Rc::new(Edge {
                                    a: Rc::clone(&a.node),
                                    b: Rc::clone(&b.node),
                                }));
                            }
                        }
                        // Create an edge on the bottom
                        if y == args.area_count_y - 1 {
                            if let Some(b) = nearest_y_without_border(&x_current.positioned_nodes, false) {
                                let a = nearest_y(&x_current.positioned_nodes, false);

                                x_current.vertical_edges.push(Rc::new(Edge {
                                    a: Rc::clone(&a.node),
                                    b: Rc::clone(&b.node),
                                }));
                            }
                        }
                    }
                }

                // Create the edge in the center
                {
                    if x_current.path.is_horizontal {
                        if let Some(a) = nearest_x_without_border(&x_current.positioned_nodes, true) {
                            if let Some(b) = nearest_x_without_border(&x_current.positioned_nodes, false) {
                                if a.loc != b.loc {
                                    x_current.horizontal_edges.push(Rc::new(Edge {
                                        a: Rc::clone(&a.node),
                                        b: Rc::clone(&b.node),
                                    }));
                                }
                            }
                        }
                    } else {
                        if let Some(a) = nearest_y_without_border(&x_current.positioned_nodes, true) {
                            if let Some(b) = nearest_y_without_border(&x_current.positioned_nodes, false) {
                                if a.loc != b.loc {
                                    x_current.vertical_edges.push(Rc::new(Edge {
                                        a: Rc::clone(&a.node),
                                        b: Rc::clone(&b.node),
                                    }));
                                }
                            }
                        }
                    }
                }

                // Create edges in all areas of the field
                {
                    if x_current.path.is_horizontal {
                        // Create the edge to the left
                        if 0 < x {
                            if let Some(left) = x_before.last_mut() {
                                for a in &left.positioned_nodes {
                                    if let Location::Right | Location::LeftRight = a.loc {
                                        let b = nearest_x(&x_current.positioned_nodes, true);

                                        x_current.horizontal_edges.push(Rc::new(Edge {
                                            a: Rc::clone(&a.node),
                                            b: Rc::clone(&b.node),
                                        }));
                                        left.horizontal_edges.push(Rc::new(Edge {
                                            a: Rc::clone(&a.node),
                                            b: Rc::clone(&b.node),
                                        }));
                                    }
                                }
                            }
                        }
                        // Create the edge to the right
                        if x < args.area_count_x - 1 {
                            if let Some(right) = x_after.first_mut() {
                                for a in &right.positioned_nodes {
                                    if let Location::Left | Location::LeftRight = a.loc {
                                        let b = nearest_x(&x_current.positioned_nodes, false);

                                        x_current.horizontal_edges.push(Rc::new(Edge {
                                            a: Rc::clone(&a.node),
                                            b: Rc::clone(&b.node),
                                        }));
                                        right.horizontal_edges.push(Rc::new(Edge {
                                            a: Rc::clone(&a.node),
                                            b: Rc::clone(&b.node),
                                        }));
                                    }
                                }
                            }
                        }
                    } else {
                        // Create the edge to the top
                        if 0 < y {
                            if let Some(top) = y_before.last_mut() {
                                let top = &mut top[x];
                                for a in &top.positioned_nodes {
                                    if let Location::Bottom | Location::TopBottom = a.loc {
                                        let b = nearest_y(&x_current.positioned_nodes, true);

                                        x_current.vertical_edges.push(Rc::new(Edge {
                                            a: Rc::clone(&a.node),
                                            b: Rc::clone(&b.node),
                                        }));
                                        top.vertical_edges.push(Rc::new(Edge {
                                            a: Rc::clone(&a.node),
                                            b: Rc::clone(&b.node),
                                        }));
                                    }
                                }
                            }
                        }
                        // Create the edge to the bottom
                        if y < args.area_count_y - 1 {
                            if let Some(bottom) = y_after.first_mut() {
                                let bottom = &mut bottom[x];
                                for a in &bottom.positioned_nodes {
                                    if let Location::Top | Location::TopBottom = a.loc {
                                        let b = nearest_y(&x_current.positioned_nodes, false);

                                        x_current.vertical_edges.push(Rc::new(Edge {
                                            a: Rc::clone(&a.node),
                                            b: Rc::clone(&b.node),
                                        }));
                                        bottom.vertical_edges.push(Rc::new(Edge {
                                            a: Rc::clone(&a.node),
                                            b: Rc::clone(&b.node),
                                        }));
                                    }
                                }
                            }
                        }
                    }
                }

                fn nearest_x(nodes: &Vec<PositionedNode>, to_left: bool) -> &PositionedNode {
                    let mut res = &nodes[0];
                    if 1 < nodes.len() {
                        for node in &nodes[1..] {
                            let should_update_left = to_left && node.node.rect.pos.x < res.node.rect.pos.x;
                            let should_update_right = !to_left && res.node.rect.pos.x < node.node.rect.pos.x;

                            if should_update_left || should_update_right {
                                res = node;
                            }
                        }
                    }
                    res
                }
                fn nearest_y(nodes: &Vec<PositionedNode>, to_top: bool) -> &PositionedNode {
                    let mut res = &nodes[0];
                    if 1 < nodes.len() {
                        for node in &nodes[1..] {
                            let should_update_top = to_top && node.node.rect.pos.y < res.node.rect.pos.y;
                            let should_update_bottom = !to_top && res.node.rect.pos.y < node.node.rect.pos.y;

                            if should_update_top || should_update_bottom {
                                res = node;
                            }
                        }
                    }
                    res
                }
                fn nearest_x_without_border(nodes: &Vec<PositionedNode>, to_left: bool) -> Option<&PositionedNode> {
                    let mut res: Option<&PositionedNode> = None;
                    for node in nodes {
                        if !node.loc.is_border() {
                            if let Some(any_node) = res {
                                let should_update_left = to_left && node.node.rect.pos.x < any_node.node.rect.pos.x;
                                let should_update_right = !to_left && any_node.node.rect.pos.x < node.node.rect.pos.x;

                                if should_update_left || should_update_right {
                                    res = Some(node);
                                }
                            } else {
                                res = Some(node);
                            }
                        }
                    }
                    res
                }
                fn nearest_y_without_border(nodes: &Vec<PositionedNode>, to_top: bool) -> Option<&PositionedNode> {
                    let mut res: Option<&PositionedNode> = None;
                    for node in nodes {
                        if !node.loc.is_border() {
                            if let Some(any_node) = res {
                                let should_update_top = to_top && node.node.rect.pos.y < any_node.node.rect.pos.y;
                                let should_update_bottom = !to_top && any_node.node.rect.pos.y < node.node.rect.pos.y;

                                if should_update_top || should_update_bottom {
                                    res = Some(node);
                                }
                            } else {
                                res = Some(node);
                            }
                        }
                    }
                    res
                }
            } else {
                if x_current.path.is_horizontal {
                    if 0 < x && x < args.area_count_x - 1 {
                        if let (Some(left), Some(right)) = (x_before.last_mut(), x_after.first_mut()) {
                            let a = 'a: {
                                for node in &left.positioned_nodes {
                                    if let Location::Right | Location::LeftRight = node.loc {
                                        break 'a Some(node);
                                    }
                                }
                                None
                            };
                            let b = 'b: {
                                for node in &right.positioned_nodes {
                                    if let Location::Left | Location::LeftRight = node.loc {
                                        break 'b Some(node);
                                    }
                                }
                                None
                            };
                            if let (Some(a), Some(b)) = (a, b) {
                                x_current.horizontal_edges.push(Rc::new(Edge {
                                    a: Rc::clone(&a.node),
                                    b: Rc::clone(&b.node),
                                }));
                                left.horizontal_edges.push(Rc::new(Edge {
                                    a: Rc::clone(&a.node),
                                    b: Rc::clone(&b.node),
                                }));
                                right.horizontal_edges.push(Rc::new(Edge {
                                    a: Rc::clone(&a.node),
                                    b: Rc::clone(&b.node),
                                }));
                            }
                        }
                    }
                } else {
                    if 0 < y && y < args.area_count_y - 1 {
                        if let (Some(top), Some(bottom)) = (y_before.last_mut(), y_after.first_mut()) {
                            let top = &mut top[x];
                            let bottom = &mut bottom[x];

                            let a = 'a: {
                                for node in &top.positioned_nodes {
                                    if let Location::Bottom | Location::TopBottom = node.loc {
                                        break 'a Some(node);
                                    }
                                }
                                None
                            };
                            let b = 'b: {
                                for node in &bottom.positioned_nodes {
                                    if let Location::Top | Location::TopBottom = node.loc {
                                        break 'b Some(node);
                                    }
                                }
                                None
                            };
                            if let (Some(a), Some(b)) = (a, b) {
                                x_current.vertical_edges.push(Rc::new(Edge {
                                    a: Rc::clone(&a.node),
                                    b: Rc::clone(&b.node),
                                }));
                                top.vertical_edges.push(Rc::new(Edge {
                                    a: Rc::clone(&a.node),
                                    b: Rc::clone(&b.node),
                                }));
                                bottom.vertical_edges.push(Rc::new(Edge {
                                    a: Rc::clone(&a.node),
                                    b: Rc::clone(&b.node),
                                }));
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn combine_regions(args: &Args, field: &Vec2<DividedArea>) -> Vec<CombinedRegion> {
    let mut vec: Vec<CombinedRegion> = Vec::new();

    for y in 0..args.area_count_y {
        for x in 0..args.area_count_x {
            if y == 0 {
                match field[y][x].region(Quadrant::RightTop) {
                    Region::Top(sr) => {
                        if x == 0 {
                            let side_edges_x = get_side_edges_x(&field[y][x], &sr);
                            if x < args.area_count_x - 1 {
                                if let Region::LeftTop(r) = field[y][x + 1].region(Quadrant::LeftTop) {
                                    let side_edges_y = get_side_edges_y(&field[y][x + 1], &r);
                                    let region = CombinedRegion {
                                        rect: combine_x(sr, r),
                                        side_edges_x,
                                        side_edges_y,
                                    };
                                    vec.push(region);
                                }
                            } else {
                                let region = CombinedRegion {
                                    rect: sr,
                                    side_edges_x,
                                    side_edges_y: Vec::new(),
                                };
                                vec.push(region);
                            }
                        }
                    }
                    Region::RightTop(sr) => {
                        if x < args.area_count_x - 1 {
                            if let Region::Top(r) = field[y][x + 1].region(Quadrant::LeftTop) {
                                let side_edges_y = get_side_edges_y(&field[y][x], &sr);
                                let side_edges_x = get_side_edges_x(&field[y][x + 1], &r);
                                if x < args.area_count_x - 2 {
                                    let sr = combine_x(sr, r);
                                    if let Region::LeftTop(r) = field[y][x + 2].region(Quadrant::LeftTop) {
                                        let edges_y = get_side_edges_y(&field[y][x + 2], &r);
                                        let region = CombinedRegion {
                                            rect: combine_x(sr, r),
                                            side_edges_x,
                                            side_edges_y: [side_edges_y, edges_y].concat(),
                                        };
                                        vec.push(region);
                                    }
                                } else {
                                    let region = CombinedRegion {
                                        rect: combine_x(sr, r),
                                        side_edges_x,
                                        side_edges_y,
                                    };
                                    vec.push(region);
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
            if y == args.area_count_y - 1 {
                match field[y][x].region(Quadrant::RightBottom) {
                    Region::Bottom(sr) => {
                        if x == 0 {
                            let side_edges_x = get_side_edges_x(&field[y][x], &sr);
                            if x < args.area_count_x - 1 {
                                if let Region::LeftBottom(r) = field[y][x + 1].region(Quadrant::LeftBottom) {
                                    let side_edges_y = get_side_edges_y(&field[y][x + 1], &r);
                                    let region = CombinedRegion {
                                        rect: combine_x(sr, r),
                                        side_edges_x,
                                        side_edges_y,
                                    };
                                    vec.push(region);
                                }
                            } else {
                                let region = CombinedRegion {
                                    rect: sr,
                                    side_edges_x,
                                    side_edges_y: Vec::new(),
                                };
                                vec.push(region);
                            }
                        }
                    }
                    Region::RightBottom(sr) => {
                        if x < args.area_count_x - 1 {
                            if let Region::Bottom(r) = field[y][x + 1].region(Quadrant::LeftBottom) {
                                let side_edges_y = get_side_edges_y(&field[y][x], &sr);
                                let side_edges_x = get_side_edges_x(&field[y][x + 1], &r);
                                if x < args.area_count_x - 2 {
                                    let sr = combine_x(sr, r);
                                    if let Region::LeftBottom(r) = field[y][x + 2].region(Quadrant::LeftBottom) {
                                        let edges_y = get_side_edges_y(&field[y][x + 2], &r);
                                        let region = CombinedRegion {
                                            rect: combine_x(sr, r),
                                            side_edges_x,
                                            side_edges_y: [side_edges_y, edges_y].concat(),
                                        };
                                        vec.push(region);
                                    }
                                } else {
                                    let region = CombinedRegion {
                                        rect: combine_x(sr, r),
                                        side_edges_x,
                                        side_edges_y,
                                    };
                                    vec.push(region);
                                }
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
                            let side_edges_y = get_side_edges_y(&field[y][x], &sr);
                            if y < args.area_count_y - 1 {
                                if let Region::LeftTop(r) = field[y + 1][x].region(Quadrant::LeftTop) {
                                    let side_edges_x = get_side_edges_x(&field[y + 1][x], &r);
                                    let region = CombinedRegion {
                                        rect: combine_y(sr, r),
                                        side_edges_x,
                                        side_edges_y,
                                    };
                                    vec.push(region);
                                }
                            } else {
                                let region = CombinedRegion {
                                    rect: sr,
                                    side_edges_x: Vec::new(),
                                    side_edges_y,
                                };
                                vec.push(region);
                            }
                        }
                    }
                    Region::LeftBottom(sr) => {
                        if y < args.area_count_y - 1 {
                            if let Region::Left(r) = field[y + 1][x].region(Quadrant::LeftTop) {
                                let side_edges_x = get_side_edges_x(&field[y][x], &sr);
                                let side_edges_y = get_side_edges_y(&field[y + 1][x], &r);
                                if y < args.area_count_y - 2 {
                                    let sr = combine_y(sr, r);
                                    if let Region::LeftTop(r) = field[y + 2][x].region(Quadrant::LeftTop) {
                                        let edges_x = get_side_edges_x(&field[y + 2][x], &r);
                                        let region = CombinedRegion {
                                            rect: combine_y(sr, r),
                                            side_edges_x: [side_edges_x, edges_x].concat(),
                                            side_edges_y,
                                        };
                                        vec.push(region);
                                    }
                                } else {
                                    let region = CombinedRegion {
                                        rect: combine_y(sr, r),
                                        side_edges_x,
                                        side_edges_y,
                                    };
                                    vec.push(region);
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
            if x == args.area_count_x - 1 {
                match field[y][x].region(Quadrant::RightBottom) {
                    Region::Right(sr) => {
                        if y == 0 {
                            let side_edges_y = get_side_edges_y(&field[y][x], &sr);
                            if y < args.area_count_y - 1 {
                                if let Region::RightTop(r) = field[y + 1][x].region(Quadrant::RightTop) {
                                    let side_edges_x = get_side_edges_x(&field[y + 1][x], &r);
                                    let region = CombinedRegion {
                                        rect: combine_y(sr, r),
                                        side_edges_x,
                                        side_edges_y,
                                    };
                                    vec.push(region);
                                }
                            } else {
                                let region = CombinedRegion {
                                    rect: sr,
                                    side_edges_x: Vec::new(),
                                    side_edges_y,
                                };
                                vec.push(region);
                            }
                        }
                    }
                    Region::RightBottom(sr) => {
                        if y < args.area_count_y - 1 {
                            if let Region::Right(r) = field[y + 1][x].region(Quadrant::RightTop) {
                                let side_edges_x = get_side_edges_x(&field[y][x], &sr);
                                let side_edges_y = get_side_edges_y(&field[y + 1][x], &r);
                                if y < args.area_count_y - 2 {
                                    let sr = combine_y(sr, r);
                                    if let Region::RightTop(r) = field[y + 2][x].region(Quadrant::RightTop) {
                                        let edges_x = get_side_edges_x(&field[y + 2][x], &r);
                                        let region = CombinedRegion {
                                            rect: combine_y(sr, r),
                                            side_edges_x: [side_edges_x, edges_x].concat(),
                                            side_edges_y,
                                        };
                                        vec.push(region);
                                    }
                                } else {
                                    let region = CombinedRegion {
                                        rect: combine_y(sr, r),
                                        side_edges_x,
                                        side_edges_y,
                                    };
                                    vec.push(region);
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
            if y < args.area_count_y - 1 && x < args.area_count_x - 1 {
                if let Region::RightBottom(sr) = field[y][x].region(Quadrant::RightBottom) {
                    if let Region::LeftBottom(r0) = field[y][x + 1].region(Quadrant::LeftBottom) {
                        if let Region::RightTop(r1) = field[y + 1][x].region(Quadrant::RightTop) {
                            if let Region::LeftTop(r2) = field[y + 1][x + 1].region(Quadrant::LeftTop) {
                                if field[y][x].path.is_horizontal {
                                    let edges_x0 = get_side_edges_x(&field[y][x], &sr);
                                    let edges_y0 = get_side_edges_y(&field[y][x + 1], &r0);
                                    let edges_y1 = get_side_edges_y(&field[y + 1][x], &r1);
                                    let edges_x1 = get_side_edges_x(&field[y + 1][x + 1], &r2);

                                    let x_rect = combine_x(sr, r0);
                                    let xy_rect = combine_y(x_rect, r1);

                                    let region = CombinedRegion {
                                        rect: xy_rect,
                                        side_edges_x: [edges_x0, edges_x1].concat(),
                                        side_edges_y: [edges_y0, edges_y1].concat(),
                                    };
                                    vec.push(region);
                                } else {
                                    let edges_y0 = get_side_edges_y(&field[y][x], &sr);
                                    let edges_x0 = get_side_edges_x(&field[y][x + 1], &r0);
                                    let edges_x1 = get_side_edges_x(&field[y + 1][x], &r1);
                                    let edges_y1 = get_side_edges_y(&field[y + 1][x + 1], &r2);

                                    let x_rect = combine_x(sr, r0);
                                    let xy_rect = combine_y(x_rect, r1);

                                    let region = CombinedRegion {
                                        rect: xy_rect,
                                        side_edges_x: [edges_x0, edges_x1].concat(),
                                        side_edges_y: [edges_y0, edges_y1].concat(),
                                    };
                                    vec.push(region);
                                }
                            }
                        }
                    }
                }
            }

            fn get_side_edges_x(area: &DividedArea, rect: &Rectangle) -> Vec<Rc<Edge>> {
                area.horizontal_edges
                    .iter()
                    .filter(|&edge| rect.intersects_x(&edge.to_rect()))
                    .map(|edge| Rc::clone(edge))
                    .collect()
            }
            fn get_side_edges_y(area: &DividedArea, rect: &Rectangle) -> Vec<Rc<Edge>> {
                area.vertical_edges
                    .iter()
                    .filter(|&edge| rect.intersects_y(&edge.to_rect()))
                    .map(|edge| Rc::clone(edge))
                    .collect()
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

pub fn create_subareas(args: &mut Args, regions: Vec<CombinedRegion>) -> Result<Vec<Subarea>, GenerationError> {
    let mut candidates: Vec<CombinedRegion> = Vec::new();

    for region in regions {
        if 0 < region.rect.size.x - 2 && 0 < region.rect.size.y - 2 {
            candidates.push(region);
        }
    }
    if args.room_count <= candidates.len() {
        let mut subareas: Vec<Subarea> = Vec::new();

        for candidate in candidates.choose_multiple(&mut args.rng, args.room_count) {
            let size = Size {
                x: args.rng.random_range(candidate.rect.size.x / 2..candidate.rect.size.x - 1),
                y: args.rng.random_range(candidate.rect.size.y / 2..candidate.rect.size.y - 1),
            };
            let pos = Position {
                x: candidate.rect.pos.x + 1 + args.rng.random_range(0..candidate.rect.size.x - 1 - size.x),
                y: candidate.rect.pos.y + 1 + args.rng.random_range(0..candidate.rect.size.y - 1 - size.y),
            };
            let room = Room {
                rect: Rectangle {
                    pos,
                    size,
                },
                is_horizontal: args.rng.random_bool(0.5),
            };
            let subarea = Subarea {
                rect: candidate.rect.clone(),
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

use crate::builder;
use crate::error::GenerationError;
use crate::utils::*;
use rand::prelude::*;

pub struct Args {
    pub area_size: i32,
    pub area_count_x: usize,
    pub area_count_y: usize,
    pub path_size: i32,
    pub room_count: usize,
    pub rng: StdRng,
}

pub fn generate_0(args: &Args) -> Vec2<Rectangle> {
    let field = builder::create_field(args);
    field
}

pub fn generate_1(args: &mut Args) -> Result<Vec2<DividedArea>, GenerationError> {
    let field = builder::create_field(args);
    let field = builder::create_paths(args, field);
    field
}

pub fn generate_2(args: &mut Args) -> Result<Vec2<DividedArea>, GenerationError> {
    let field = builder::create_field(args);
    let mut field = match builder::create_paths(args, field) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    builder::extend_paths(args, &mut field);
    builder::create_nodes(args, &mut field);
    builder::create_edges(args, &mut field);
    Ok(field)
}

pub fn generate_3(args: &mut Args) -> Result<Vec<CombinedRegion>, GenerationError> {
    let field = builder::create_field(args);
    let mut field = match builder::create_paths(args, field) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    builder::extend_paths(args, &mut field);
    builder::create_nodes(args, &mut field);
    builder::create_edges(args, &mut field);
    let regions = builder::combine_regions(args, &field);
    Ok(regions)
}

pub fn generate_4(args: &mut Args) -> Result<Vec<CombinedRegion>, GenerationError> {
    let field = builder::create_field(args);
    let mut field = match builder::create_paths(args, field) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    builder::extend_paths(args, &mut field);
    builder::create_nodes(args, &mut field);
    builder::create_edges(args, &mut field);
    let mut regions = builder::combine_regions(args, &field);
    match builder::create_rooms(args, &mut regions) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };
    Ok(regions)
}

pub fn generate_5(args: &mut Args) -> Result<(Vec2<DividedArea>, Vec<CombinedRegion>), GenerationError> {
    let field = builder::create_field(args);
    let mut field = match builder::create_paths(args, field) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    builder::extend_paths(args, &mut field);
    builder::create_nodes(args, &mut field);
    builder::create_edges(args, &mut field);
    let mut regions = builder::combine_regions(args, &field);
    match builder::create_rooms(args, &mut regions) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };
    builder::remove_edges(&regions);
    Ok((field, regions))
}

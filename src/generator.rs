use crate::builder;
use crate::error::GenerationError;
use crate::utils::*;
use rand::prelude::*;

pub struct Args {
    pub area_size: i32,
    pub area_count_x: i32,
    pub area_count_y: i32,
    pub path_size: i32,
    pub room_count: i32,
    pub rng: StdRng,
}

pub fn generate_0(args: &Args) -> Vec2<Rectangle> {
    builder::create_field(args)
}

pub fn generate_1(args: &mut Args) -> Result<Vec2<DividedArea>, GenerationError> {
    let field = builder::create_field(args);
    let field = builder::create_paths(args, field);

    field
}

pub fn generate_2(args: &mut Args) -> Result<Vec2<DividedArea>, GenerationError> {
    let field = builder::create_field(args);
    let field = match builder::create_paths(args, field) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let field = builder::extend_paths(args, field);

    Ok(field)
}

pub fn generate_3(args: &mut Args) -> Result<Vec<Rectangle>, GenerationError> {
    let field = builder::create_field(args);
    let field = match builder::create_paths(args, field) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let field = builder::extend_paths(args, field);
    let regions = builder::combine_regions(&field);

    Ok(regions)
}

pub fn generate_4(args: &mut Args) -> Result<(Vec<Rectangle>, Vec<Subarea>), GenerationError> {
    let field = builder::create_field(args);
    let field = match builder::create_paths(args, field) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let field = builder::extend_paths(args, field);
    let regions = builder::combine_regions(&field);
    let subareas = match builder::create_subareas(args, regions.clone()) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok((regions, subareas))
}

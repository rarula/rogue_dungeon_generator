pub mod builder;
pub mod error;
pub mod generator;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::generator;
    use rand::prelude::*;

    fn create_args() -> generator::Args {
        generator::Args {
            area_size: 4,
            area_count_x: 2,
            area_count_y: 2,
            path_size: 2,
            rng: StdRng::from_seed([1; 32]),
        }
    }

    mod test_all_steps {
        use super::create_args;
        use super::generator;

        // #[test]
        // fn step_0() {}

        #[test]
        fn step_1() {
            let mut args = create_args();
            let output = generator::generate_1(&mut args);

            assert!(output.is_ok());
        }

        #[test]
        fn step_2() {
            let mut args = create_args();
            let output = generator::generate_2(&mut args);

            assert!(output.is_ok());
        }

        #[test]
        fn step_3() {
            let mut args = create_args();
            let output = generator::generate_3(&mut args);

            assert!(output.is_ok());
        }
    }

    mod debug_all_steps {
        use super::create_args;
        use super::generator;

        #[test]
        fn gen_0() {
            let args = create_args();
            let output = generator::generate_0(&args);

            println!("GENERATED:\n{:#?}", output);
        }

        #[test]
        fn gen_1() {
            let mut args = create_args();
            let output = generator::generate_1(&mut args);

            match output {
                Ok(v) => println!("GENERATED:\n{:#?}", v),
                Err(e) => println!("ERROR:\n{}", e),
            }
        }

        #[test]
        fn gen_2() {
            let mut args = create_args();
            let output = generator::generate_2(&mut args);

            match output {
                Ok(v) => println!("GENERATED:\n{:#?}", v),
                Err(e) => println!("ERROR:\n{}", e),
            }
        }

        #[test]
        fn gen_3() {
            let mut args = create_args();
            let output = generator::generate_3(&mut args);

            match output {
                Ok(v) => println!("GENERATED:\n{:#?}", v),
                Err(e) => println!("ERROR:\n{}", e),
            }
        }
    }
}

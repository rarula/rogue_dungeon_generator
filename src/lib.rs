pub mod builder;
pub mod error;
pub mod generator;
pub mod utils;
pub mod visualizer;

#[cfg(test)]
mod tests {
    use super::generator;
    use super::visualizer;
    use rand::prelude::*;

    fn create_args() -> generator::Args {
        generator::Args {
            area_size: 15,
            area_count_x: 5,
            area_count_y: 5,
            path_size: 2,
            room_count: 10,
            rng: StdRng::from_seed([0; 32]),
        }
    }

    mod test_all_steps {
        use super::create_args;
        use super::generator;

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

        #[test]
        fn step_4() {
            let mut args = create_args();
            let output = generator::generate_4(&mut args);

            assert!(output.is_ok());
        }

        #[test]
        fn step_5() {
            let mut args = create_args();
            let output = generator::generate_5(&mut args);

            assert!(output.is_ok());
        }
    }

    mod debug_all_steps {
        use super::create_args;
        use super::generator;
        use super::visualizer;

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
                Ok(v) => {
                    println!("GENERATED:\n{:#?}", v);
                    println!("VISUALIZED:\n{}", visualizer::visualizer_3(&args, &v));
                }
                Err(e) => println!("ERROR:\n{}", e),
            }
        }

        #[test]
        fn gen_4() {
            let mut args = create_args();
            let output = generator::generate_4(&mut args);

            match output {
                Ok(v) => {
                    println!("GENERATED:\n{:#?}", v);
                    println!("VISUALIZED:\n{}", visualizer::visualizer_4(&args, &v))
                }
                Err(e) => println!("ERROR:\n{}", e),
            };
        }

        #[test]
        fn gen_5() {
            let mut args = create_args();
            let output = generator::generate_5(&mut args);

            match output {
                Ok((v0, v1)) => {
                    println!("GENERATED AREAS:\n{:#?}", v0);
                    println!("GENERATED REGIONS:\n{:#?}", v1);
                    println!("VISUALIZED:\n{}", visualizer::visualizer_5(&args, &v0, &v1))
                }
                Err(e) => println!("ERROR:\n{}", e),
            };
        }
    }
}

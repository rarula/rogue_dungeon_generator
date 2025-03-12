use std::fmt;

#[derive(Debug)]
pub enum GenerationError {
    GE0001,
}

impl GenerationError {
    pub fn error_code(&self) -> u32 {
        match self {
            Self::GE0001 => 1,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::GE0001 => "エリア内に通路を生成できる場所が存在しません。",
        }
    }

    pub fn cause(&self) -> &str {
        match self {
            Self::GE0001 => {
                "'area_size' の値に対して 'path_size' の値が大きすぎるか、'path_size' の値に対して 'area_size' の値が小さすぎます。"
            }
        }
    }
}

impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error_code: {},\ndescription: {},\ncause: {}",
            self.error_code(),
            self.description(),
            self.cause(),
        )
    }
}

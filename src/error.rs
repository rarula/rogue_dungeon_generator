use std::fmt;

#[derive(Debug)]
pub enum GenerationError {
    GE0001,
    GE0002(i32, usize),
    GE0003(i32, usize),
}

impl GenerationError {
    pub fn error_code(&self) -> u32 {
        match self {
            Self::GE0001 => 1,
            Self::GE0002(_, _) => 2,
            Self::GE0003(_, _) => 3,
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::GE0001 => String::from("エリア内に通路を生成できる場所が存在しません。"),
            Self::GE0002(expected, received) => format!(
                "{}個の部屋が生成できることを期待しましたが、実際に生成できるのは{}個までです。",
                expected, received
            ),
            Self::GE0003(expected, received) => format!(
                "{}個の部屋が生成できることを期待しましたが、実際に生成できるのは{}個までです。",
                expected, received
            ),
        }
    }

    pub fn cause(&self) -> String {
        match self {
            Self::GE0001 => String::from("'area_size' の値に対して 'path_size' の値が大きすぎるか、'path_size' の値に対して 'area_size' の値が小さすぎます。"),
            Self::GE0002(_, _) => String::from("生成する部屋の数がエリアの総数に対して多すぎるため、期待された数の部屋を生成することができません。"),
            Self::GE0003(_, _) => {
                String::from("生成されたサブエリアのサイズが小さすぎるため、最小サイズの部屋ですら期待された数だけ生成することができません。")
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

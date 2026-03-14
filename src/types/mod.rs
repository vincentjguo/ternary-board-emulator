pub mod trit;
pub mod word;

pub use trit::Trit;
pub use word::Word;

pub const WORD_SIZE: usize = 9;
pub const MAX_VALUE: i32 = 3_i32.pow(WORD_SIZE as u32) / 2;
pub const MIN_VALUE: i32 = -MAX_VALUE;
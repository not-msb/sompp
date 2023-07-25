pub mod tools;
pub mod types;

pub type Res<T> = Result<T, Box<dyn std::error::Error>>;

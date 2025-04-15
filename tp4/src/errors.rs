#[derive(Debug)]
pub enum Error {
    EmptyFile,
    LargeFile,
    Overload,
    InvalidRoute,
}

#[derive(Clone, Copy)]
pub enum SearchType {
    Sequential,
    Concurrent,
    ChunkConcurrent,
}

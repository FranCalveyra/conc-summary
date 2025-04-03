use crate::server::start_server;

pub mod server;

fn main() { start_server() }
// Testing request with n = 500, c = 100
// With pi = 9_000_000, it lasts 7.5 secs avg.



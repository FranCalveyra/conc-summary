mod server;
// ThreadPool approach
fn main() {server::start_pool_server(thread_pool::thread_pool::get_system_thread_amount())}
// Testing request with n = 500, c = 50
// With pi = 9_000_000 and 8 threads, it lasts 3.0 secs avg.

// fn main() {start_pool_server(8)}
// Testing request with n = 500, c = 100
// With pi = 9_000_000 and 8 threads, it lasts 5.7 secs avg.

mod tests{
    #[test]
    fn assert_available_cores() {
        assert_eq!(thread_pool::thread_pool::get_system_thread_amount(), 8) // Depends on system
    }
}
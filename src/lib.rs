mod server;
mod thread_pool;

mod leibniz_adder;
mod errors;

#[cfg(test)]
mod tests{
    // TODO: develop more tests
    use crate::thread_pool::get_system_thread_amount;
    #[test]
    fn assert_available_cores() {
        assert_eq!(get_system_thread_amount(), 8) // Depends on system
    }
}

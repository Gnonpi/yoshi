// repetition of init_logger from test_utils to use in integration tests

pub fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

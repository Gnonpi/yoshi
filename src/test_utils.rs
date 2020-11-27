// in integration tests (tests/), we could use this as a module super easily,
// let's keep it simple

pub fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}
